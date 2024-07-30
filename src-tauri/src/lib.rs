use commands::config::{get_config, save_config};
use dotenvy::dotenv;
use utils::explore::{explore_movies_folder, explore_series_folder};
use utils::ftp::{create_stream, save_file};
use utils::types::MediaType;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, Manager, State};

pub mod commands;
pub mod utils;

struct SetupState {
    frontend_task: bool,
    backend_task: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().expect("Failed to load .env file");

    tauri::Builder::default()
        .manage(Mutex::new(SetupState {
            frontend_task: false,
            backend_task: false,
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_config, greet, save_config, set_complete])
        .setup(|app| {
            spawn(setup(app.handle().clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn set_complete(
    app: AppHandle,
    state: State<'_, Mutex<SetupState>>,
    task: String,
) -> Result<(), ()> {
    let mut state_lock = state.lock().unwrap();
    match task.as_str() {
        "frontend" => state_lock.frontend_task = true,
        "backend" => state_lock.backend_task = true,
        _ => panic!("invalid task completed!"),
    }
    if state_lock.backend_task && state_lock.frontend_task {
        let splash_window = app.get_webview_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        splash_window.close().unwrap();
        main_window.show().unwrap();
    }
    Ok(())
}

async fn setup(app: AppHandle) -> Result<(), ()> {
    let config = get_config(app.clone());
    if config.is_none() {
        set_complete(
            app.clone(),
            app.state::<Mutex<SetupState>>(),
            "backend".to_string(),
        )
        .await?;
        return Ok(());
    }

    let mut stream = create_stream();

    let movies = explore_movies_folder(&mut stream, MediaType::Movie, None).await;
    let movies_content = serde_json::to_string(&movies).unwrap();
    let _ = save_file(&mut stream, "movies", &movies_content);

    let animes = explore_movies_folder(&mut stream, MediaType::Anime, None).await;
    let animes_content = serde_json::to_string(&animes).unwrap();
    let _ = save_file(&mut stream, "animes", &animes_content);

    let series = explore_series_folder(&mut stream, None).await;
    let series_content = serde_json::to_string(&series).unwrap();
    let _ = save_file(&mut stream, "series", &series_content);

    let _ = stream.quit();

    set_complete(
        app.clone(),
        app.state::<Mutex<SetupState>>(),
        "backend".to_string(),
    )
    .await?;
    Ok(())
}
