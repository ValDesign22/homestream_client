export interface Config {
  http_server: string,
}

export interface RemoteFolder {
  id: number,
  name: string,
  path: string,
  children?: RemoteFolder[],
}

export interface Folder {
  id: number,
  name: string,
  path: string,
  media_type: MediaType,
}

export enum MediaType {
  Movie,
  TvShow,
}

export interface Movie {
  id: number,
  collection_id: number,
  title: string,
  original_title: string,
  overview: string,
  poster_path: string,
  backdrop_path: string,
  logo_path: string,
  release_date: string,
  runtime: number,
  genres: Genre[],
  path: string,
}

export interface TvShow {
  id: number,
  title: string,
  original_title: string,
  overview: string,
  poster_path: string,
  backdrop_path: string,
  logo_path: string,
  genres: Genre[],
  seasons: Season[],
  path: string,
}

export interface Season {
  id: number,
  season_number: number,
  name: string,
  overview: string,
  episodes: Episode[],
  poster_path?: string,
  path: string,
}

export interface Episode {
  id: number,
  episode_number: number,
  name: string,
  overview: string,
  air_date: string,
  still_path: string,
  path: string,
}

export interface Genre {
  id: number,
  name: string,
}
