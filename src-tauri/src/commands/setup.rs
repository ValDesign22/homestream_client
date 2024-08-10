use tauri::AppHandle;
use tauri_plugin_http::reqwest;

use super::config::get_config;

#[tauri::command]
pub async fn setup(app: AppHandle) -> Result<(), ()> {
    let config = get_config(app.clone());
    if config.is_none() {
        return Ok(());
    }
    let config = config.unwrap();

    let res = reqwest::get(format!("{}/setup", config.http_server).as_str()).await;
    if res.is_err() {
        println!("{:?}", res.err());
        return Err(());
    }

    Ok(())
}
