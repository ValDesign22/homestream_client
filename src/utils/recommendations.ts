import { invoke } from "@tauri-apps/api/core";
import { Config, Movie, TvShow } from "./types";
import { fetch } from "@tauri-apps/plugin-http";

export default async function getRecommendations(item: Movie | TvShow): Promise<(Movie | TvShow)[]> {
  const config = await invoke<Config | null>("get_config");
  if (config) {
    const response = await fetch(config.http_server + "/stores", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
    });
    if (!response.ok) return [];

    const stores: Record<string, Movie[] | TvShow[]> = await response.json();
    const groupedStores = Object.values(stores).flat();

    const genreIds = new Set(item.genres.map(genre => genre.id));
    const genreCounts: Record<number, number> = {};

    // Count how many genres each item shares with the input item
    for (const storeItem of groupedStores) {
      if (storeItem.id === item.id) continue; // Skip the same item

      const commonGenres = storeItem.genres.filter(genre => genreIds.has(genre.id));
      if (commonGenres.length > 0) {
        genreCounts[storeItem.id] = commonGenres.length;
      }
    }

    // Sort the items by the number of common genres in descending order
    const recommendations = groupedStores
      .filter(storeItem => genreCounts[storeItem.id] > 0)
      .sort((a, b) => genreCounts[b.id] - genreCounts[a.id])
      .slice(0, 10);

    return recommendations;
  }

  return [];
}
