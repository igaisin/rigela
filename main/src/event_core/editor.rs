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

use crate::{
    commander::keys::Keys::{VkDown, VkUp},
    context::Context,
};
use a11y::ia2::{
    text::IA2TextBoundaryType::{IA2_TEXT_BOUNDARY_CHAR, IA2_TEXT_BOUNDARY_LINE},
    WinEventSourceExt,
};
use log::error;
use std::sync::Arc;
use win_wrap::uia::pattern::text::{TextUnit, UiAutomationTextPattern};

//noinspection SpellCheckingInspection
/**
 * 订阅编辑器事件。
 * `context` 读屏框架的上下文环境。
 * */
pub(crate) async fn subscribe_editor_events(context: Arc<Context>) {
    subscribe_uia_events(context.clone()).await;
    subscribe_ia2_events(context).await;
}

async fn subscribe_uia_events(context: Arc<Context>) {
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();
    let commander = context.commander.clone();
    let ui_automation = context.ui_automation.clone();
    let root = ui_automation.get_root_element();

    let group = ui_automation.create_event_handler_group();
    // group.add_active_text_position_changed_listener(|element, range| {});
    // group.add_text_edit_text_changed_listener(|element| {});
    // group.add_changes_listener(|| {});

    group.add_text_selection_changed_listener(move |element| {
        let text_pattern = match UiAutomationTextPattern::obtain(&element) {
            Ok(p) => p,
            Err(_) => return,
        };
        let selection = text_pattern.get_selection();
        if selection.is_empty() {
            return;
        }
        let caret = selection[0].clone();
        match commander.get_last_pressed_key() {
            VkUp | VkDown => caret.expand_to_enclosing_unit(TextUnit::Line),
            _ => caret.expand_to_enclosing_unit(TextUnit::Character),
        }
        let performer = performer.clone();
        main_handler.spawn(async move {
            performer.speak(caret).await;
        });
    });

    if ui_automation
        .add_event_handler_group(&root, &group)
        .is_err()
    {
        error!("Add the event handler group of the uia is failed.");
    }

    context
        .terminator
        .add_exiting_listener(move || ui_automation.remove_event_handler_group(&root, &group))
        .await;
}

async fn subscribe_ia2_events(context: Arc<Context>) {
    let commander = context.commander.clone();
    let main_handler = context.main_handler.clone();
    let performer = context.performer.clone();

    context.ia2.add_on_text_caret_moved_listener(move |src| {
        let text = match src.get_text() {
            Ok(t) => t,
            Err(e) => {
                error!("{}", e);
                return;
            }
        };
        let caret = text.caret_offset().unwrap_or(0);
        let (_, _, text) = match commander.get_last_pressed_key() {
            VkDown | VkUp => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_LINE),
            _ => text.text_at_offset(caret, IA2_TEXT_BOUNDARY_CHAR),
        };
        let performer = performer.clone();
        main_handler.spawn(async move {
            performer.speak(text).await;
        });
    })
}
