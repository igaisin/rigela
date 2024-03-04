/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use crate::model::IbmeciVoiceParams;
use encoding_rs::GBK;
use log::{error, info};
use rigela_resources::clone_resource;
use rigela_utils::{
    call_proc,
    fs::{get_file_modified_duration, get_program_directory},
    SERVER_HOME_URI,
};
use std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    borrow::Cow,
    ffi::{c_char, CString},
    sync::OnceLock,
    thread,
    time::Duration,
};
use tokio::{
    sync::oneshot::{self, channel, Sender},
    time::sleep,
};
use win_wrap::{
    common::{free_library, get_proc_address, load_library, FARPROC, HMODULE, LPARAM, WPARAM},
    message::{message_loop, post_thread_message, register_window_message},
    threading::get_current_thread_id,
    wm,
};

macro_rules! eci {
    ($module:expr,new) => {
        call_proc!($module, eciNew, extern "system" fn() -> i32,)
    };
    ($module:expr,delete,$handle:expr) => {
        call_proc!($module, eciDelete, extern "system" fn(i32) -> i32, $handle)
    };
    ($module:expr,speaking,$handle:expr) => {
        call_proc!(
            $module,
            eciSpeaking,
            extern "system" fn(i32) -> bool,
            $handle
        )
    };
    ($module:expr,stop,$handle:expr) => {
        call_proc!($module, eciStop, extern "system" fn(i32) -> bool, $handle)
    };
    ($module:expr,register_callback,$handle:expr,$cb:expr,$data:expr) => {
        call_proc!(
            $module,
            eciRegisterCallback,
            extern "system" fn(i32, extern "system" fn(u32, u32, u32, u32) -> u32, u32),
            $handle,
            $cb,
            $data
        )
    };
    ($module:expr,set_output_buffer,$handle:expr,$samples:expr,$buffer:expr) => {
        call_proc!(
            $module,
            eciSetOutputBuffer,
            extern "system" fn(i32, u32, *mut u8),
            $handle,
            $samples,
            $buffer
        )
    };
    ($module:expr,add_text,$handle:expr,$text:expr) => {{
        let p = CString::new($text).unwrap();
        call_proc!(
            $module,
            eciAddText,
            extern "system" fn(i32, *const c_char) -> bool,
            $handle,
            p.as_ptr()
        )
    }};
    ($module:expr,speak_text,$text:expr) => {{
        let p = CString::new($text).unwrap();
        call_proc!(
            $module,
            eciSpeakText,
            extern "system" fn(*mut c_char),
            p.as_ptr()
        )
    }};
    ($module:expr,synthesize,$handle:expr) => {
        call_proc!($module, eciSynthesize, extern "system" fn(i32), $handle)
    };
    ($module:expr,synchronize,$handle:expr) => {
        call_proc!($module, eciSynchronize, extern "system" fn(i32), $handle)
    };
    ($module:expr,set_voice_param,$handle:expr,$voice:expr,$key:expr,$value:expr) => {
        call_proc!(
            $module,
            eciSetVoiceParam,
            extern "system" fn(i32, i32, u32, i32),
            $handle,
            $voice,
            $key,
            $value
        )
    };
    ($module:expr,get_voice_param,$handle:expr,$voice:expr,$key:expr) => {
        call_proc!(
            $module,
            eciGetVoiceParam,
            extern "system" fn(i32, i32, u32) -> i32,
            $handle,
            $voice,
            $key
        )
    };
    ($module:expr,copy_voice,$handle:expr,$copy_from:expr,$copy_to:expr) => {
        call_proc!(
            $module,
            eciCopyVoice,
            extern "system" fn(i32, u32, u32),
            $handle,
            $copy_from,
            $copy_to
        )
    };
}

#[allow(unused)]
const MSG_WAVEFORM_BUFFER: u32 = 0;
#[allow(unused)]
const MSG_PHONEME_BUFFER: u32 = 1;
#[allow(unused)]
const MSG_INDEX_REPLY: u32 = 2;
#[allow(unused)]
const MSG_PHONEME_INDEX_REPLY: u32 = 3;
#[allow(unused)]
const MSG_WORD_INDEX_REPLY: u32 = 4;
#[allow(unused)]
const RETURN_DATA_NOT_PROCESSED: u32 = 0;
#[allow(unused)]
const RETURN_DATA_PROCESSED: u32 = 1;
#[allow(unused)]
const RETURN_DATA_ABORT: u32 = 2;
#[allow(unused)]
const VP_GENDER: u32 = 0;
#[allow(unused)]
const VP_HEAD_SIZE: u32 = 1;
#[allow(unused)]
const VP_PITCH_BASELINE: u32 = 2;
#[allow(unused)]
const VP_PITCH_FLUCTUATION: u32 = 3;
#[allow(unused)]
const VP_ROUGHNESS: u32 = 4;
//noinspection SpellCheckingInspection
#[allow(unused)]
const VP_BREATHINESS: u32 = 5;
#[allow(unused)]
const VP_SPEED: u32 = 6;
#[allow(unused)]
const VP_VOLUME: u32 = 7;

