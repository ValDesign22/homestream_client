use crate::utils::types::Movie;
use std::io::Write;
use suppaftp::FtpStream;

use super::types::{Config, TvShow};

pub fn create_stream(config: &Config) -> FtpStream {
    let ftp_url = format!("{}:{}", config.ftp_host, config.ftp_port,);

    let mut stream = FtpStream::connect(ftp_url).unwrap();
    let _ = stream
        .login(&config.ftp_user, &config.ftp_password)
        .unwrap();

    stream
}

pub fn load_movies(stream: &mut FtpStream, config: &Config, store: String) -> Vec<Movie> {
    if let Err(e) = stream.cwd(&config.app_storage_path) {
        eprintln!("Error changing directory: {}", e);
        return vec![];
    }

    match stream.retr_as_buffer(format!("{}_store.json", store).as_str()) {
        Ok(buffer) => {
            let content = String::from_utf8(buffer.into_inner()).unwrap();
            let json_data: Vec<Movie> = match serde_json::from_str(&content.trim()) {
                Ok(json_data) => json_data,
                Err(e) => {
                    eprintln!("Error parsing {}_store.json: {}", store, e);
                    vec![]
                }
            };
            json_data
        }
        Err(e) => {
            eprintln!("Error reading {}_store.json: {}", store, e);
            let _ = save_file(stream, &config, store, "[]");
            vec![]
        }
    }
}

pub fn load_tv_shows(stream: &mut FtpStream, config: &Config, store: String) -> Vec<TvShow> {
    if let Err(e) = stream.cwd(&config.app_storage_path) {
        eprintln!("Error changing directory: {}", e);
        return vec![];
    }

    match stream.retr_as_buffer(format!("{}_store.json", store).as_str()) {
        Ok(buffer) => {
            let content = String::from_utf8(buffer.into_inner()).unwrap();
            let json_data: Vec<TvShow> = match serde_json::from_str(&content.trim()) {
                Ok(json_data) => json_data,
                Err(e) => {
                    eprintln!("Error parsing {}_store.json: {}", store, e);
                    vec![]
                }
            };
            json_data
        }
        Err(e) => {
            eprintln!("Error reading {}_store.json: {}", store, e);
            let _ = save_file(stream, &config, store, "[]");
            vec![]
        }
    }
}

pub fn save_file(
    stream: &mut FtpStream,
    config: &Config,
    store: String,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = stream.cwd(&config.app_storage_path)?;

    let mut put_stream = stream.put_with_stream(format!("{}_store.json", store).as_str())?;
    let _ = put_stream.write_all(content.as_bytes())?;
    let _ = stream.finalize_put_stream(put_stream)?;

    Ok(())
}
