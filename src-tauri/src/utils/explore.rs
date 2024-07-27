use std::{collections::VecDeque, io::Write};

use suppaftp::FtpStream;

use super::{tmdb::search_movie, types::Movie};

pub fn load_file(stream: &mut FtpStream, store: &str) -> Vec<Movie> {
    let app_storage_path = std::env::var("APP_STORAGE_PATH").expect("APP_STORAGE_PATH not set");

    if let Err(e) = stream.cwd(&app_storage_path) {
        println!("Error changing directory: {}", e);
        return vec![];
    }

    match stream.retr_as_buffer(format!("{}_store.json", store).as_str()) {
        Ok(buffer) => {
            let content = String::from_utf8(buffer.into_inner()).unwrap();
            let json_data: Vec<Movie> = match serde_json::from_str(&content.trim()) {
                Ok(json_data) => json_data,
                Err(e) => {
                    println!("Error parsing movies_store.json: {}", e);
                    vec![]
                }
            };
            json_data
        }
        Err(e) => {
            println!("Error retrieving movies_store.json: {}", e);
            vec![]
        }
    }
}

pub fn save_file(stream: &mut FtpStream, store: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app_storage_path = std::env::var("APP_STORAGE_PATH").expect("APP_STORAGE_PATH not set");

    if let Err(e) = stream.cwd(&app_storage_path) {
        return Err(e.into());
    }

    let mut put_stream = stream.put_with_stream(format!("{}_store.json", store).as_str())?;
    if let Err(e) = put_stream.write_all(content.as_bytes()) {
        return Err(e.into());
    }
    if let Err(e) = stream.finalize_put_stream(put_stream) {
        return Err(e.into());
    }

    Ok(())
}

pub async fn explore_movies_folder(stream: &mut FtpStream, folder: Option<&str>) -> Vec<Movie> {
    let default_path = format!(
        "{}{}",
        std::env::var("FTP_PATH").expect("FTP_PATH not set"),
        std::env::var("MOVIES_FOLDER").expect("MOVIES_FOLDER not set"),
    );
    let initial_path = match folder {
        Some(folder) => format!("{}/{}", default_path, folder),
        None => default_path,
    };

    let mut stack = VecDeque::new();
    stack.push_back(initial_path);

    let mut movies: Vec<Movie> = Vec::new();
    let existing_movies = load_file(stream, "movies");

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

            if existing_movies.iter().any(|movie| movie.path.as_ref().unwrap() == &format!("{}/{}", path, file)) {
                println!("Movie already exists: {}", file);
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
