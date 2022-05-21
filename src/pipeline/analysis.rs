use std::collections::HashMap;

use super::data::{HurricaneFinalAnalysis, HurricaneLandfallAnalysis, HurricaneTrack};

pub fn reduce(
    mut analysis_1: HashMap<usize, HurricaneFinalAnalysis>,
    analysis_2: HashMap<usize, HurricaneFinalAnalysis>,
) -> HashMap<usize, HurricaneFinalAnalysis> {
    for (i, analysis) in analysis_2 {
        analysis_1.insert(i, analysis);
    }
    analysis_1
}

pub fn estimate_landfall(track: HurricaneTrack) -> Option<HurricaneLandfallAnalysis> {
    todo!()
    // let mut max_track_sustained_wind_speed = -1;
    // for snapshot in &track.path {
    //     if snapshot.max_sustained_wind_speed > max_track_sustained_wind_speed {
    //         max_track_sustained_wind_speed = snapshot.max_sustained_wind_speed;
    //     }
    // }

    // let max_gust_wind = (max_track_sustained_wind_speed as f64) * 1.55;
    // HurricaneLandfallAnalysis {
    //     track,
    //     est_max_sustained_wind: max_track_sustained_wind_speed,
    //     est_max_gust_wind: max_gust_wind.floor() as i64,
    // }
}

pub fn estimate_max_winds(
    landfall_analysis: Option<HurricaneLandfallAnalysis>,
) -> HashMap<usize, HurricaneFinalAnalysis> {
    let mut indexed_analysis = HashMap::new();
    if let Some(analysis) = landfall_analysis {
        let mut max_sustained_wind_speed = -1;
        for snapshot in &analysis.path {
            let snapshot_is_over_florida = snapshot.datetime >= analysis.florida_enter_date
                && snapshot.datetime <= analysis.florida_exit_date;

            if snapshot_is_over_florida && snapshot.max_sustained_wind_speed > max_sustained_wind_speed {
                max_sustained_wind_speed = snapshot.max_sustained_wind_speed;
            }
        }
        let max_gust_wind_speed = (max_sustained_wind_speed as f64) * 1.55;
        let final_analysis = HurricaneFinalAnalysis {
            name: analysis.name,
            landfall_date: analysis.florida_enter_date,
            max_sustained_wind_speed,
            max_gust_wind_speed: max_gust_wind_speed as i64
        };
        indexed_analysis.insert(analysis.index, final_analysis);
    }

    indexed_analysis
}
