/*
 * Copyright (c) 2023. The RigelA open source project team and
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

pub(crate) mod hooks;
pub(crate) mod keyboard;

use crate::commander::keyboard::combo_keys::ComboKey;
use crate::commander::keyboard::combo_keys_manager::ComboKeysManage;
use crate::commander::keyboard::keys::Keys::VkNone;
use crate::{
    commander::hooks::{set_keyboard_hook, set_mouse_hook},
    context::Context,
    talent::Talented,
};
use keyboard::keys::Keys;
use rust_i18n::AtomicStr;
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex, OnceLock, Weak},
};
use win_wrap::hook::WindowsHook;

type Talent = Arc<dyn Talented + Send + Sync>;
type KeyCallbackFn = Arc<dyn Fn(Keys, bool) + Send + Sync>;

/**
 * 命令类型枚举。
 * */
#[allow(dead_code)]
pub(crate) enum CommandType {
    // 键盘命令
    Key(ComboKey),
    // 触摸命令
    Touch,
    // 语音命令
    Voice,
}

/**
 * 指挥官结构。
 * */
#[allow(unused)]
pub(crate) struct Commander {
    keyboard_hook: OnceLock<WindowsHook>,
    mouse_hook: OnceLock<WindowsHook>,
    last_pressed_key: AtomicStr,
    key_callback_fns: Mutex<Vec<(Vec<Keys>, KeyCallbackFn)>>,
    pub(crate) combo_key_manager: Arc<ComboKeysManage>,
}

impl Commander {
    /**
     * 创建一个指挥官对象。
     * 负责收集用户的操作请求，例如快捷键、触摸动作、语音命令等，之后把这些命令调度给具体的任务。
     * */
    pub(crate) fn new() -> Self {
        let key_none: &str = VkNone.into();
        Self {
            keyboard_hook: Default::default(),
            mouse_hook: Default::default(),
            last_pressed_key: AtomicStr::from(key_none),
            key_callback_fns: Mutex::new(Vec::new()),
            combo_key_manager: ComboKeysManage::new().into(),
        }
    }

    /**
     * 让指挥官开始工作。
     * `context` 框架上下文环境，可以通过此对象访问整个框架的所有API。
     * */
    pub(crate) fn apply(&self, context: Weak<Context>) {
        self.keyboard_hook
            .set(set_keyboard_hook(context.clone()))
            .unwrap_or(());

        self.mouse_hook
            .set(set_mouse_hook(context.clone()))
            .unwrap_or(());
    }

    /**
     * 清理环境，后续不可以重复使用。
     * */
    pub(crate) fn dispose(&self) {
        self.keyboard_hook.get().unwrap().unhook();
        self.mouse_hook.get().unwrap().unhook();
    }

    /**
     * 获取最后一次按下的键。
     * */
    pub(crate) fn get_last_pressed_key(&self) -> Keys {
        let text = self.last_pressed_key.to_string();
        text.as_str().into()
    }

    /**
     * 设置最后一次按下的键。
     * `key` 键盘枚举。
     * */
    pub(crate) fn set_last_pressed_key(&self, key: &Keys) {
        let key: &str = key.clone().into();
        self.last_pressed_key.replace(key);
    }

    pub(crate) fn add_key_event_listener(
        &self,
        keys: &[Keys],
        listener: impl Fn(Keys, bool) + Sync + Send + 'static,
    ) {
        self.key_callback_fns
            .lock()
            .unwrap()
            .push((Vec::from(keys), Arc::new(listener)));
    }

    pub(crate) fn get_key_callback_fns(&self) -> Vec<(Vec<Keys>, KeyCallbackFn)> {
        self.key_callback_fns.lock().unwrap().clone()
    }
}

impl Debug for Commander {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Commander").finish()
    }
}
