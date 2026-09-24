use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Http(reqwest::Error),
    Status(reqwest::StatusCode),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Http(e) => write!(f, "Request failed: {e}"),
            AppError::Status(code) => {
                if code.as_u16() == 429 {
                    write!(f, "nhentai is rate limiting us (HTTP 429). Please wait a moment and retry.")
                } else {
                    write!(f, "nhentai returned HTTP {}", code.as_u16())
                }
            }
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
        }
    }
}