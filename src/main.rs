use std::{collections::HashMap, sync::mpsc};

use pipeline::{analysis, data::HurricaneTrack, display, io};
use rayon::prelude::*;

mod pipeline;
mod startup;

fn main() {
    let (filename, start_year, end_year) = startup::env_args();
    let file = startup::open_file(&filename)
        .expect("Failed to open file");

    let (tx, rx) = mpsc::channel::<HurricaneTrack>();
    rayon::spawn(move || io::stream_file(tx, file, start_year, end_year));

    let analyses = rx
        .into_iter()
        .par_bridge()
        .map(|path| analysis::estimate_landfall(path))
        .map(|landfall_analysis| analysis::estimate_max_winds(landfall_analysis))
        .reduce(
            || HashMap::new(),
            |coll, path_map| analysis::reduce(coll, path_map),
        );

    display::all_analyses(analyses);
}
