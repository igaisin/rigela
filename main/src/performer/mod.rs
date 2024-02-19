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

pub(crate) mod cache;
pub(crate) mod sound;
pub(crate) mod tts;

use crate::performer::sound::SoundArgument;
use crate::{
    context::Context,
    performer::{
        sound::Sound,
        tts::{sapi5::Sapi5Engine, vvtts::VvttsEngine, Tts},
    },
};
use std::{sync::Arc, time::Duration};
use tokio::{sync::OnceCell, time::sleep};

/// 表演者语音信息收集接口。 实现此接口的对象可以调用表演者的speak方法进行输出。
pub(crate) trait Speakable {
    fn get_sentence(&self) -> String;
}

/// 表演者对象结构。 可以进行语音输出或音效提示。
#[derive(Debug)]
pub(crate) struct Performer {
    tts: OnceCell<Arc<Tts>>,
    sound: Arc<Sound>,
    cache: Arc<cache::Cache>,
}

impl Performer {
    /// 创建表演者对象。
    pub(crate) fn new() -> Self {
        Self {
            tts: OnceCell::new().into(),
            sound: Sound::new().into(),
            cache: Arc::new(cache::Cache::new()).into(),
        }
    }

    //noinspection StructuralWrap
    /**
     * 配置表演者。
     * `context` 读屏框架的上下文环境。
     * */
    pub(crate) async fn apply(&self, context: Arc<Context>) {
        self.sound.apply(context.clone());

        let tts = Arc::new(Tts::new(context.clone()));
        self.tts.set(tts.clone()).unwrap_or(());
        tts.put_default_engine(Sapi5Engine::new())
            .await
            .add_engine(VvttsEngine::new(context.clone()))
            .await;
    }

    /// 获取表演者的TTS对象
    pub(crate) fn get_tts(&self) -> Arc<Tts> {
        self.tts.get().unwrap().clone()
    }

    /// 获取表演者的缓冲区
    pub(crate) fn get_cache(&self) -> Arc<cache::Cache> {
        self.cache.clone()
    }

    /**
     * 朗读文字，如果当前有朗读的任务，则进行排队。
     * 本方法会等待朗读完毕，如果朗读成功，则返回true；如果中途通过stop函数停止，或者朗读失败，则返回false。
     * `speakable` 实现了Speakable特征的对象。
     * */
    pub(crate) async fn speak(&self, speakable: impl Speakable) -> bool {
        let text = speakable.get_sentence().trim_end().to_string();
        if text.is_empty() {
            return false;
        }

        loop {
            if let Some(tts) = self.tts.get() {
                tts.stop().await;
                // 更新缓存
                self.cache.update(text.clone()).await;
                return tts.speak(text).await;
            }
            // 如果tts没有加载好，就继续等待
            sleep(Duration::from_millis(100)).await;
        }
    }

    /// 播放音效
    pub(crate) async fn play_sound(&self, arg: SoundArgument) {
        self.sound.stop_all().await;
        self.sound.play(arg).await;
    }
}
