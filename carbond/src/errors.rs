use carbond_lib::metrics::metric::MetricError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarbondError {
    #[error("API request was not successful.")]
    Api(#[from] APIError),
    #[error("Config is invalid.")]
    Config(#[from] ConfigError),
    #[error("Error handling metric.")]
    Metric(#[from] MetricError),
    #[error("IO: {msg}: {source}")]
    Io {
        msg: String,
        #[source]
        source: io::Error,
    },
}

#[derive(Error, Debug, PartialEq)]
pub enum APIError {
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("Unauthorized.")]
    Authentication,
    #[error("Invalid region {0}.")]
    InvalidRegion(String),
    #[error("Could not deserialize response body {0}.")]
    Deserialze(String),
    #[error("Something went wrong while getting response from {0}.")]
    Unhandled(String),
}

#[derive(Error, Debug, PartialEq)]
pub enum ConfigError {
    #[error("Could not parse config.")]
    ParseConfig(#[from] toml::de::Error),
    #[error("\"{0}\" is not a valid interval.")]
    ParseInterval(String),
    #[error("Could not find all required options: {0}.")]
    ConfigMissing(String),
}
