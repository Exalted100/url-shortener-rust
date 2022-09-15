use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlRequest {
    pub url: String,
    pub ttl: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlResponse {
    pub url: String,
    pub shortenedUrl: String,
}