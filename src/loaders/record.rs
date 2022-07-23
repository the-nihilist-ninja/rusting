use std::error::Error;

use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    adult: Option<String>,
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
    #[serde(with = "my_date_format")]
    release_date: NaiveDate,
    // release_date: Option<String>,
    revenue:Option<f32>,
    runtime:Option<f32>,
    spoken_languages: String,
    status: Option<String>,
    tagline: Option<String>,
    title: String,
    video: Option<String>,
    vote_average: Option<f32>,
    vote_count: Option<u64>,
}

// Took from official documentation
mod my_date_format{
    use chrono::{Date, NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn read_record(record_path:String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(record_path)?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.    
        let rc: Metadata = result?;
    // sprintln!("{:?}", rc);
    }
    Ok(())
}