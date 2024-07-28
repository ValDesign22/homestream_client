use serde::{Deserialize, Serialize};

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
pub struct Serie {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Season {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Episode {}

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
