import { invoke } from "@tauri-apps/api/core";
import { IConfig, IMovie, ITvShow } from "./types";
import { fetch } from "@tauri-apps/plugin-http";

async function fetchStores(): Promise<(IMovie | ITvShow)[]> {
  const config = await invoke<IConfig | null>("get_config");
  if (!config) return [];

  try {
    const response = await fetch(`${config.http_server}/stores`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (!response.ok) return [];
    const stores: Record<string, IMovie[] | ITvShow[]> = await response.json();
    return Object.values(stores).flat();
  } catch (error) {
    console.error(error);
    return [];
  }
}

export default async function getRecommendations(item: IMovie | ITvShow): Promise<(IMovie | ITvShow)[]> {
  const allItems = await fetchStores();
  if (allItems.length === 0) return [];

  const genreIds = new Set(item.genres.map(genre => genre.id));

  const recommendations = allItems
    .filter(storeItem => storeItem.id !== item.id)
    .map(storeItem => ({
      item: storeItem,
      similarity: storeItem.genres.filter(genre => genreIds.has(genre.id)).length
    }))
    .filter(({ similarity }) => similarity > 0)
    .sort((a, b) => b.similarity - a.similarity)
    .slice(0, 10)
    .map(({ item }) => item);

  return recommendations;
}
