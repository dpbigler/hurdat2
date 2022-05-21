use std::{collections::HashMap, error::Error, sync::mpsc};

use pipeline::{analysis, data::HurricaneTrack, display, io};
use rayon::prelude::*;

mod pipeline;
mod startup;

fn main() -> Result<(), Box<dyn Error>> {
    let (filename, start_year, end_year) = startup::env_args();
    let file = startup::open_file(&filename)?;

    let (tx, rx) = mpsc::channel::<HurricaneTrack>();
    rayon::spawn(move || io::stream_file(tx, file, start_year, end_year));

    let analyses = rx
        .into_iter()
        .par_bridge()
        .map(|path| analysis::estimate_winds(path))
        .map(|wind_analysis| analysis::estimate_landfall(wind_analysis))
        .reduce(
            || HashMap::new(),
            |coll, path_map| analysis::reduce(coll, path_map),
        );

    display::all_analyses(analyses);

    Ok(())
}
