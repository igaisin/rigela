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

mod dialog;
mod editor;
mod focus;
mod ime;
mod input;
mod progress;

use crate::commander::keys::Keys;
use crate::event_core::editor::handle_cusor_key;
use crate::event_core::input::handle_lockkey;
use crate::{
    context::Context,
    event_core::{
        dialog::subscribe_dialog_events, editor::subscribe_editor_events,
        focus::subscribe_focus_events, ime::subscribe_ime_events, input::subscribe_input_events,
        progress::subscribe_progress_events,
    },
};
use event_observer::*;
use std::fmt::{Debug, Formatter};
use std::{
    sync::{Arc, Mutex as StdMutex},
    time::{Duration, SystemTime},
};
use tokio::sync::{Mutex, OnceCell};

type EventCoreSubject = Subject<Event>;

/// 事件过滤器
#[derive(Debug)]
pub(crate) struct EventItem {
    same: String,
    time: SystemTime,
}

/// 事件处理中心
#[derive(Clone)]
pub(crate) struct EventCore {
    context: OnceCell<Arc<Context>>,
    filter: Arc<Mutex<Vec<EventItem>>>,
    subject: Arc<StdMutex<EventCoreSubject>>,
}

impl EventCore {
    pub(crate) fn new() -> Self {
        Self {
            context: OnceCell::new(),
            filter: Arc::new(vec![].into()),
            subject: Arc::new(EventCoreSubject::new().into()),
        }
    }

    //noinspection StructuralWrap
    /**
     * 给定一个事件的特征，判断是否应该忽略此事件。
     * `same` 事件的特征文字。
     * `interval` 一个时间内如果此事件出现过，则表示他应该被忽略。
     * */
    pub(crate) async fn should_ignore(&self, same: String, interval: Duration) -> bool {
        let item = EventItem {
            same: same.clone(),
            time: SystemTime::now(),
        };
        let mut lock = self.filter.lock().await;
        for i in lock.iter() {
            if i.same == same && i.time.elapsed().unwrap() < interval {
                return true;
            }
        }
        for (i, j) in lock.iter().enumerate() {
            if j.same == same {
                lock.remove(i);
                break;
            }
        }
        lock.push(item);
        return false;
    }

    /// 启动事件监听
    pub(crate) async fn run(&self, context: Arc<Context>) {
        self.context.set(context.clone()).unwrap_or(());

        // 订阅UIA的焦点元素改变事件
        subscribe_focus_events(context.clone()).await;

        // 监听前台窗口变动
        subscribe_foreground_window_events(context.clone()).await;

        // 订阅对话框事件
        subscribe_dialog_events(context.clone()).await;

        // 订阅输入事件
        subscribe_input_events(context.clone()).await;

        // 订阅输入法候选事件
        subscribe_ime_events(context.clone()).await;

        // 订阅编辑框事件
        subscribe_editor_events(context.clone()).await;

        // 订阅进度栏事件
        subscribe_progress_events(context.clone()).await;

        // 订阅动态事件
        let ctx = context.clone();
        self.subject
            .lock()
            .unwrap()
            .add_arc_observer(ctx.event_core.clone());
    }

    pub(crate) fn get_subject(&self) -> Arc<StdMutex<EventCoreSubject>> {
        self.subject.clone()
    }

    /**
     * 停止所有事件处理。
     * */
    pub(crate) fn shutdown(&self) {}
}

impl Debug for EventCore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventCore").finish()
    }
}

unsafe impl Send for EventCore {}

unsafe impl Sync for EventCore {}

/// 分派事件
impl Observer<Event> for EventCore {
    #[allow(unreachable_patterns)]
    fn on_notify(&self, event: &Event) {
        let context = self.context.get().unwrap().clone();
        let ctx = context.clone();
        let event = event.clone();
        context.main_handler.spawn(async move {
            match event {
                Event::CursorKey(key) => handle_cusor_key(ctx.clone(), key).await,
                Event::LockKey(vk) => handle_lockkey(ctx.clone(), vk).await,
                _ => {}
            }
        });
    }
}

/// 监测前台窗口变动，发送控件元素到form_browser
async fn subscribe_foreground_window_events(context: Arc<Context>) {
    // 给MSAA前台窗口改变绑定处理事件
    let ctx = context.clone();
    context.msaa.add_on_system_foreground_listener(move |src| {
        let form_browser = ctx.form_browser.clone();
        let ui_automation = ctx.ui_automation.clone();

        // form_browser需要异步操作
        ctx.main_handler.spawn(async move {
            if let Some(root) = ui_automation.element_from_handle(src.h_wnd) {
                form_browser.render(Arc::new(root)).await
            }
        });
    });
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub(crate) enum Event {
    CursorKey(Keys),
    LockKey(u16),
}

unsafe impl Send for Event {}
unsafe impl Sync for Event {}
