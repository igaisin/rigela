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

pub(crate) mod tts;

use crate::{
    configs::tts::TtsConfig
};
use rigela_utils::{read_file, write_file};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use log::error;
use toml;

/// 配置项目的根元素
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigRoot {
    pub(crate) tts_config: Option<TtsConfig>,
}

/// 配置管理器
pub struct ConfigManager {
    path: PathBuf,
}

impl ConfigManager {
    /**
     * 创建一个配置管理器
     * */
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /*
     * 读取配置数据。
     * */
    pub(crate) async fn read(&self) -> ConfigRoot {
        match read_file(&self.path.clone()).await {
            Ok(mut content) => toml::from_str::<ConfigRoot>(content.as_mut_str()).unwrap(),

            _ => {
                let config = ConfigRoot {
                    tts_config: Some(TtsConfig::default()),
                };
                self.write(&config).await;
                config
            }
        }
    }

    /**
     * 写出配置数据。
     * `config_root` 完整的配置数据。
     * */
    pub(crate) async fn write(&self, config_root: &ConfigRoot) {
        let path = self.path.clone();
        if let Err(e) = write_file(&path, toml::to_string(config_root).unwrap().as_bytes()).await {
            error!("{}", e);
        }
    }
}
