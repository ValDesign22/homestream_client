use std::{fs::create_dir_all, path::Path};

use commands::config::{get_config, save_config};
use commands::setup::setup;
use tauri::{Listener, Manager};

pub mod commands;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            setup
        ])
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

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
