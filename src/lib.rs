#![allow(dead_code)]

use chrono::Duration;
use reqwest::Client;

pub use error::Error;

pub mod data_types;
mod error;
mod util;

pub struct API {
    refresh_interval: Duration,
    base_url: String,
    library: util::XMLLibrary,
    client: Client,
}
