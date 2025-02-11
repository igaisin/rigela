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
    commander::keyboard::keys::Keys,
    configs::{
        items::{general::Lang, tts::TtsConfig},
        operations::{apply_mouse_config, save_auto_check_update, save_lang, save_run_on_startup},
        ConfigRoot,
    },
    context::{Context, ContextAccessor},
    gui::utils::{
        backup_data, check_update, confirm_update_exists, create_shortcut_link, restore_data,
        set_startup_registry, UpdateState, HELP_DIR,
    },
    talent::Talented,
};
use log::error;
use native_windows_gui::{message, MessageButtons, MessageChoice, MessageIcons, MessageParams};
use rigela_utils::fs::get_rigela_program_directory;
use std::{
    env::current_exe,
    fs::remove_file,
    path::{Path, PathBuf},
    process::Command,
    sync::Weak,
};
use win_wrap::{
    common::{message_box, MB_OK},
    shell::{get_known_folder_path, FOLDERID_Desktop, KF_FLAG_DEFAULT},
};

/// 退出程序。
pub(crate) fn exit_cmd(context: Weak<Context>) {
    let talent = context.get_talent_provider().get_exit_talent();
    talent.perform(context);
}

/// 打开帮助文档。
pub(crate) fn help_cmd(_context: Weak<Context>) {
    let help_path = get_rigela_program_directory().join(HELP_DIR);
    Command::new("notepad")
        .arg(help_path)
        .spawn()
        .expect("Failed to start notepad");
}

/// 打开设置窗口。
pub(crate) fn settings_cmd(context: Weak<Context>) {
    context.get_gui_provider().show_settings_form();
}

/// 检查更新。
#[allow(unused)]
pub(crate) fn check_update_cmd(context: Weak<Context>, auto: bool) {
    // {
    //     #[cfg(debug_assertions)]
    //     return;
    // }

    context.get_work_runtime().spawn(async move {
        match (auto, check_update().await) {
            (true, UpdateState::None) => {
                // 如果是自动检查更新，且检查失败，则不做任何操作
                return;
            }
            (false, UpdateState::None) => {
                // 手动检查, 未检测到更新需要弹窗提示
                message_box(
                    None,
                    &t!("command.msg_newest_version"),
                    &t!("command.msg_mind_title"),
                    MB_OK,
                );
                return;
            }
            (_, UpdateState::Updated) => {
                message_box(
                    None,
                    &t!("command.msg_newest_version"),
                    &t!("command.msg_mind_title"),
                    MB_OK,
                );
                return;
            }
            _ => {}
        };

        // 启动更新器
        let child = match confirm_update_exists().await {
            Ok(_) => Command::new("cmd.exe")
                // 需要使用cmd.exe辅助启动，使用start参数不等待，否则当更新器尝试kill主进程时，更新器自己也会被kill
                .arg("/c")
                .arg("start")
                .arg(get_rigela_program_directory().join("libs/update.exe"))
                .arg(current_exe().unwrap().to_str().unwrap())
                .spawn(),
            Err(_) => {
                message_box(
                    None,
                    &t!("command.msg_no_updater"),
                    &t!("command.msg_mind_title"),
                    MB_OK,
                );
                return;
            }
        };
        if let Err(e) = child {
            error!("Failed to start update.exe. {}", e);
        }
    });
}

/// 打开自定义热键窗口。
pub(crate) fn custom_hotkeys_cmd(context: Weak<Context>) {
    context.get_gui_provider().show_hotkeys_form();
}

/// 打开欢迎界面。
pub(crate) fn welcome_form_cmd(context: Weak<Context>) {
    context.get_gui_provider().show_welcome_form();
}

/// 打开捐赠界面。
pub(crate) fn donate_cmd(_context: Weak<Context>) {
    message_box(None, &t!("command.msg_thanks"), "RigelA", MB_OK);
}

