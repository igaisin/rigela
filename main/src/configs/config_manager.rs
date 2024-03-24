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

use crate::configs::{
    general::GeneralConfig, hotkeys::HotKeysConfig, mouse::MouseConfig,
    navigation::NavigationConfig, tts::TtsConfig,
};
use log::{error as err_log, info};
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, File},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

/// 配置项目的根元素
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub(crate) struct ConfigRoot {
    pub(crate) tts_config: TtsConfig,
    pub(crate) mouse_config: MouseConfig,
    pub(crate) hotkeys_config: HotKeysConfig,
    pub(crate) general_config: GeneralConfig,
    pub(crate) navigation_config: NavigationConfig,
}

/// 配置管理器
#[derive(Debug)]
pub(crate) struct ConfigManager {
    // 配置文件的路径
    pub(crate) path: PathBuf,
    // 当前的配置
    config: Arc<Mutex<ConfigRoot>>,
    // 更新配置的操作时间
    update_time: Arc<Mutex<Instant>>,
    // 延时写入是否完成
    write_finished: Arc<Mutex<bool>>,
}

impl ConfigManager {
    /**
     * 创建一个配置管理器
     * */
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            config: Default::default(),
            update_time: Arc::new(Mutex::new(Instant::now())),
            write_finished: Mutex::new(true).into(),
        }
    }

    /// 初始化当前配置，从配置文件获取配置信息
    pub(crate) fn apply(&self) {
        *self.config.lock().unwrap() = self.read();

        // 设置当前程序显示语言
        let lang: String = self.get_config().general_config.lang.clone().into();
        rust_i18n::set_locale(&lang);
        info!("The current locale of the user is {lang}.");
    }

    /// 获取当前的配置
    pub(crate) fn get_config(&self) -> ConfigRoot {
        self.config.lock().unwrap().clone()
    }

    /// 修改当前的配置，修改完写入配置文件
    pub(crate) fn set_config(&self, config: &ConfigRoot) {
        *self.config.lock().unwrap() = config.clone();

        *self.update_time.lock().unwrap() = Instant::now();
        self.write();
    }

    /*
     * 读取配置数据。如果不存在配置文件，写入默认配置
     * */
    pub(crate) fn read(&self) -> ConfigRoot {
        _read_config(&self.path).unwrap_or_else(|_| {
            err_log!("The config file is invalid, this will be regenerated.");
            _write_config(&self.path, &ConfigRoot::default()).unwrap_or_else(|_| {
                err_log!("Can't write the default config file.");
            });
            info!("Using default config.");
            Default::default()
        })
    }

    // 延时写出配置数据。
    fn write(&self) {
        let path = self.path.clone();
        let config = self.config.clone();
        let update_time = self.update_time.clone();
        let write_finished = self.write_finished.clone();

        // 写出操作再新的线程进行
        spawn(move || {
            // 延时写入没有完成前，防止重复调用
            if !*write_finished.lock().unwrap() {
                return;
            }
            {
                *write_finished.lock().unwrap() = false;
            }

            // 延迟到当前配置更新时间的推后10秒钟
            loop {
                if Instant::now() >= *update_time.lock().unwrap() + Duration::from_secs(10) {
                    break;
                }
                sleep(Duration::from_secs(10));
            }

            // 开始执行配置写入
            let cfg = config.lock().unwrap().clone();
            _write_config(&path, &cfg).unwrap_or_else(|_| err_log!("Can't write the config file."));

            *write_finished.lock().unwrap() = true;
        });
    }

    /// 直接保存配置，不延时
    pub(crate) fn save_config(&self) {
        let path = self.path.clone();
        let config = self.config.clone();

        let _job = spawn(move || {
            let cfg = config.lock().unwrap().clone();
            _write_config(&path, &cfg).unwrap_or_else(|_| err_log!("写入配置文件失败"));
        })
        .join();
    }
}

// 写出配置到指定文件
fn _write_config(path: &PathBuf, config: &ConfigRoot) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = toml::to_string(config)?;
    let mut file = File::create(&path)?;
    file.write_all(&cfg.into_bytes())?;

    Ok(())
}

// 读取配置文件
fn _read_config(path: &PathBuf) -> Result<ConfigRoot, Box<dyn std::error::Error>> {
    let mut data = read_to_string(path.clone())?;
    let cfg = toml::from_str::<ConfigRoot>(data.as_mut_str())?;

    Ok(cfg)
}
