use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub http_server: String,
    pub ws_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub media_type: MediaType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MediaType {
    Movie,
    TvShow,
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
pub struct TvShow {
    pub id: i32,
    pub title: String,
    pub original_title: String,
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
    pub title: String,
    pub overview: String,
    pub air_date: String,
    pub still_path: Option<String>,
    pub runtime: i32,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: i32,
    pub name: String,
    pub password: Option<String>,
    pub role: Role,
    pub history: Option<Vec<History>>,
    pub watchlist: Option<Vec<Folder>>,
    pub favorites: Option<Vec<Folder>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct History {
    pub id: i32,
    pub date: String,
    pub title: String,
    pub media_type: MediaType,
    pub watched: bool,
    pub progress: i32,
}
