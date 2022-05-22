use std::{collections::BTreeMap, sync::mpsc};

use pipeline::{analysis, data::HurricaneTrack, display, io};
use rayon::prelude::*;

mod pipeline;
mod startup;

const FLORIDA_KML_FILE: &str = include_str!("../data/cb_2021_us_state_20m/florida.kml");

/// This application attempts to construct an analysis of the hurdat2
/// dataset provided by the National Hurricane Center. In particular,
/// we print a table consisting of all hurricanes that landed in Florida,
/// the date and time that landfall occurred, and an estimate of the
/// maximum sustained and gust wind speeds.
///
/// This application also attempts to parallelize everything that can
/// be parallelized, which informs many of the design choices made
/// below.
///
/// After installation, this application can be run from the command line, e.g.,
/// ```
/// hurdat2 <hurdat2 file> 2000 2010
///
/// | Name            | Landfall                  | Max Sustained (kt)   | Max Gust (kt)   |
/// ----------------------------------------------------------------------------------------
/// | GORDON          | 2000-09-18 06:00:00 UTC   |                   40 |              62 |
/// ----------------------------------------------------------------------------------------
/// | HELENE          | 2000-09-22 12:00:00 UTC   |                   35 |              54 |
/// ----------------------------------------------------------------------------------------
/// | LESLIE          | 2000-10-04 12:00:00 UTC   |                   30 |              46 |
/// ...
///
/// ```
fn main() {
    let (filename, start_year, end_year) = startup::env_args();
    let file = startup::open_file(&filename);

    let (tx, rx) = mpsc::channel::<HurricaneTrack>();

    // Start a process that reads in hurdat2 data, parses it to a HurricaneTrack struct,
    // and passes them back to a channel so that processing can begin "off the wire."
    rayon::spawn(move || io::stream_file(tx, file, start_year, end_year));

    let florida_polygon_vec = io::parse_florida_kml(FLORIDA_KML_FILE);

    let analyses = rx
        .into_iter()
        // Converts the iterator of HurricaneTracks that we pull off the channel
        // into a ParallelIterator. Importantly, par_bridge makes no attempt to
        // preserve the order of the iterator, so we have to keep track of
        // the order in which we receive HurricaneTracks ourselves.
        .par_bridge()
        .map(|path| analysis::estimate_landfall(path, &florida_polygon_vec))
        .map(|landfall_analysis| analysis::estimate_max_winds(landfall_analysis))
        .reduce(
            || BTreeMap::new(),
            |coll, path_map| analysis::reduce(coll, path_map),
        );

    display::all_analyses(analyses);
}
