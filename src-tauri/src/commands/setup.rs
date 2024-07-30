use tauri::AppHandle;
use crate::utils::explore::{explore_movies_folder, explore_series_folder};
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

    let movies = explore_movies_folder(&mut stream, &config, MediaType::Movie, None).await;
    let movies_content = serde_json::to_string(&movies).unwrap();
    let _ = save_file(&mut stream, &config, "movies", &movies_content);

    let animes = explore_movies_folder(&mut stream, &config, MediaType::Anime, None).await;
    let animes_content = serde_json::to_string(&animes).unwrap();
    let _ = save_file(&mut stream, &config, "animes", &animes_content);

    let series = explore_series_folder(&mut stream, &config, None).await;
    let series_content = serde_json::to_string(&series).unwrap();
    let _ = save_file(&mut stream, &config, "series", &series_content);

    let _ = stream.quit();

    Ok(())
}
