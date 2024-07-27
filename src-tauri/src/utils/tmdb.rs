use serde_json::Value;
use tauri_plugin_http::reqwest::{self, Error};

use super::types::{Episode, Genre, Movie, Season, Series};

async fn create_request(url: &str) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let tmdb_api_key = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY not set");

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", tmdb_api_key))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await;

    match res {
        Ok(res) => Ok(res),
        Err(e) => Err(e.into()),
    }
}

pub async fn search_movie(
    title: &str,
    date: Option<&str>,
) -> Result<Movie, Box<dyn std::error::Error>> {
    let tmdb_language = std::env::var("TMDB_LANGUAGE").expect("TMDB_LANGUAGE not set");

    let search_url = format!(
        "https://api.themoviedb.org/3/search/movie?query={}&include_adult=false&language={}{}",
        title,
        tmdb_language,
        match date {
            Some(date) => format!("&year={}", date),
            None => "".to_string(),
        }
    );

    let search_res = create_request(&search_url).await;
    if let Err(e) = search_res {
        return Err(e);
    }

    let search_res = search_res.unwrap();
    let search_data: Result<Value, Error> = search_res.json().await;
    if let Err(e) = search_data {
        return Err(e.into());
    }
    let search_data = search_data.unwrap();

    if search_data["results"].as_array().unwrap().is_empty() {
        return Err("No results found".into());
    }

    let movie_id = search_data["results"][0]["id"].as_i64().unwrap();
    let movie_url = format!(
        "https://api.themoviedb.org/3/movie/{}?language={}&append_to_response=release_dates",
        movie_id, tmdb_language
    );

    let movie_res = create_request(&movie_url).await;
    if let Err(e) = movie_res {
        return Err(e);
    }

    let movie_res = movie_res.unwrap();
    let movie_data: Result<Value, Error> = movie_res.json().await;
    if let Err(e) = movie_data {
        return Err(e.into());
    }
    let movie_data = movie_data.unwrap();

    let collection_id = match movie_data["belongs_to_collection"].as_object() {
        Some(collection) => Some(collection["id"].as_i64().unwrap() as i32),
        None => None::<i32>,
    };
    let poster_path = match movie_data["poster_path"].as_str() {
        Some(poster_path) => Some(poster_path.to_string()),
        None => None::<String>,
    };
    let backdrop_path = match movie_data["backdrop_path"].as_str() {
        Some(backdrop_path) => Some(backdrop_path.to_string()),
        None => None::<String>,
    };
    let genres = match movie_data["genres"].as_array() {
        Some(genres) => Some(
            genres
                .iter()
                .map(|genre| {
                    let genre = genre.as_object().unwrap();
                    Genre {
                        id: genre["id"].as_i64().unwrap() as i32,
                        name: genre["name"].as_str().unwrap().to_string(),
                    }
                })
                .collect::<Vec<Genre>>(),
        ),
        None => None::<Vec<Genre>>,
    };

    Ok(Movie {
        id: movie_data["id"].as_i64().unwrap() as i32,
        collection_id,
        title: movie_data["title"].as_str().unwrap().to_string(),
        original_title: movie_data["original_title"].as_str().unwrap().to_string(),
        overview: movie_data["overview"].as_str().unwrap().to_string(),
        poster_path,
        backdrop_path,
        logo_path: None::<String>,
        release_date: movie_data["release_date"].as_str().unwrap().to_string(),
        runtime: movie_data["runtime"].as_i64().unwrap() as i32,
        genres,
        path: None::<String>,
    })
}

pub async fn search_serie(
    title: &str,
    date: Option<&str>,
) -> Result<Series, Box<dyn std::error::Error>> {
    Ok(Series {})
}

pub async fn search_season(serie_id: i32, id: i32) -> Result<Season, Box<dyn std::error::Error>> {
    Ok(Season {})
}

pub async fn search_episode(
    serie_id: i32,
    season_number: i32,
    id: i32,
) -> Result<Episode, Box<dyn std::error::Error>> {
    Ok(Episode {})
}
