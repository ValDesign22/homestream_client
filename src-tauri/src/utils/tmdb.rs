use serde_json::Value;
use strsim::levenshtein;
use tauri_plugin_http::reqwest;

use super::types::{Config, Episode, Genre, ImagesResponse, MediaType, Movie, Season, TvShow};

async fn create_request(
    url: &str,
    config: &Config,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let tmdb_api_key = &config.tmdb_api_key;

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

fn find_image_path<'a>(images: &'a [Value], language: &str) -> Option<String> {
    images.iter().find_map(|image| {
        image.get("iso_639_1").and_then(|lang| {
            if lang == language || lang == "en" {
                image
                    .get("file_path")
                    .and_then(|file_path| file_path.as_str().map(String::from))
            } else {
                None
            }
        })
    })
}

async fn fetch_images(
    id: &str,
    media_type: MediaType,
    config: &Config,
) -> Result<ImagesResponse, Box<dyn std::error::Error>> {
    let base_url = match media_type {
        MediaType::Movie => "https://api.themoviedb.org/3/movie/",
        MediaType::TvShow => "https://api.themoviedb.org/3/tv/",
    };
    let url = format!("{}{}/images", base_url, id);

    let images_res = create_request(&url, &config).await?;
    let images_data: Value = images_res.json().await?;

    let language = &config.tmdb_language;

    let backdrops = images_data["backdrops"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let backdrop =
        find_image_path(backdrops, &language).or_else(|| find_image_path(backdrops, "en"));

    let logos = images_data["logos"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let logo = find_image_path(logos, &language).or_else(|| find_image_path(logos, "en"));

    let posters = images_data["posters"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let poster = find_image_path(posters, &language).or_else(|| find_image_path(posters, "en"));

    Ok(ImagesResponse {
        backdrop,
        logo,
        poster,
    })
}

pub async fn search_movie(
    config: &Config,
    title: &str,
    date: Option<&str>,
) -> Result<Movie, Box<dyn std::error::Error>> {
    let tmdb_language = &config.tmdb_language;

    let search_url = format!(
        "https://api.themoviedb.org/3/search/movie?query={}&include_adult=false&language={}{}",
        title,
        tmdb_language,
        match date {
            Some(date) => format!("&year={}", date),
            None => "".to_string(),
        }
    );

    let search_res = create_request(&search_url, &config).await?;
    let search_data: Value = search_res.json().await?;

    let results = search_data["results"].as_array().unwrap();
    if results.is_empty() {
        return Err("No results found".into());
    }

    let mut best_match = None;
    let mut lowest_distance = usize::MAX;

    for result in results {
        let result_title = result["title"].as_str().unwrap();
        let distance = levenshtein(title, result_title);
        if distance < lowest_distance {
            lowest_distance = distance;
            best_match = Some(result);
        }
    }

    let movie_id = best_match.unwrap()["id"].as_i64().unwrap();
    let movie_url = format!(
        "https://api.themoviedb.org/3/movie/{}?language={}&append_to_response=release_dates",
        movie_id, tmdb_language
    );

    let movie_res = create_request(&movie_url, &config).await?;
    let movie_data: Value = movie_res.json().await?;

    let collection_id = match movie_data["belongs_to_collection"].as_object() {
        Some(collection) => Some(collection["id"].as_i64().unwrap() as i32),
        None => None::<i32>,
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

    let images_res = fetch_images(&movie_id.to_string(), MediaType::Movie, &config).await?;

    Ok(Movie {
        id: movie_data["id"].as_i64().unwrap() as i32,
        collection_id,
        title: movie_data["title"].as_str().unwrap().to_string(),
        original_title: movie_data["original_title"].as_str().unwrap().to_string(),
        overview: movie_data["overview"].as_str().unwrap().to_string(),
        poster_path: images_res.poster,
        backdrop_path: images_res.backdrop,
        logo_path: images_res.logo,
        release_date: movie_data["release_date"].as_str().unwrap().to_string(),
        runtime: movie_data["runtime"].as_i64().unwrap() as i32,
        genres,
        path: None::<String>,
    })
}

pub async fn search_tv_show(
    config: &Config,
    title: &str,
    date: Option<&str>,
) -> Result<TvShow, Box<dyn std::error::Error>> {
    let tmdb_language = &config.tmdb_language;

    let search_url = format!(
        "https://api.themoviedb.org/3/search/tv?query={}&include_adult=false&language={}{}",
        title,
        tmdb_language,
        match date {
            Some(date) => format!("&year={}", date),
            None => "".to_string(),
        }
    );

    let search_res = create_request(&search_url, &config).await?;
    let search_data: Value = search_res.json().await?;

    let results = search_data["results"].as_array().unwrap();
    if results.is_empty() {
        return Err("No results found".into());
    }

    let mut best_match = None;
    let mut lowest_distance = usize::MAX;

    for result in results {
        let result_title = result["name"].as_str().unwrap();
        let distance = levenshtein(title, result_title);
        if distance < lowest_distance {
            lowest_distance = distance;
            best_match = Some(result);
        }
    }

    let tv_show_id = best_match.unwrap()["id"].as_i64().unwrap();
    let tv_show_url = format!(
        "https://api.themoviedb.org/3/tv/{}?language={}",
        tv_show_id, tmdb_language
    );

    let tv_show_res = create_request(&tv_show_url, &config).await?;
    let tv_show_data: Value = tv_show_res.json().await?;

    let genres = match tv_show_data["genres"].as_array() {
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

    let images_res = fetch_images(&tv_show_id.to_string(), MediaType::TvShow, &config).await?;

    Ok(TvShow {
        id: tv_show_data["id"].as_i64().unwrap() as i32,
        title: tv_show_data["name"].as_str().unwrap().to_string(),
        original_title: tv_show_data["original_name"].as_str().unwrap().to_string(),
        overview: tv_show_data["overview"].as_str().unwrap().to_string(),
        poster_path: images_res.poster,
        backdrop_path: images_res.backdrop,
        logo_path: images_res.logo,
        genres,
        seasons: None::<Vec<Season>>,
        path: None::<String>,
    })
}

pub async fn search_season(
    config: &Config,
    tv_show_id: i32,
    season_number: i32,
) -> Result<Season, Box<dyn std::error::Error>> {
    let tmdb_language = &config.tmdb_language;

    let season_url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}?language={}",
        tv_show_id, season_number, tmdb_language
    );

    let season_res = create_request(&season_url, &config).await?;
    let season_data: Value = season_res.json().await?;

    let posters_url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}/images",
        tv_show_id, season_number
    );

    let posters_res = create_request(&posters_url, &config).await?;
    let posters_data: Value = posters_res.json().await?;

    let posters = posters_data["posters"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let poster_path =
        find_image_path(posters, &tmdb_language).or_else(|| find_image_path(posters, "en"));

    Ok(Season {
        id: season_data["id"].as_i64().unwrap() as i32,
        season_number: season_data["season_number"].as_i64().unwrap() as i32,
        name: season_data["name"].as_str().unwrap().to_string(),
        overview: season_data["overview"].as_str().unwrap().to_string(),
        episodes: None::<Vec<Episode>>,
        poster_path,
        path: None::<String>,
    })
}

pub async fn search_episode(
    config: &Config,
    tv_show_id: i32,
    season_number: i32,
    episode_number: i32,
) -> Result<Episode, Box<dyn std::error::Error>> {
    let tmdb_language = &config.tmdb_language;

    let episode_url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}?language={}",
        tv_show_id, season_number, episode_number, tmdb_language
    );

    let episode_res = create_request(&episode_url, &config).await?;
    let episode_data: Value = episode_res.json().await?;

    let stills_url = format!(
        "https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}/images",
        tv_show_id, season_number, episode_number
    );

    let stills_res = create_request(&stills_url, &config).await?;
    let stills_data: Value = stills_res.json().await?;

    let stills = stills_data["stills"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let still_path =
        find_image_path(stills, &tmdb_language).or_else(|| find_image_path(stills, "en"));

    Ok(Episode {
        id: episode_data["id"].as_i64().unwrap() as i32,
        episode_number: episode_data["episode_number"].as_i64().unwrap() as i32,
        name: episode_data["name"].as_str().unwrap().to_string(),
        overview: episode_data["overview"].as_str().unwrap().to_string(),
        air_date: episode_data["air_date"].as_str().unwrap().to_string(),
        still_path,
        path: None::<String>,
    })
}
