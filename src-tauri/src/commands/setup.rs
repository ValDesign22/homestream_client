use tauri::AppHandle;
use crate::utils::explore::{explore_movies_folder, explore_tv_shows_folder};
use crate::utils::ftp::{create_stream, save_file};
use crate::utils::types::MediaType;

use super::config::get_config;

#[tauri::command]
pub async fn setup(app: AppHandle) -> Result<(), ()> {
    let config = get_config(app.clone());
    if config.is_none() {
        return Ok(());
    }
    let config = config.unwrap();

    let mut stream = create_stream(&config);

    for folder in &config.folders {
        match folder.media_type {
            MediaType::Movie => {
                let movies = explore_movies_folder(&mut stream, &config, folder).await;
                let movies_content = serde_json::to_string(&movies).unwrap();
                let _ = save_file(&mut stream, &config, folder.id.to_string(), &movies_content);
            }
            MediaType::TvShow => {
                let tv_shows = explore_tv_shows_folder(&mut stream, &config, folder).await;
                let tv_shows_content = serde_json::to_string(&tv_shows).unwrap();
                let _ = save_file(&mut stream, &config, folder.id.to_string(), &tv_shows_content);
            }
        }
    }

    let _ = stream.quit();

    Ok(())
}