/// 打开关于窗口
pub(crate) fn about_form_cmd(context: Weak<Context>) {
    context.get_gui_provider().show_about_form();
}

/// 访问开源官网
pub(crate) fn visit_host_website_cmd(_context: Weak<Context>) {
    const URL: &str = "https://github.com/mzdk100/rigela";

    Command::new("cmd")
        .args(&["/c", "start", URL])
        .spawn()
        .expect("Failed to start cmd");
}

/// 设置开机自动启动
pub(crate) fn set_auto_start_cmd(context: Weak<Context>, toggle: bool) {
    if set_startup_registry(toggle).is_err() {
        error!("registry operation failed! ");
    }
    save_run_on_startup(context.clone(), toggle);

    let msg = if toggle {
        t!("command.msg_auto_start_on").to_string()
    } else {
        t!("command.msg_auto_start_off").to_string()
    };

    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        ctx.get_performer().speak(&msg).await;
    });
}

/// 设置自动检测更新
pub(crate) fn set_auto_check_update_cmd(context: Weak<Context>, toggle: bool) {
    save_auto_check_update(context.clone(), toggle);

    let msg = if toggle {
        t!("command.msg_auto_check_on").to_string()
    } else {
        t!("command.msg_auto_check_off").to_string()
    };

    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        ctx.get_performer().speak(&msg).await;
    });
}

/// 设置语言
pub(crate) fn set_lang_cmd(context: Weak<Context>, index: usize) {
    let lang = match index {
        1 => Lang::En,
        2 => Lang::Zh,
        _ => Lang::FollowSystem,
    };
    save_lang(context.clone(), &lang);

    let msg = match lang {
        Lang::Zh => t!("command.msg_switch_to_zh"),
        Lang::En => t!("command.msg_switch_to_en"),
        _ => t!("command.msg_switch_to_follow_system"),
    }
    .to_string();

    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        ctx.get_performer().speak(&msg).await;
    });
}

/// 设置语音角色
pub(crate) fn set_voice_cmd(context: Weak<Context>, engine: String, name: String) {
    let ctx = context.clone();
    let tts = context.get_performer().get_tts().clone();
    context.get_work_runtime().spawn(async move {
        let info = tts
            .get_all_voiceinfo()
            .await
            .iter()
            .find(|v| v.name == name && v.engine == engine)
            .unwrap()
            .clone();

        let mut root = ctx.get_config_manager().get_config();
        let config = TtsConfig {
            voice: (info.engine, info.id),
            ..root.tts_config
        };
        root.tts_config = config.clone();
        ctx.get_config_manager().set_config(&root);
        tts.apply_config(&config.clone()).await;
        ctx.get_performer()
            .speak(&t!("command.tts_role", value = info.name))
            .await;
    });
}

/// 设置语音速度
pub(crate) fn set_speed_cmd(context: Weak<Context>, index: usize) {
    let speed = 100 - index as i32;

    let mut root = context.get_config_manager().get_config();
    let config = TtsConfig {
        speed,
        ..root.tts_config
    };
    root.tts_config = config.clone();
    context.get_config_manager().set_config(&root);

    let tts = context.get_performer().get_tts();
    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        tts.apply_config(&config.clone()).await;
        ctx.get_performer()
            .speak(&t!("command.tts_speed", value = speed))
            .await;
    });
}

/// 设置语音音调
pub(crate) fn set_pitch_cmd(context: Weak<Context>, index: usize) {
    let pitch = 100 - index as i32;

    let mut root = context.get_config_manager().get_config();
    let config = TtsConfig {
        pitch,
        ..root.tts_config
    };
    root.tts_config = config.clone();
    context.get_config_manager().set_config(&root);

    let tts = context.get_performer().get_tts();
    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        tts.apply_config(&config.clone()).await;
        ctx.get_performer()
            .speak(&t!("command.tts_pitch", value = pitch))
            .await;
    });
}

