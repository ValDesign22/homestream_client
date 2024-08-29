export interface IConfig {
  http_server: string,
}

export interface IRemoteFolder {
  id: number,
  name: string,
  path: string,
  children?: IRemoteFolder[],
}

export interface IFolder {
  id: number,
  name: string,
  path: string,
  media_type: EMediaType,
}

export enum EMediaType {
  Movie,
  ITvShow,
}

export interface IMovie {
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
  genres: IGenre[],
  path: string,
}

export interface ITvShow {
  id: number,
  title: string,
  original_title: string,
  overview: string,
  poster_path: string,
  backdrop_path: string,
  logo_path: string,
  genres: IGenre[],
  seasons: ISeason[],
  path: string,
}

export interface ISeason {
  id: number,
  season_number: number,
  name: string,
  overview: string,
  episodes: IEpisode[],
  poster_path?: string,
  path: string,
}

export interface IEpisode {
  id: number,
  episode_number: number,
  title: string,
  overview: string,
  air_date: string,
  still_path: string,
  runtime: number,
  path: string,
}

export interface IGenre {
  id: number,
  name: string,
  items: (IMovie | ITvShow)[],
}

export interface IAudioTrack {
  index: number,
  codec_name: string,
  codec_type: string,
  channel_layout: string,
  language: string,
  handler_name: string,
  enabled: boolean,
}

export interface ISubtitleTrack {
  index: number,
  codec_name: string,
  codec_type: string,
  language: string,
  handler_name: string,
  enabled: boolean,
}

export interface IProfile {
  id: number,
  name: string,
  password?: string,
  role: ERole,
  history: IHistory[],
  watchlist: (IMovie | ITvShow)[],
  favorites: (IMovie | ITvShow)[],
}

export enum ERole {
  Admin,
  User,
}

export interface IHistory {
  id: number,
  date: string,
  title: string,
  media_type: EMediaType,
  watched: boolean,
  progress: number,
}
