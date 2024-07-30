use std::collections::VecDeque;

use suppaftp::FtpStream;

use super::{
    ftp::{load_movies, load_series},
    tmdb::{search_episode, search_movie, search_season, search_serie},
    types::{Config, Episode, MediaType, Movie, Season, Serie}
};

pub async fn explore_movies_folder(stream: &mut FtpStream, config: &Config, media_type: MediaType, folder: Option<&str>) -> Vec<Movie> {
    let default_path = format!(
        "{}{}",
        &config.ftp_path,
        match media_type {
            MediaType::Anime => &config.animes_folder,
            MediaType::Movie => &config.movies_folder,
            MediaType::Serie => unreachable!(),
        },
    );
    let initial_path = folder
        .map(|folder| format!("{}/{}", default_path, folder))
        .unwrap_or_else(|| default_path.clone());

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut movies: Vec<Movie> = Vec::new();
    let existing_movies = load_movies(stream, &config, match media_type {
        MediaType::Anime => "animes",
        MediaType::Movie => "movies",
        MediaType::Serie => unreachable!(),
    });

    while let Some(path) = stack.pop_back() {
        if let Err(e) = stream.cwd(&path) {
            eprintln!("Error changing directory: {}", e);
            continue;
        }

        let nlst = match stream.nlst(Some(&path)) {
            Ok(nlst) => nlst,
            Err(e) => {
                eprintln!("Error listing directory: {}", e);
                continue;
            }
        };

        for file in nlst {
            if file == "." || file == ".." {
                continue;
            }

            if !file.contains('.') {
                stack.push_back(format!("{}/{}", path, file));
                continue;
            }

            let existing_movie = existing_movies.iter().find(|movie| movie.path.as_ref().unwrap() == &format!("{}/{}", path, file));
            if existing_movie.is_some() {
                movies.push(existing_movie.unwrap().clone());
                continue;
            }

            let file_name = file.split('.').next().unwrap();
            let date = match file_name.split(' ').last() {
                Some(date) if date.len() == 4 && date.len() != file_name.len() && date.chars().all(char::is_numeric) => Some(date),
                _ => None,
            };
            let title = match date {
                Some(date) => file_name.split(' ').take_while(|&x| x != date).collect::<Vec<&str>>().join(" "),
                None => file_name.to_string(),
            };

            match search_movie(&config, &title, date).await {
                Ok(movie_data) => movies.push(Movie {
                    path: Some(format!("{}/{}", path, file)),
                    ..movie_data
                }),
                Err(e) => eprintln!("Error searching movie: {}", e),
            }
        }
    }

    movies
}

pub async fn explore_series_folder(stream: &mut FtpStream, config: &Config, folder: Option<&str>) -> Vec<Serie> {
    let default_path = format!(
        "{}{}",
        &config.ftp_path,
        &config.series_folder,
    );
    let initial_path = folder
        .map(|folder| format!("{}/{}", default_path, folder))
        .unwrap_or_else(|| default_path.clone());

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut series: Vec<Serie> = Vec::new();

    while let Some(path) = stack.pop_back() {
        if let Err(e) = stream.cwd(&path) {
            eprintln!("Error changing directory: {}", e);
            continue;
        }

        let nlst = match stream.nlst(Some(&path)) {
            Ok(nlst) => nlst,
            Err(e) => {
                eprintln!("Error listing directory: {}", e);
                continue;
            }
        };

        for file in nlst {
            if file == "." || file == ".." {
                continue;
            }

            if file.contains('.') {
                continue;
            }

            let date = match file.split(' ').last() {
                Some(date) if date.len() == 4 && date.len() != file.len() && date.chars().all(char::is_numeric) => Some(date),
                _ => None,
            };
            let title = match date {
                Some(date) => file.split(' ').take_while(|&x| x != date).collect::<Vec<&str>>().join(" "),
                None => file.to_string(),
            };

            let serie = match search_serie(&config, &title, date).await {
                Ok(serie) => Serie {
                    path: Some(format!("{}/{}", path, file)),
                    ..serie
                },
                Err(e) => {
                    eprintln!("Error searching serie: {}", e);
                    continue;
                },
            };

            let seasons = explore_seasons_folder(stream, &config, &serie, None).await;
            series.push(Serie {
                seasons: Some(seasons),
                ..serie
            });
        }
    };

    series
}

