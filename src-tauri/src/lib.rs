use std::{fs::create_dir_all, path::Path};

use commands::config::{get_config, save_config};
use commands::setup::setup;
use tauri::{Listener, Manager};
use tauri_plugin_window_state::{AppHandleExt, StateFlags, WindowExt};

pub mod commands;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![get_config, save_config, setup])
        .setup(|app| {
            app.listen("homestream://register", |url| {
                dbg!(url);
            });

            let config_dir = app.path().app_config_dir();
            if config_dir.is_err() {
                return Err("Failed to get config dir".into());
            }
            let config_path = config_dir.unwrap();
            if !Path::new(&config_path).exists() {
                if let Err(error) = create_dir_all(&config_path) {
                    return Err(format!("Failed to create config dir: {}", error).into());
                }
            }

            tauri::async_runtime::block_on(async {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.restore_state(StateFlags::all());

                    #[cfg(debug_assertions)]
                    {
                        let _ = window.open_devtools();
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|app, event| match event {
            tauri::WindowEvent::CloseRequested { .. } | tauri::WindowEvent::Destroyed => {
                let _ = AppHandleExt::save_window_state(app.app_handle(), StateFlags::all());
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
