use std::{fmt::Debug, str::FromStr};

use chrono::{DateTime, NaiveTime, TimeZone, Utc};

pub struct HurricaneTrack {
    pub index: usize,
    pub name: String,
    pub path: Vec<HurricanePathSnapshot>,
}

pub struct HurricanePathSnapshot {
    pub datetime: DateTime<Utc>,
    pub latitude: String,
    pub longitude: String,
    pub max_sustained_wind_speed: i64,
}

pub struct HurricaneLandfallAnalysis {
    pub index: usize,
    pub name: String,
    pub path: Vec<HurricanePathSnapshot>,
    pub florida_enter_date: DateTime<Utc>,
    pub florida_exit_date: DateTime<Utc>,
}

pub struct HurricaneFinalAnalysis {
    pub name: String,
    pub landfall_date: DateTime<Utc>,
    pub max_sustained_wind_speed: i64,
    pub max_gust_wind_speed: i64,
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

        let latitude = split[4].to_string();
        let longitude = split[5].to_string();
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
