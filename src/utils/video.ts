import { invoke } from "@tauri-apps/api/core";
import { IConfig, IMovie, ITvShow } from "./types";

export async function getTvShowFromEpisode(episode_id: number): Promise<ITvShow | null> {
  const config = await invoke<IConfig | null>("get_config");
  if (config) {
    const response = await fetch(config.http_server + "/stores", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (!response.ok) return null;

    const stores: Record<string, IMovie[] | ITvShow[]> = await response.json();
    for (const store of Object.values(stores)) {
      if (store.length === 0) continue;
      if ("seasons" in store[0]) {
        for (const item of store) {
          if ('seasons' in item) {
            for (const season of item.seasons) {
              for (const episode of season.episodes) {
                if (episode.id === episode_id) return item;
              }
            }
          }
        }
      }
    }
  }

  return null;
}

export async function getMovieFromId(movie_id: number): Promise<IMovie | null> {
  const config = await invoke<IConfig | null>("get_config");
  if (config) {
    const response = await fetch(config.http_server + "/stores", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (!response.ok) return null;

    const stores: Record<string, IMovie[] | ITvShow[]> = await response.json();
    for (const store of Object.values(stores)) {
      if (store.length === 0) continue;
      if (!("seasons" in store[0])) {
        for (const item of (store as IMovie[])) {
          if (item.id === movie_id) return item;
        }
      }
    }
  }

  return null;
}