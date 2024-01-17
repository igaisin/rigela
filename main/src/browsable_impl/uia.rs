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

use crate::browser::form_browser::BrowserElement;
use crate::browser::Browsable;
use std::sync::Arc;
use win_wrap::uia::element::UiAutomationElement;
use win_wrap::uia::pattern::UiAutomationLegacyIAccessiblePattern;

impl Browsable for UiAutomationElement {
    fn get_name(&self) -> String {
        let mut name = self.get_name();

        if name.is_empty() {
            let accessible: UiAutomationLegacyIAccessiblePattern = self.into();
            name = accessible.get_name();

            if name.is_empty() {
                name = accessible.get_description();
            }
        }

        name
    }

    fn get_role(&self) -> String {
        self.get_localized_control_type()
    }
    fn get_child_count(&self) -> usize {
        self.get_child_count() as usize
    }
    fn get_child(&self, index: usize) -> Option<BrowserElement> {
        if let Some(x) = self.get_child(index as i32) {
            return Some(Arc::new(x));
        }
        None
    }
}