pub async fn explore_seasons_folder(stream: &mut FtpStream, config: &Config, serie: &Serie, folder: Option<&str>) -> Vec<Season> {
    let initial_path = folder
        .map(|folder| format!("{}/{}", serie.path.as_deref().unwrap_or_default(), folder))
        .unwrap_or_else(|| serie.path.as_deref().unwrap_or_default().to_string());

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut seasons: Vec<Season> = Vec::new();

    while let Some(path) = stack.pop_back() {
        if let Err(e) = stream.cwd(&path) {
            eprintln!("Error changing directory: {}", e);
            continue;
        }

        let nlst = match stream.nlst(Some(&path)) {
            Ok(nlst) => nlst,
            Err(e) => {
                eprintln!("Error listing directory: {}", e);
                continue;
            }
        };

        for file in nlst {
            if file == "." || file == ".." {
                continue;
            }

            if file.contains('.') {
                continue;
            }

            let season_number = match file.split(' ').last() {
                Some(season_number) if season_number.len() != file.len() && season_number.chars().all(char::is_numeric) => season_number,
                _ => continue,
            };

            let season = match search_season(&config, serie.id, season_number.parse().unwrap()).await {
                Ok(season) => Season {
                    path: Some(format!("{}/{}", path, file)),
                    ..season
                },
                Err(e) => {
                    eprintln!("Error searching season: {}", e);
                    continue;
                },
            };

            let episodes = explore_episodes(stream, &config, serie, &season, None).await;
            seasons.push(Season {
                episodes: Some(episodes),
                ..season
            });
        }
    }

    seasons
}

pub async fn explore_episodes(
    stream: &mut FtpStream,
    config: &Config,
    serie: &Serie,
    season: &Season,
    folder: Option<&str>
) -> Vec<Episode> {
    let initial_path = folder
        .map(|folder| format!("{}/{}", season.path.as_deref().unwrap_or_default(), folder))
        .unwrap_or_else(|| season.path.as_deref().unwrap_or_default().to_string());

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut episodes: Vec<Episode> = Vec::new();
    let existing_series = load_series(stream, &config);

    while let Some(path) = stack.pop_back() {
        if let Err(e) = stream.cwd(&path) {
            eprintln!("Error changing directory: {}", e);
            continue;
        }

        let nlst = match stream.nlst(Some(&path)) {
            Ok(nlst) => nlst,
            Err(e) => {
                eprintln!("Error listing directory: {}", e);
                continue;
            }
        };

        for file in nlst {
            if file == "." || file == ".." {
                continue;
            }

            let file_path = format!("{}/{}", path, file);

            if !file.contains('.') {
                stack.push_back(file_path);
                continue;
            }

            if let Some(existing_serie) = existing_series.iter().find(|e_serie| e_serie.path.as_deref() == serie.path.as_deref()) {
                if let Some(existing_season) = existing_serie.seasons.as_ref().and_then(|seasons| {
                    seasons.iter().find(|e_season| e_season.path.as_deref() == season.path.as_deref())
                }) {
                    if let Some(existing_episode) = existing_season.episodes.as_ref().and_then(|episodes| {
                        episodes.iter().find(|episode| episode.path.as_deref() == Some(&file_path))
                    }) {
                        episodes.push(existing_episode.clone());
                        continue;
                    }
                }
            }

            let episode_number = match file.split('.').next().and_then(|f| f.split_whitespace().last()).and_then(|s| s.split('E').last()).filter(|&n| n.chars().all(char::is_numeric)) {
                Some(episode_number) => episode_number,
                None => continue,
            };

            match search_episode(&config, serie.id, season.season_number, episode_number.parse().unwrap()).await {
                Ok(episode) => episodes.push(Episode {
                    path: Some(file_path),
                    ..episode
                }),
                Err(e) => eprintln!("Error searching episode: {}", e),
            }
        }
    }

    episodes
}
