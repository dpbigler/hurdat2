use std::{fs::File, io::BufRead, io::BufReader, sync::mpsc};

use geo::{Geometry, MultiPolygon};
use kml::Kml;

use super::data::{HurricanePathSnapshot, HurricaneTrack};

pub fn stream_file(tx: mpsc::Sender<HurricaneTrack>, file: File, start_year: i64, end_year: i64) {
    let mut hurricane_track: Option<HurricaneTrack> = None;
    let mut hurricane_index = 0;
    let mut rows_to_follow = 0;

    for line in BufReader::new(file).lines() {
        if rows_to_follow == 0 {
            if let Some(track) = hurricane_track {
                tx.send(track).unwrap();
                hurricane_index += 1;
            }

            ParsedLine {
                rows_to_follow,
                track: hurricane_track,
            } = ParsedLine::new(line.unwrap(), hurricane_index, start_year, end_year);

            continue;
        }

        if let Some(ref mut track) = hurricane_track {
            let snapshot = HurricanePathSnapshot::build_from_hurdat2(line.unwrap());
            track.path.push(snapshot);
        }

        rows_to_follow -= 1;
    }
}

pub fn parse_florida_kml(kml_data: &str) -> MultiPolygon<f64> {
    let florida_kml: Kml = kml_data.parse().unwrap();
    let florida_geometry_collection = kml::quick_collection(florida_kml).unwrap();

    let mut polygon_vec = Vec::new();
    for geometry in florida_geometry_collection {
        if let Geometry::Polygon(p) = geometry {
            polygon_vec.push(p)
        } else {
            panic!("Incorrectly specified multigeometry KML file")
        }
    }

    MultiPolygon::new(polygon_vec)
}

struct ParsedLine {
    rows_to_follow: i64,
    track: Option<HurricaneTrack>,
}

impl ParsedLine {
    pub fn new(line: String, index: usize, start_year: i64, end_year: i64) -> ParsedLine {
        let line_vals: Vec<&str> = line.split(",").map(|s| s.trim()).collect();

        let hurricane_id = line_vals[0];
        let hurricane_name = line_vals[1];
        let rows_to_follow = line_vals[2].parse().unwrap();

        let hurricane_year: i64 = hurricane_id[4..8].parse().unwrap();
        if hurricane_year < start_year || hurricane_year > end_year {
            return ParsedLine {
                rows_to_follow,
                track: None,
            };
        }

        ParsedLine {
            rows_to_follow,
            track: Some(HurricaneTrack::new(index, hurricane_name.to_string())),
        }
    }
}
