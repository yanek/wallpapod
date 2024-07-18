use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Struct representing a response from the Astronomy Picture of the Day (APOD) API
#[derive(Serialize, Deserialize, Debug)]
pub struct ApodResponse {
    pub copyright: String,
    pub date: NaiveDate,
    pub explanation: String,
    pub hdurl: String,
    pub title: String,
}