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

use crate::audio::AudioOutputStream;
use std::{
    ops::Add,
    sync::{Arc, Mutex},
};
use windows::{
    core::HSTRING, Media::SpeechSynthesis::SpeechSynthesizer, Storage::Streams::DataReader,
};

#[derive(Clone)]
pub struct Tts {
    synth: Arc<SpeechSynthesizer>,
    output_stream: Arc<AudioOutputStream>,
    task_id: Arc<Mutex<u32>>,
}

impl Tts {
    /**
     * 创建一个TTS对象（语音合成，SAPI5）
     * */
    pub fn new() -> Self {
        // 创建语音合成器
        let synth = SpeechSynthesizer::new().expect("Can't create the speech synthesizer.");
        // 创建音频输出流
        let stream = AudioOutputStream::new(16000, 1);
        Self {
            synth: synth.into(),
            output_stream: stream.into(),
            task_id: Arc::new(0u32.into()),
        }
    }

    /**
     * 设置语速。
     * 某些语音的最低语速快于 0.5，最大语速低于 6.0。
     * 说话率不能直接转换为每分钟单词数，因为每种语音和语言的默认语速可能不同。
     * `val` 语速值。
     * */
    pub fn set_speed(&self, val: u32) {
        // https://learn.microsoft.com/zh-cn/uwp/api/windows.media.speechsynthesis.speechsynthesizeroptions.speakingrate?view=winrt-22621#windows-media-speechsynthesis-speechsynthesizeroptions-speakingrate
        self.synth
            .Options()
            .unwrap()
            .SetSpeakingRate(2.0 + (val as f64 - 50.0) * 0.02)
            .expect("Can't set the speed value.");
    }

    /**
     * 朗读一段文字（直接播放）
     * 此函数是异步函数，需要使用.await。
     * `text` 要朗读的文字。
     * */
    pub async fn speak(&self, text: &str) {
        let current_id = {
            let mut lock = self.task_id.lock().unwrap();
            let index = lock.add(1);
            *lock = index;
            index
        };
        let stream = self
            .synth
            .SynthesizeTextToStreamAsync(&HSTRING::from(text))
            .unwrap()
            .await
            .unwrap();
        let size = stream.Size().unwrap();
        let reader = DataReader::CreateDataReader(&stream).unwrap();
        reader.LoadAsync(size as u32).unwrap().await.unwrap();
        self.output_stream.flush();
        self.output_stream.stop();
        self.output_stream.start();
        // 跳过音频文件头的44个字节
        let mut data: [u8; 44] = [0; 44];
        reader.ReadBytes(&mut data).unwrap();
        loop {
            // 获取合成任务的id
            let id = match self.task_id.lock() {
                Ok(x) => *x,
                Err(_) => 0u32,
            };
            if id != current_id {
                // 这里检查是否已经有新的合成任务，如果有就打断当前的合成任务
                break;
            }
            let mut data: [u8; 3200] = [0; 3200];
            reader.ReadBytes(&mut data).unwrap_or(());
            self.output_stream.write(&data).await;
            if let Ok(x) = reader.UnconsumedBufferLength() {
                if x < data.len() as u32 {
                    break;
                }
            }
        }
    }
}