//noinspection SpellCheckingInspection
static mut IBMECI: OnceLock<Ibmeci> = OnceLock::new();
static SYNTH_TASK: OnceLock<u32> = OnceLock::new();

extern "system" fn _callback_internal(
    #[allow(unused_variables)] h_eci: u32,
    msg: u32,
    param: u32,
    #[allow(unused_variables)] data: u32,
) -> u32 {
    if msg != MSG_WAVEFORM_BUFFER {
        return RETURN_DATA_PROCESSED;
    }
    unsafe {
        let eci = IBMECI.get_mut();
        if eci.is_none() {
            return RETURN_DATA_PROCESSED;
        }

        let eci = eci.unwrap();
        let mut vec = vec![];
        for i in 0..(param * 2) {
            vec.push(*eci.buffer_ptr.wrapping_add(i as usize));
        }
        eci.data.extend(vec);
    }
    RETURN_DATA_PROCESSED
}

//noinspection SpellCheckingInspection
#[derive(Debug)]
pub(crate) struct Ibmeci {
    buffer_layout: Layout,
    buffer_ptr: *mut u8,
    data: Vec<u8>,
    h_module: HMODULE,
    h_eci: i32,
    thread: u32,
}

impl Ibmeci {
    //noinspection SpellCheckingInspection
    /**
     * 获取一个实例。
     * */
    pub async fn get<'a>() -> Option<&'a Self> {
        unsafe {
            // 单例模式
            if let Some(self_) = IBMECI.get() {
                return Some(self_);
            }
        }
        const LIB_NAME: &str = "ibmeci.dll";
        let url = format!("{}/{}", SERVER_HOME_URI, LIB_NAME);

        let eci_path = get_program_directory().join("libs").join(LIB_NAME);
        if get_file_modified_duration(&eci_path).await > 3600 * 6 {
            // 资源修改时间超过6小时才重新从服务器上克隆，加快启动速度
            let file = clone_resource(url, &eci_path).await;
            if let Err(e) = file {
                error!("{}", e);
                return None;
            }
            drop(file);
        }
        let h_module = loop {
            match load_library(eci_path.to_str().unwrap()) {
                Ok(h) => break h,
                Err(e) => {
                    error!("Can't open the library ({}). {}", eci_path.display(), e);
                    // 文件刚释放可能被安全软件锁定，推迟加载他
                    sleep(Duration::from_millis(1000)).await;
                }
            }
        };
        info!("{} loaded.", eci_path.display());
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            let h_eci = eci!(h_module, new).unwrap_or(0);
            let buffer_layout = Layout::new::<[u8; 8192]>();
            let buffer_ptr = unsafe { alloc_zeroed(buffer_layout) };

            let self_ = Self {
                buffer_layout,
                buffer_ptr,
                data: vec![],
                h_module,
                h_eci,
                thread: get_current_thread_id(),
            };

            eci!(h_module, register_callback, h_eci, _callback_internal, 0);
            eci!(
                h_module,
                set_output_buffer,
                h_eci,
                (buffer_layout.size() / 2) as u32,
                buffer_ptr
            );
            info!("Module handle: {}, eci handle: {}", h_module.0, h_eci);
            unsafe {
                IBMECI.set(self_).unwrap();
                tx.send(IBMECI.get().unwrap()).unwrap();
            }
            message_loop(|m| unsafe {
                if wm!(SYNTH_TASK) == m.message {
                    let b = Box::from_raw(m.wParam.0 as *mut Cow<[u8]>);
                    eci!(h_module, add_text, h_eci, *b);
                    eci!(h_module, synthesize, h_eci);
                    eci!(h_module, synchronize, h_eci);
                    let b = Box::from_raw(m.lParam.0 as *mut Sender<()>);
                    b.send(()).unwrap_or(());
                }
            });
        });
        match rx.await {
            Err(e) => {
                error!("{}", e);
                None
            }
            Ok(x) => Some(x),
        }
    }

    /**
     * 合成语音。
     * */
    pub(crate) async fn synth(&self, text: &str) -> Vec<u8> {
        eci!(self.h_module, stop, self.h_eci);
        let (text, _, unmapped) = GBK.encode(text);
        let text = if unmapped {
            // 如果有不能被编码成gbk的字符，我们需要过滤他们
            let mut v = vec![];
            let mut u = vec![];
            let mut has_html_char = false;
            let mut last_char = 0u8;
            for i in text.iter() {
                let i = i.clone();
                if last_char == 38 {
                    has_html_char = i == 35u8;
                    if has_html_char {
                        u.clear();
                        u.push(last_char);
                        u.push(i);
                    } else {
                        v.push(last_char);
                        v.push(i);
                    }
                } else {
                    if has_html_char {
                        u.push(i);
                        if i == 59u8 {
                            has_html_char = false;
                        } else if !(i >= 48u8 && i <= 57u8) {
                            v.extend(&u);
                            has_html_char = false;
                        }
                    } else if i != 38 {
                        v.push(i);
                    }
                }
                last_char = i;
            }
            Cow::from(v)
        } else {
            text
        };
        if let Some(eci) = unsafe { IBMECI.get_mut() } {
            eci.data.clear();
            let (tx, rx) = channel();
            let tx = Box::new(tx);
            post_thread_message(
                eci.thread,
                wm!(SYNTH_TASK),
                WPARAM(Box::into_raw(Box::new(text)) as usize),
                LPARAM(Box::into_raw(tx) as isize),
            );
            rx.await.unwrap_or(());
            eci.data.clone()
        } else {
            vec![]
        }
    }

    /**
     * 设置语音参数。
     * `params` 参数数据。
     * */
    pub fn set_voice_params(&self, params: &IbmeciVoiceParams) {
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_BREATHINESS,
            params.breathiness
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_HEAD_SIZE,
            params.head_size
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_GENDER,
            params.gender
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_ROUGHNESS,
            params.roughness
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_SPEED,
            params.speed
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_PITCH_BASELINE,
            params.pitch_baseline
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_PITCH_FLUCTUATION,
            params.pitch_fluctuation
        );
        eci!(
            self.h_module,
            set_voice_param,
            self.h_eci,
            0,
            VP_VOLUME,
            params.volume
        );
    }

    /**
     * 获取语音参数。
     * */
    pub fn get_voice_params(&self) -> IbmeciVoiceParams {
        IbmeciVoiceParams {
            gender: eci!(self.h_module, get_voice_param, self.h_eci, 0, VP_GENDER).unwrap_or(0),
            head_size: eci!(self.h_module, get_voice_param, self.h_eci, 0, VP_HEAD_SIZE)
                .unwrap_or(0),
            pitch_baseline: eci!(
                self.h_module,
                get_voice_param,
                self.h_eci,
                0,
                VP_PITCH_BASELINE
            )
            .unwrap_or(0),
            pitch_fluctuation: eci!(
                self.h_module,
                get_voice_param,
                self.h_eci,
                0,
                VP_PITCH_BASELINE
            )
            .unwrap_or(0),
            roughness: eci!(self.h_module, get_voice_param, self.h_eci, 0, VP_ROUGHNESS)
                .unwrap_or(0),
            breathiness: eci!(
                self.h_module,
                get_voice_param,
                self.h_eci,
                0,
                VP_BREATHINESS
            )
            .unwrap_or(0),
            speed: eci!(self.h_module, get_voice_param, self.h_eci, 0, VP_SPEED).unwrap_or(0),
            volume: eci!(self.h_module, get_voice_param, self.h_eci, 0, VP_VOLUME).unwrap_or(0),
        }
    }

    /**
     * 获取发音人列表。
     * */
    pub(crate) fn get_voices(&self) -> Vec<(u32, String)> {
        vec![
            (1, "Adult Male 1"),
            (2, "Adult Female 1"),
            (3, "Child 1"),
            (4, "Adult Male 2"),
            (5, "Adult Male 3"),
            (6, "Adult Female 2"),
            (7, "Elderly Female 1"),
            (8, "Elderly Male 1"),
        ]
        .iter()
        .map(|i| (i.0, i.1.to_string()))
        .collect()
    }

    /**
     * 设置当前发音人。
     * `voice_id` 声音id。
     * */
    pub(crate) fn set_voice(&self, voice_id: u32) {
        eci!(self.h_module, copy_voice, self.h_eci, voice_id, 0);
    }
}

impl Drop for Ibmeci {
    fn drop(&mut self) {
        if !self.h_module.is_invalid() {
            eci!(self.h_module, delete, self.h_eci);
            free_library(self.h_module);
        }
        unsafe {
            dealloc(self.buffer_ptr, self.buffer_layout);
        }
    }
}

unsafe impl Sync for Ibmeci {}

unsafe impl Send for Ibmeci {}

#[cfg(all(test, target_arch = "x86"))]
mod test_eci {
    use super::Ibmeci;
    use rigela_utils::logger::init_logger;

    #[tokio::test]
    async fn main() {
        init_logger(Some("test.log"));
        let eci = Ibmeci::get().await.unwrap();
        for _ in 0..1000 {
            let data = eci.synth("abc‎").await;
            assert_eq!(data.len(), 21978);
        }
    }
}
