use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Debug)]
pub enum ConfigError {
    FetchError(gloo_net::Error),
    HttpError(u16, String),
    ParsingError(gloo_net::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConfigError::FetchError(ref e) => write!(f, "Fetching config error: {}", e),
            ConfigError::HttpError(ref e, ref s) => write!(f, "HTTP error {}: {}", e, s),
            ConfigError::ParsingError(ref e) => write!(f, "Parsing config error: {}", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AppConfig {
    pub public_key: String,
    pub title: String,
}

impl AppConfig {
    pub async fn load() -> Result<AppConfig, ConfigError> {
        let response = Request::get("/config.json")   // relative path — served from same folder
        .send()
        .await
        .map_err(|e| ConfigError::FetchError(e))?;

    if !response.ok() {
        let status = response.status();
        let error_body = response.text().await
            .unwrap_or_else(|_| String::from("<no error body>"));

        return Err(ConfigError::HttpError(status, error_body));
    }

    response.json::<AppConfig>()
        .await
        .map_err(|e| ConfigError::ParsingError(e))
    }
}