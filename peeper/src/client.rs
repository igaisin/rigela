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
    model::{PeeperData, PeeperPacket},
    utils::get_pipe_name,
};
use log::{error, trace};
use rigela_utils::pipe::PipeStream;
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};
use tokio::{
    net::windows::named_pipe::{ClientOptions, NamedPipeClient},
    runtime::{Builder, Runtime},
    sync::Mutex,
};

/**
 * peeper的client运行在远进程中。
 * */
pub(crate) struct PeeperClient {
    module: String,
    sender: Arc<OnceLock<Mutex<PipeStream<PeeperPacket, NamedPipeClient>>>>,
    rt: OnceLock<Runtime>,
}

impl PeeperClient {
    /**
     * 创建一个peeper的客户端。
     * */
    pub fn new(module: String) -> Self {
        trace!("New pipe connection.");
        let self_ = Self {
            module,
            sender: OnceLock::new().into(),
            rt: OnceLock::new(),
        };
        let rt = Builder::new_multi_thread()
            .enable_all()
            .worker_threads(1)
            .build();
        if let Err(e) = rt {
            error!("{}", e);
            return self_;
        }
        if let Err(_) = self_.rt.set(rt.unwrap()) {
            error!("Can't create the runtime of the multi thread.");
        }
        self_
    }

    fn get_stream() -> PipeStream<PeeperPacket, NamedPipeClient> {
        match ClientOptions::new().open(get_pipe_name()) {
            Ok(p) => PipeStream::new(p),
            Err(e) => {
                error!("{}", e);
                return Self::get_stream();
            }
        }
    }

    /**
     * 发送数据。
     * `data` peeper的业务数据。
     * */
    pub fn push(&self, data: PeeperData) {
        let packet = PeeperPacket {
            name: self.module.clone(),
            data,
        };
        let sender = self.sender.clone();
        if let Some(rt) = self.rt.get() {
            rt.spawn(async move {
                let sender = sender.get_or_init(|| Self::get_stream().into());
                let mut sender = sender.lock().await;
                if let Err(e) = sender.send(&packet).await {
                    error!("{}", e);
                }
            });
        }
    }

    /**
     * 发送日志。
     * `msg` 日志消息。
     * */
    #[allow(unused)]
    pub fn log(&self, msg: String) {
        self.push(PeeperData::Log(msg));
    }

    /**
     * 退出客户端，这也会通知服务端退出。
     * */
    pub fn quit(&mut self) {
        self.push(PeeperData::Quit);
        self.rt
            .take()
            .unwrap()
            .shutdown_timeout(Duration::from_millis(1000));
    }
}