/// 设置语音音量
pub(crate) fn set_volume_cmd(context: Weak<Context>, index: usize) {
    let volume = 100 - index as i32;

    let mut root = context.get_config_manager().get_config();
    let config = TtsConfig {
        volume,
        ..root.tts_config
    };
    root.tts_config = config.clone();
    context.get_config_manager().set_config(&root);

    let tts = context.get_performer().get_tts();
    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        tts.apply_config(&config.clone()).await;
        ctx.get_performer()
            .speak(&t!("command.tts_volume", value = volume))
            .await;
    });
}

/// 设置鼠标朗读
pub(crate) fn set_mouse_read_cmd(context: Weak<Context>, toggle: bool) {
    apply_mouse_config(context.clone(), toggle);

    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        let state = match toggle {
            true => t!("command.msg_mouse_read_on"),
            false => t!("command.msg_mouse_read_off"),
        };
        ctx.get_performer().speak(&state).await;
    });
}

/// 导出配置
pub(crate) fn export_config_cmd(_context: Weak<Context>, path: PathBuf) {
    if backup_data(&path).is_err() {
        error!("备份数据失败");
    }

    message_box(
        None,
        &t!("command.msg_export_success"),
        &t!("command.msg_mind_title"),
        MB_OK,
    );
}

/// 导入配置
pub(crate) fn import_config_cmd(context: Weak<Context>, path: PathBuf) {
    if restore_data(&path).is_err() {
        error!("Can't restore the data.");
    }

    context.get_config_manager().apply();
    reapply_config(context.clone());

    message_box(
        None,
        &t!("command.msg_import_success"),
        &t!("command.msg_mind_title"),
        MB_OK,
    );
}

/// 还原默认配置
pub(crate) fn reset_config_cmd(context: Weak<Context>) {
    let msg_params = MessageParams {
        title: &t!("command.msg_confirm_title"),
        content: &t!("command.msg_reset_confirm"),
        buttons: MessageButtons::OkCancel,
        icons: MessageIcons::Question,
    };
    if message(&msg_params) == MessageChoice::Cancel {
        return;
    }

    context
        .get_config_manager()
        .set_config(&ConfigRoot::default());
    reapply_config(context.clone());
}

// 重新应用配置
fn reapply_config(context: Weak<Context>) {
    let config = context.get_config_manager().get_config();

    let enable = config.general_config.run_on_startup.clone();
    if set_startup_registry(enable).is_err() {
        error!("关闭开机自动启动失败");
    }

    let ctx = context.clone();
    let tts = context.get_performer().get_tts();
    let tts_cfg = config.tts_config.clone();
    context.get_work_runtime().spawn(async move {
        // 应用配置到TTS
        tts.apply_config(&tts_cfg.clone()).await;

        // 重新显示设置界面，更新界面上的状态值
        ctx.get_gui_provider().show_settings_form();

        ctx.get_performer()
            .speak(&t!("command.msg_reset_success"))
            .await;
    });
}

/// 获取桌面快捷方式的路径
pub(crate) fn get_desktop_shortcut_path() -> PathBuf {
    let path = get_known_folder_path(&FOLDERID_Desktop, KF_FLAG_DEFAULT, None).unwrap();
    Path::new(&path).join(t!("command.program_name").to_string() + ".lnk")
}

/// 添加桌面快捷方式
pub(crate) fn add_desktop_shortcut_cmd(context: Weak<Context>, toggle: bool, keys: &[Keys]) {
    let path = get_desktop_shortcut_path();
    match toggle {
        true => {
            create_shortcut_link(path.to_str().unwrap().to_string(), keys);
        }
        false => remove_file(&path).unwrap_or(()),
    }

    let ctx = context.clone();
    context.get_work_runtime().spawn(async move {
        let state = if toggle {
            t!("command.added_desktop_shortcut")
        } else {
            t!("command.deleted_desktop_shortcut")
        };
        ctx.get_performer().speak(&state).await;
    });
}
