use std::collections::VecDeque;

use suppaftp::FtpStream;

use super::{
    ftp::{load_movies, load_tv_shows},
    tmdb::{search_episode, search_movie, search_season, search_tv_show},
    types::{Config, Episode, Folder, Movie, Season, TvShow},
};

pub async fn explore_movies_folder(
    stream: &mut FtpStream,
    config: &Config,
    folder: &Folder,
) -> Vec<Movie> {
    let mut stack = VecDeque::new();
    stack.push_back(folder.path.clone());

    let mut movies: Vec<Movie> = Vec::new();
    let existing_movies = load_movies(stream, &config, folder.id.to_string());

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

            let existing_movie = existing_movies
                .iter()
                .find(|movie| movie.path.as_ref().unwrap() == &format!("{}/{}", path, file));
            if existing_movie.is_some() {
                movies.push(existing_movie.unwrap().clone());
                continue;
            }

            let file_name = file.split('.').next().unwrap();
            let date = match file_name.split(' ').last() {
                Some(date)
                    if date.len() == 4
                        && date.len() != file_name.len()
                        && date.chars().all(char::is_numeric) =>
                {
                    Some(date)
                }
                _ => None,
            };
            let title = match date {
                Some(date) => file_name
                    .split(' ')
                    .take_while(|&x| x != date)
                    .collect::<Vec<&str>>()
                    .join(" "),
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

pub async fn explore_tv_shows_folder(
    stream: &mut FtpStream,
    config: &Config,
    folder: &Folder,
) -> Vec<TvShow> {let mut stack = VecDeque::new();
    stack.push_back(folder.path.clone());

    let mut tv_shows: Vec<TvShow> = Vec::new();

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
                Some(date)
                    if date.len() == 4
                        && date.len() != file.len()
                        && date.chars().all(char::is_numeric) =>
                {
                    Some(date)
                }
                _ => None,
            };
            let title = match date {
                Some(date) => file
                    .split(' ')
                    .take_while(|&x| x != date)
                    .collect::<Vec<&str>>()
                    .join(" "),
                None => file.to_string(),
            };

            let tv_show = match search_tv_show(&config, &title, date).await {
                Ok(tv_show) => TvShow {
                    path: Some(format!("{}/{}", path, file)),
                    ..tv_show
                },
                Err(e) => {
                    eprintln!("Error searching tv_show: {}", e);
                    continue;
                }
            };

            let seasons = explore_seasons_folder(stream, &config, &tv_show, None, folder.id.to_string()).await;
            tv_shows.push(TvShow {
                seasons: Some(seasons),
                ..tv_show
            });
        }
    }

    tv_shows
}

pub async fn explore_seasons_folder(
    stream: &mut FtpStream,
    config: &Config,
    tv_show: &TvShow,
    folder: Option<&str>,
    store: String,
) -> Vec<Season> {
    let initial_path = folder
        .map(|folder| format!("{}/{}", tv_show.path.as_deref().unwrap_or_default(), folder))
        .unwrap_or_else(|| tv_show.path.as_deref().unwrap_or_default().to_string());

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
                Some(season_number)
                    if season_number.len() != file.len()
                        && season_number.chars().all(char::is_numeric) =>
                {
                    season_number
                }
                _ => continue,
            };

            let season =
                match search_season(&config, tv_show.id, season_number.parse().unwrap()).await {
                    Ok(season) => Season {
                        path: Some(format!("{}/{}", path, file)),
                        ..season
                    },
                    Err(e) => {
                        eprintln!("Error searching season: {}", e);
                        continue;
                    }
                };

            let episodes = explore_episodes(stream, &config, tv_show, &season, None, store.clone()).await;
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
    tv_show: &TvShow,
    season: &Season,
    folder: Option<&str>,
    store: String,
) -> Vec<Episode> {
    let initial_path = folder
        .map(|folder| format!("{}/{}", season.path.as_deref().unwrap_or_default(), folder))
        .unwrap_or_else(|| season.path.as_deref().unwrap_or_default().to_string());

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut episodes: Vec<Episode> = Vec::new();
    let existing_tv_shows = load_tv_shows(stream, &config, store);

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

            if let Some(existing_tv_show) = existing_tv_show
                .iter()
                .find(|e_tv_show| e_tv_show.path.as_deref() == tv_show.path.as_deref())
            {
                if let Some(existing_season) = existing_tv_show.seasons.as_ref().and_then(|seasons| {
                    seasons
                        .iter()
                        .find(|e_season| e_season.path.as_deref() == season.path.as_deref())
                }) {
                    if let Some(existing_episode) =
                        existing_season.episodes.as_ref().and_then(|episodes| {
                            episodes
                                .iter()
                                .find(|episode| episode.path.as_deref() == Some(&file_path))
                        })
                    {
                        episodes.push(existing_episode.clone());
                        continue;
                    }
                }
            }

            let episode_number = match file
                .split('.')
                .next()
                .and_then(|f| f.split_whitespace().last())
                .and_then(|s| s.split('E').last())
                .filter(|&n| n.chars().all(char::is_numeric))
            {
                Some(episode_number) => episode_number,
                None => continue,
            };

            match search_episode(
                &config,
                tv_show.id,
                season.season_number,
                episode_number.parse().unwrap(),
            )
            .await
            {
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
