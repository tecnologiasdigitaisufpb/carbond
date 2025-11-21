#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct WattTimeResponse {
    ba: String,
    freq: String,
    pub moer: String,
    point_time: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct WattTimeLoginResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct WattTimeError {
    pub error: String,
    pub message: String,
}
