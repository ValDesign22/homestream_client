import { invoke } from "@tauri-apps/api/core";
import { IConfig, IEpisode, IMovie, ITvShow } from "./types";

async function fetchStores(): Promise<Record<string, IMovie[] | ITvShow[]>> {
  const config = await invoke<IConfig | null>("get_config");
  if (config) {
    const response = await fetch(config.http_server + "/stores", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (response.ok) return await response.json();
  }

  return {};
}

function isTvShowStore(store: IMovie[] | ITvShow[]): store is ITvShow[] {
  return store.length > 0 && "seasons" in store[0];
}

export async function getTvShowFromEpisode(episode_id: number): Promise<ITvShow | null> {
  const stores = await fetchStores();
  if (!stores) return null;

  for (const store of Object.values(stores)) {
    if (isTvShowStore(store)) {
      for (const item of store) {
        for (const season of item.seasons) {
          if (season.episodes.some((episode) => episode.id === episode_id)) return item;
        }
      }
    }
  }

  return null;
}

export async function getMovieFromId(movie_id: number): Promise<IMovie | null> {
  const stores = await fetchStores();
  if (!stores) return null;

  for (const store of Object.values(stores)) {
    if (!isTvShowStore(store)) {
      const movie = store.find((item) => item.id === movie_id);
      if (movie) return movie;
    }
  }

  return null;
}

export async function getTvShowEpisodes(tvshow_id: number): Promise<IEpisode[]> {
  const stores = await fetchStores();
  if (!stores) return [];

  const episodes: IEpisode[] = [];

  for (const store of Object.values(stores)) {
    if (isTvShowStore(store)) {
      const tvshow = store.find((item) => item.id === tvshow_id);
      if (tvshow) {
        for (const season of tvshow.seasons) {
          episodes.push(...season.episodes);
        }
      }
    }
  }

  return episodes;
}