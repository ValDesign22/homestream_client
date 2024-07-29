use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
  pub ftp_host: String,
  pub ftp_port: String,
  pub ftp_user: String,
  pub ftp_password: String,
  pub ftp_path: String,
  pub movies_folder: String,
  pub series_folder: String,
  pub animes_folder: String,
  pub app_storage_path: String,
  pub tmdb_api_key: String,
  pub tmdb_language: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
  pub id: i32,
  pub collection_id: Option<i32>,
  pub title: String,
  pub original_title: String,
  pub overview: String,
  pub poster_path: Option<String>,
  pub backdrop_path: Option<String>,
  pub logo_path: Option<String>,
  pub release_date: String,
  pub runtime: i32,
  pub genres: Option<Vec<Genre>>,
  pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Serie {
  pub id: i32,
  pub title: String,
  pub overview: String,
  pub poster_path: Option<String>,
  pub backdrop_path: Option<String>,
  pub logo_path: Option<String>,
  pub genres: Option<Vec<Genre>>,
  pub seasons: Option<Vec<Season>>,
  pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Season {
  pub id: i32,
  pub season_number: i32,
  pub name: String,
  pub overview: String,
  pub episodes: Option<Vec<Episode>>,
  pub poster_path: Option<String>,
  pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
  pub id: i32,
  pub episode_number: i32,
  pub name: String,
  pub overview: String,
  pub air_date: String,
  pub still_path: Option<String>,
  pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genre {
  pub id: i32,
  pub name: String,
}

pub struct ImagesResponse {
  pub backdrop: Option<String>,
  pub logo: Option<String>,
  pub poster: Option<String>,
}

pub enum MediaType {
  Anime,
  Movie,
  Serie,
}
