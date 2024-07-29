use std::io::Write;
use crate::utils::types::Movie;
use suppaftp::FtpStream;

use super::types::Serie;

pub fn create_stream() -> FtpStream {
    let ftp_url = format!(
        "{}:{}",
        std::env::var("FTP_HOST").expect("FTP_HOST not set"),
        std::env::var("FTP_PORT").expect("FTP_PORT not set"),
    );
    let ftp_user = std::env::var("FTP_USER").expect("FTP_USER not set");
    let ftp_password = std::env::var("FTP_PASSWORD").expect("FTP_PASSWORD not set");

    let mut stream = FtpStream::connect(ftp_url).unwrap();
    let _ = stream.login(&ftp_user, &ftp_password).unwrap();

    stream
}

pub fn load_movies(stream: &mut FtpStream, store: &str) -> Vec<Movie> {
    let app_storage_path = std::env::var("APP_STORAGE_PATH").expect("APP_STORAGE_PATH not set");

    if let Err(e) = stream.cwd(&app_storage_path) {
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
            let _ = save_file(stream, store, "[]");
            vec![]
        }
    }
}

pub fn load_series(stream: &mut FtpStream) -> Vec<Serie> {
    let app_storage_path = std::env::var("APP_STORAGE_PATH").expect("APP_STORAGE_PATH not set");

    if let Err(e) = stream.cwd(&app_storage_path) {
        eprintln!("Error changing directory: {}", e);
        return vec![];
    }

    match stream.retr_as_buffer("series_store.json") {
        Ok(buffer) => {
            let content = String::from_utf8(buffer.into_inner()).unwrap();
            let json_data: Vec<Serie> = match serde_json::from_str(&content.trim()) {
                Ok(json_data) => json_data,
                Err(e) => {
                    eprintln!("Error parsing series_store.json: {}", e);
                    vec![]
                }
            };
            json_data
        }
        Err(e) => {
            eprintln!("Error reading series_store.json: {}", e);
            let _ = save_file(stream, "series", "[]");
            vec![]
        }
    }
}

pub fn save_file(stream: &mut FtpStream, store: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let app_storage_path = std::env::var("APP_STORAGE_PATH").expect("APP_STORAGE_PATH not set");

    let _ = stream.cwd(&app_storage_path)?;

    let mut put_stream = stream.put_with_stream(format!("{}_store.json", store).as_str())?;
    let _ = put_stream.write_all(content.as_bytes())?;
    let _ = stream.finalize_put_stream(put_stream)?;

    Ok(())
}
