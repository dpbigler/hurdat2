use std::collections::HashMap;

use super::data::{HurricaneFinalAnalysis, HurricaneTrack, HurricaneWindAnalysis};

pub fn reduce(
    mut analysis_1: HashMap<usize, HurricaneFinalAnalysis>,
    analysis_2: HashMap<usize, HurricaneFinalAnalysis>,
) -> HashMap<usize, HurricaneFinalAnalysis> {
    for (i, analysis) in analysis_2 {
        analysis_1.insert(i, analysis);
    }
    analysis_1
}

pub fn estimate_winds(track: HurricaneTrack) -> HurricaneWindAnalysis {
    let mut max_track_sustained_wind_speed = -1;
    for snapshot in &track.path {
        if snapshot.max_sustained_wind_speed > max_track_sustained_wind_speed {
            max_track_sustained_wind_speed = snapshot.max_sustained_wind_speed;
        }
    }

    let max_gust_wind = (max_track_sustained_wind_speed as f64) * 1.55;
    HurricaneWindAnalysis {
        track,
        est_max_sustained_wind: max_track_sustained_wind_speed,
        est_max_gust_wind: max_gust_wind.floor() as i64,
    }
}

pub fn estimate_landfall(
    wind_analysis: HurricaneWindAnalysis,
) -> HashMap<usize, HurricaneFinalAnalysis> {
    let mut indexed_analysis = HashMap::new();

    let final_analysis = HurricaneFinalAnalysis {
        name: wind_analysis.track.name,
        landfall_date: "Who knows".to_string(),
        est_max_sustained_wind: wind_analysis.est_max_sustained_wind,
        est_max_gust_wind: wind_analysis.est_max_gust_wind,
    };

    indexed_analysis.insert(wind_analysis.track.index, final_analysis);
    indexed_analysis
}
