use std::collections::VecDeque;

use suppaftp::FtpStream;

use super::{ftp::load_file, tmdb::search_movie, types::{MediaType, Movie, Serie}};

pub async fn explore_movies_folder(stream: &mut FtpStream, media_type: MediaType, folder: Option<&str>) -> Vec<Movie> {
    let default_path = format!(
        "{}{}",
        std::env::var("FTP_PATH").expect("FTP_PATH not set"),
        match media_type {
            MediaType::Anime => std::env::var("ANIMES_FOLDER").expect("ANIMES_FOLDER not set"),
            MediaType::Movie => std::env::var("MOVIES_FOLDER").expect("MOVIES_FOLDER not set"),
            MediaType::Serie => unreachable!(),
        },
    );
    let initial_path = match folder {
        Some(folder) => format!("{}/{}", default_path, folder),
        None => default_path,
    };

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut movies: Vec<Movie> = Vec::new();
    let existing_movies = load_file(stream, match media_type {
        MediaType::Anime => "animes",
        MediaType::Movie => "movies",
        MediaType::Serie => unreachable!(),
    });

    while let Some(path) = stack.pop_back() {
        if let Err(e) = stream.cwd(&path) {
            println!("Error changing directory: {}", e);
            continue;
        }

        let nlst = match stream.nlst(Some(&path)) {
            Ok(nlst) => nlst,
            Err(e) => {
                println!("Error listing directory: {}", e);
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
                Some(date) if date.len() == 4 && date.chars().all(char::is_numeric) => Some(date),
                _ => None,
            };
            let title = match date {
                Some(date) => file_name.split(' ').take_while(|&x| x != date).collect::<Vec<&str>>().join(" "),
                None => file_name.to_string(),
            };

            match search_movie(&title, date).await {
                Ok(movie_data) => movies.push(Movie {
                    path: Some(format!("{}/{}", path, file)),
                    ..movie_data
                }),
                Err(e) => println!("Error searching movie: {}", e),
            }
        }
    }

    movies
}

pub async fn explore_series_folder(stream: &mut FtpStream, folder: Option<&str>) -> Vec<Serie> {
    let default_path = format!(
        "{}{}",
        std::env::var("FTP_PATH").expect("FTP_PATH not set"),
        std::env::var("SERIES_FOLDER").expect("SERIES_FOLDER not set"),
    );
    let initial_path = match folder {
        Some(folder) => format!("{}/{}", default_path, folder),
        None => default_path,
    };

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut series: Vec<Serie> = Vec::new();
    let existing_series = load_file(stream, "series");

    while let Some(path) = stack.pop_back() {};

    series
}
