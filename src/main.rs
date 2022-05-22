use std::{collections::BTreeMap, sync::mpsc};

use pipeline::{analysis, data::HurricaneTrack, display, io};
use rayon::prelude::*;

mod pipeline;
mod startup;

const FLORIDA_KML_FILE: &str = include_str!("../data/cb_2021_us_state_20m/florida.kml");

fn main() {
    let (filename, start_year, end_year) = startup::env_args();
    let file = startup::open_file(&filename);

    let (tx, rx) = mpsc::channel::<HurricaneTrack>();
    rayon::spawn(move || io::stream_file(tx, file, start_year, end_year));

    let florida_polygon_vec = io::parse_florida_kml(FLORIDA_KML_FILE);

    let analyses = rx
        .into_iter()
        .par_bridge()
        .map(|path| analysis::estimate_landfall(path, &florida_polygon_vec))
        .map(|landfall_analysis| analysis::estimate_max_winds(landfall_analysis))
        .reduce(
            || BTreeMap::new(),
            |coll, path_map| analysis::reduce(coll, path_map),
        );

    display::all_analyses(analyses);
}
