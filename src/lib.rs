#![allow(dead_code)]
use chrono::Duration;

pub use error::Error;

mod xml_document;
mod xml_library;
mod error;

pub struct API {
    refresh_interval: Duration,
}
