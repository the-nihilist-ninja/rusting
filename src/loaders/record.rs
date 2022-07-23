use std::error::Error;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
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
    #[serde(with = "my_date_format")]
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

mod my_date_format{
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

fn example(record_path:String) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(record_path)?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}