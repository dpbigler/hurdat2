use std::{fmt::Debug, str::FromStr};

use chrono::{DateTime, NaiveTime, TimeZone, Utc};

pub struct HurricaneTrack {
    pub index: usize,
    pub name: String,
    pub path: Vec<HurricanePathSnapshot>,
}

pub struct HurricanePathSnapshot {
    pub datetime: DateTime<Utc>,
    pub latitude: f64,
    pub longitude: f64,
    pub max_sustained_wind_speed: i64,
}

pub struct HurricaneLandfallAnalysis {
    pub index: usize,
    pub name: String,
    pub path: Vec<HurricanePathSnapshot>,
    pub landfall: DateTime<Utc>,
    pub first_datetime_over_florida: DateTime<Utc>,
    pub last_datetime_over_florida: DateTime<Utc>,
}

pub struct HurricaneFinalAnalysis {
    pub name: String,
    pub landfall_date: DateTime<Utc>,
    pub max_sustained_wind_speed: f64,
    pub max_gust_wind_speed: f64,
}

impl HurricaneTrack {
    pub fn new(index: usize, name: String) -> HurricaneTrack {
        HurricaneTrack {
            index,
            name,
            path: Vec::new(),
        }
    }
}
impl HurricanePathSnapshot {
    pub fn build_from_hurdat2(line: String) -> HurricanePathSnapshot {
        let split: Vec<&str> = line.split(",").map(|s| s.trim()).collect();

        let year = parse_int(&split[0][0..4]);
        let month = parse_int(&split[0][4..6]);
        let day = parse_int(&split[0][6..8]);

        let hours = parse_int(&split[1][0..2]);
        let minutes = parse_int(&split[1][2..4]);

        let datetime = Utc
            .ymd(year, month, day)
            .and_time(NaiveTime::from_hms(hours, minutes, 0))
            .expect("Error parsing hurdat2 DateTime");

        let latitude = parse_coordinate(split[4]);
        let longitude = parse_coordinate(split[5]);

        let max_sustained_wind_speed = parse_int(&split[6]);

        HurricanePathSnapshot {
            datetime,
            latitude,
            longitude,
            max_sustained_wind_speed,
        }
    }
}

fn parse_int<T: FromStr>(s: &str) -> T
where
    T::Err: Debug,
{
    s.parse().expect("Error while parsing hurdat2 data line")
}

fn parse_coordinate(s: &str) -> f64 {
    let coordinate_value: f64 = s[0..s.len() - 1]
        .parse()
        .expect("Error parsing coordinate value");

    let coordinate_direction = &s[s.len() - 1..s.len()];

    match coordinate_direction.as_bytes() {
        b"W" | b"S" => -coordinate_value,
        b"E" | b"N" => coordinate_value,
        _ => panic!("Error parsing coordinate direction"),
    }
}
