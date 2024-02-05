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

use crate::commander::keys::Keys;
use crate::{bring_window_front, commander::CommandType::Key, context::Context};
use nwd::NwgUi;
use nwg::{InsertListViewItem, NativeUi};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock, RwLock};
use win_wrap::{
    common::LRESULT,
    ext::LParamExt,
    hook::{KbdLlHookStruct, WindowsHook, HOOK_TYPE_KEYBOARD_LL, LLKHF_EXTENDED},
    input::{WM_KEYDOWN, WM_SYSKEYDOWN},
};

#[derive(Default, NwgUi)]
pub struct HotKeysForm {
    context: RefCell<Option<Arc<Context>>>,
    hook: RefCell<Option<WindowsHook>>,
    enable_hook: Arc<Mutex<bool>>,

    #[nwg_control(size: (600, 480), position: (300, 300), title: "热键自定义", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [HotKeysForm::exit], OnInit: [HotKeysForm::load_data])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 10)]
    layout: nwg::GridLayout,

    #[nwg_control(size: (580, 400), list_style: nwg::ListViewStyle::Detailed, focus: true,
    ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, col_span: 6, row: 0, row_span: 6)]
    data_view: nwg::ListView,

    #[nwg_control(text: "自定义: ")]
    #[nwg_layout_item(layout: layout, col: 0, row: 6)]
    label: nwg::Label,

    #[nwg_control(readonly: true, text: "请输入新的热键!")]
    #[nwg_layout_item(layout: layout, col: 1, row: 6, col_span: 3)]
    #[nwg_events(OnKeyPress: [HotKeysForm::tb_key_press(SELF, EVT_DATA)])]
    text_box: nwg::TextInput,

    #[nwg_control(text: "设置 (&S)")]
    #[nwg_layout_item(layout: layout, col: 4, row: 6)]
    #[nwg_events(OnButtonClick: [HotKeysForm::set_hotkey])]
    set_btn: nwg::Button,

    #[nwg_control(text: "清除 (&C)")]
    #[nwg_layout_item(layout: layout, col: 5, row: 6)]
    #[nwg_events(OnButtonClick: [HotKeysForm::clear_hotkey])]
    clear_btn: nwg::Button,

    #[nwg_control()]
    #[nwg_events(OnNotice: [HotKeysForm::new_hotkeys])]
    new_hotkeys: nwg::Notice,

    #[nwg_control()]
    #[nwg_events(OnNotice: [HotKeysForm::next_control])]
    next_control: nwg::Notice,
}

impl HotKeysForm {
    fn load_data(&self) {
        const COL_DATA: [(i32, &str); 3] =
            [(100, "技能名称"), (240, "初始热键"), (240, "自定义热键")];
        let dv = &self.data_view;

        for (i, (n, s)) in COL_DATA.into_iter().enumerate() {
            dv.insert_column(nwg::InsertListViewColumn {
                index: Some(i as i32),
                fmt: None,
                width: Some(n),
                text: Some(s.into()),
            });
        }

        dv.set_headers_enabled(true);
        self.update_list();

        *self.hook.borrow_mut() = Some(self.set_hook());
    }

    fn update_list(&self) {
        let dv = &self.data_view;
        dv.clear();

        if let Some(context) = self.context.borrow().clone() {
            let talents = context.talent_accessor.talents.clone();

            for (i, talent) in talents.iter().enumerate() {
                dv.insert_item(talent.get_doc());

                for cmd_type in talent.get_supported_cmd_list() {
                    if let Key(keys) = cmd_type {
                        let key_str = keys
                            .iter()
                            .map(|k| -> &str { (*k).into() })
                            .collect::<Vec<&str>>()
                            .join(" + ");

                        dv.insert_item(InsertListViewItem {
                            index: Some(i as i32),
                            column_index: 1,
                            text: Some(key_str),
                            image: None,
                        });
                        break;
                    }
                }
            }
        }
    }

    fn set_hotkey(&self) {
        nwg::modal_info_message(&self.window, "Hotkey", "Press a hotkey");
    }

    fn clear_hotkey(&self) {
        nwg::modal_info_message(&self.window, "Hotkey", "Clear hotkey");
    }

    fn exit(&self) {
        self.hook.take().unwrap().unhook();
        nwg::stop_thread_dispatch();
    }

    fn set_hook(&self) -> WindowsHook {
        let key_track: RwLock<HashMap<Keys, bool>> = RwLock::new(HashMap::new());
        let enable_hook: Arc<Mutex<bool>> = Arc::clone(&self.enable_hook);
        let new_hotkeys_sender = self.new_hotkeys.sender();
        let next_ctrl_sender = self.next_control.sender();

        WindowsHook::new(HOOK_TYPE_KEYBOARD_LL, move |w_param, l_param, next| {
            let info: &KbdLlHookStruct = l_param.to();
            let is_extended = info.flags.contains(LLKHF_EXTENDED);
            let pressed = w_param.0 == WM_KEYDOWN as usize || w_param.0 == WM_SYSKEYDOWN as usize;

            let mut map = key_track.write().unwrap();
            map.insert((info.vkCode, is_extended).into(), pressed);

            if !pressed {
                drop(map); // 必须先释放锁再next()，否则可能会死锁
                return next();
            }

            if *enable_hook.lock().unwrap() {
                let keys = map
                    .iter()
                    .filter(|(_, p)| **p)
                    .map(|(x, _)| *x)
                    .collect::<Vec<Keys>>();

                if keys.len() == 1 && keys[0] == Keys::VkTab {
                    next_ctrl_sender.notice();
                    return LRESULT(1);
                }

                hotkeys().lock().unwrap().clear();
                hotkeys().lock().unwrap().extend(keys);

                new_hotkeys_sender.notice();
                return LRESULT(1);
            }

            drop(map); // 必须先释放锁再next()，否则可能会死锁
            next()
        })
    }

    fn new_hotkeys(&self) {
        let key_str = hotkeys()
            .lock()
            .unwrap()
            .iter()
            .map(|x| -> &str { (*x).into() })
            .collect::<Vec<&str>>()
            .join("+");
        self.text_box.set_text(&key_str);
    }

    fn next_control(&self) {
        *self.enable_hook.lock().unwrap() = false;
        self.set_btn.set_focus();
    }

    fn tb_key_press(&self, data: &nwg::EventData) {
        if data.on_key() != nwg::keys::TAB {
            *self.enable_hook.lock().unwrap() = true;
        }
    }

    fn set_context(&self, context: Arc<Context>) {
        *self.context.borrow_mut() = Some(context);
    }
}

fn hotkeys() -> &'static Mutex<Vec<Keys>> {
    static INSTANCE: OnceLock<Mutex<Vec<Keys>>> = OnceLock::new();
    INSTANCE.get_or_init(|| Mutex::new(vec![]))
}

pub(crate) fn show(context: Arc<Context>) {
    nwg::init().expect("Failed to init Native Windows GUI");
    let ui = HotKeysForm::build_ui(Default::default()).expect("Failed to build UI");
    ui.set_context(context);
    bring_window_front!(ui.window);
    nwg::dispatch_thread_events();
}
