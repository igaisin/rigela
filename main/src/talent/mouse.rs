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

use crate::context::Context;
use async_trait::async_trait;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Arc;
#[allow(unused_imports)]
use win_wrap::input::{VK_DIVIDE, VK_MULTIPLY};

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标单击", key = ((VK_DIVIDE, true)))]
async fn click(context: Arc<Context>) {
    context.performer.speak_text("单击").await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标右击", key = ((VK_MULTIPLY, false)))]
async fn right_click(context: Arc<Context>) {
    context.performer.speak_text("右击").await;
}
