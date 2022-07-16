use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
struct Record {
    adult: Option<bool>,
    belongs_to_collection:Option<String>,
    budget: f32,
    genres: Option<String>,
    homepage: Option<String>,
    id: u64,
    imdb_id: String,
    original_language: String,
    original_title: String,
    overview: Option<String>,
    popularity: Option<f32>,
    poster_path: Option<String>,
    production_companies: Option<String>,
    production_countries: Option<String>,
    release_date: DateTime<Utc>,
    revenue:f32,
    runtime:f32,
    spoken_languages: String,
    status: Option<String>,
    tagline: Option<String>,
    title: String,
    video: Option<bool>,
    vote_average: Option<f32>,
    vote_count: Option<u64>,
}
