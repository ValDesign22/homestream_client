use std::fs::read_to_string;

use tauri::{AppHandle, Manager};

use crate::utils::types::Config;

#[tauri::command]
pub fn get_config(app: AppHandle) -> Option<Config> {
    let config_dir = app.path().app_config_dir();
    if config_dir.is_err() {
        return None;
    }
    let mut config_path = config_dir.unwrap();
    config_path.push("config.json");

    let config = match read_to_string(config_path) {
        Ok(config) => config,
        Err(_) => return None,
    };

    match serde_json::from_str(&config) {
        Ok(config) => Some(config),
        Err(_) => None,
    }
}

#[tauri::command]
pub fn save_config(app: AppHandle, config: Config) -> Result<(), ()> {
    println!("{:?}", config);
    let config_dir = app.path().app_config_dir();
    if config_dir.is_err() {
        return Err(());
    }
    let mut config_path = config_dir.unwrap();
    config_path.push("config.json");

    let config = match serde_json::to_string(&config) {
        Ok(config) => config,
        Err(_) => return Err(()),
    };

    match std::fs::write(config_path, config) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
