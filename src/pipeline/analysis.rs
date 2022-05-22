use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use geo::{
    coord,
    prelude::{Contains, EuclideanDistance},
    Coordinate, MultiPolygon, Point,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::data::{
    HurricaneFinalAnalysis, HurricaneLandfallAnalysis, HurricanePathSnapshot, HurricaneTrack,
};

const HURRICANE_GUST_FACTOR: f64 = 1.55;

/// A helper struct for landfall analysis. Stores
/// the time of a snapshot, the coordinate of the hurricane,
/// and whether that coordinate is over Florida.
struct SnapshotOverFlorida {
    is_in_florida: bool,
    datetime: DateTime<Utc>,
    coordinate: Coordinate<f64>,
}

/// A helper struct for landfall analysis. Stores
/// only the time of a snapshot and the coordinate
/// of the hurricane.
struct SnapshotTimedCoordinate {
    coordinate: Coordinate<f64>,
    datetime: DateTime<Utc>,
}

impl SnapshotTimedCoordinate {
    fn build(
        coordinate: Coordinate<f64>,
        datetime: DateTime<Utc>,
    ) -> Option<SnapshotTimedCoordinate> {
        Some(SnapshotTimedCoordinate {
            coordinate,
            datetime,
        })
    }
}

/// Estimates the time and date that the hurricane landed in Florida.
/// If the hurricane did not land in Florida, we return None. These
/// hurricanes will be silently filtered out during the following
/// steps in the pipeline.
pub fn estimate_landfall(
    track: HurricaneTrack,
    florida_multipolygon: &MultiPolygon<f64>,
) -> Option<HurricaneLandfallAnalysis> {
    let mut last_coord_outside_florida: Option<SnapshotTimedCoordinate> = None;
    let mut first_coord_inside_florida: Option<SnapshotTimedCoordinate> = None;
    let mut last_coord_inside_florida: Option<SnapshotTimedCoordinate> = None;

    // In parallel, we determine whether or not a hurricane snapshot was
    // taken over Florida.
    let snapshot_summaries: Vec<SnapshotOverFlorida> = track
        .path
        .par_iter()
        .map(|snapshot| {
            let snapshot_coordinate = coord! {
                x: snapshot.longitude,
                y: snapshot.latitude
            };
            let snapshot_is_in_florida = florida_multipolygon.contains(&snapshot_coordinate);
            SnapshotOverFlorida {
                is_in_florida: snapshot_is_in_florida,
                datetime: snapshot.datetime,
                coordinate: snapshot_coordinate,
            }
        })
        .collect();

    for snapshot in snapshot_summaries {
        if !snapshot.is_in_florida && first_coord_inside_florida.is_none() {
            last_coord_outside_florida =
                SnapshotTimedCoordinate::build(snapshot.coordinate, snapshot.datetime);
        }

        if snapshot.is_in_florida && first_coord_inside_florida.is_none() {
            first_coord_inside_florida =
                SnapshotTimedCoordinate::build(snapshot.coordinate, snapshot.datetime);
        }

        if snapshot.is_in_florida && first_coord_inside_florida.is_some() {
            last_coord_inside_florida =
                SnapshotTimedCoordinate::build(snapshot.coordinate, snapshot.datetime);
        }
    }

    let landfall: Option<DateTime<Utc>> =
        match (&last_coord_outside_florida, &first_coord_inside_florida) {
            (_, None) => None,
            (None, Some(tracking_coordinate)) => Some(tracking_coordinate.datetime),
            (Some(outside_florida), Some(inside_florida)) => Some(interpolate_landfall(
                outside_florida,
                inside_florida,
                florida_multipolygon,
            )),
        };

    match landfall {
        None => None,
        Some(landfall_time) => Some(HurricaneLandfallAnalysis {
            index: track.index,
            name: track.name,
            path: track.path,
            landfall: landfall_time,
            first_datetime_over_florida: first_coord_inside_florida.unwrap().datetime,
            last_datetime_over_florida: last_coord_inside_florida.unwrap().datetime,
        }),
    }
}

/// Estimates the max wind speeds of the hurricane while it was over
/// Florida. More details about selection of the calculation can be
/// found in the ReadMe.
///
/// We return a BTreeMap, mapping the hurricane index to the
/// HurricaneFinalAnalysis for the reduce step in our pipeline.
pub fn estimate_max_winds(
    landfall_analysis: Option<HurricaneLandfallAnalysis>,
) -> BTreeMap<usize, HurricaneFinalAnalysis> {
    let mut indexed_analysis = BTreeMap::new();
    if let Some(analysis) = landfall_analysis {
        let max_sustained_wind_speed = &analysis
            .path
            .par_iter()
            .filter(|snapshot| snapshot_is_over_florida(&snapshot, &analysis))
            .map(|snapshot| snapshot.max_sustained_wind_speed)
            .reduce(
                || 0,
                |max_wind_1, max_wind_2| std::cmp::max(max_wind_1, max_wind_2),
            );

        let max_gust_wind_speed = (*max_sustained_wind_speed as f64) * HURRICANE_GUST_FACTOR;
        let final_analysis = HurricaneFinalAnalysis {
            name: analysis.name,
            landfall: analysis.landfall,
            max_sustained_wind_speed: *max_sustained_wind_speed as f64,
            max_gust_wind_speed,
        };
        indexed_analysis.insert(analysis.index, final_analysis);
    }
    indexed_analysis
}

/// Used in the reduce step of the map-reduce pipeline. The
/// reduce provided by Rayon is fully parallel, so the parameter
/// types must be the same as the return type.
///
/// A BTreeMap is used so that we can print the data for each hurricane
/// in the order that they appeared in the hurdat2 dataset without
/// performing a sort step.
pub fn reduce(
    mut analysis_1: BTreeMap<usize, HurricaneFinalAnalysis>,
    analysis_2: BTreeMap<usize, HurricaneFinalAnalysis>,
) -> BTreeMap<usize, HurricaneFinalAnalysis> {
    for (i, analysis) in analysis_2 {
        analysis_1.insert(i, analysis);
    }
    analysis_1
}

fn interpolate_landfall(
    outside_florida_tc: &SnapshotTimedCoordinate,
    inside_florida_tc: &SnapshotTimedCoordinate,
    florida_multipolygon: &MultiPolygon<f64>,
) -> DateTime<Utc> {
    let outside_point = Point::from(outside_florida_tc.coordinate);
    let inside_point = Point::from(inside_florida_tc.coordinate);
    let d1 = outside_point.euclidean_distance(&inside_point);
    let d2 = outside_point.euclidean_distance(florida_multipolygon);

    // Chrono arithmetic hack
    let normalized_d1 = (d1 * 1_000.0) as i32;
    let normalized_d2 = (d2 * 1_000.0) as i32;

    let time_to_florida =
        (inside_florida_tc.datetime - outside_florida_tc.datetime) * normalized_d2 / normalized_d1;

    outside_florida_tc.datetime + time_to_florida
}

fn snapshot_is_over_florida(
    snapshot: &HurricanePathSnapshot,
    analysis: &HurricaneLandfallAnalysis,
) -> bool {
    snapshot.datetime >= analysis.first_datetime_over_florida
        && snapshot.datetime <= analysis.last_datetime_over_florida
}
