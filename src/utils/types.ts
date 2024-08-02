export interface Config {
  ftp_host: string,
  ftp_port: number,
  ftp_user: string,
  ftp_password: string,
  folders: Folder[],
  app_storage_path: string,
  tmdb_api_key: string,
  tmdb_language: string,
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
