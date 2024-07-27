use suppaftp::FtpStream;

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
