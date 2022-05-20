use std::{fs::File, io::BufRead, io::BufReader, sync::mpsc};

use super::data::HurricanePath;

pub fn stream_file(
    mut tx: mpsc::Sender<HurricanePath>,
    file: File,
    start_year: i64,
    end_year: i64,
) {
    let mut path: Option<HurricanePath> = None;
    let mut rows_to_follow = 0;

    for line_result in BufReader::new(file).lines() {
        let line = line_result.unwrap();
        let line_vals: Vec<&str> = line.split(",").collect();

        if rows_to_follow == 0 {
            if let Some(p) = path {
                tx.send(p).unwrap();
            }

            // TODO - check that year makes sense here

            let hurricane_id = line_vals[0];
            let hurricane_name = line_vals[1];
            // path = Some(HurricanePath::new(hurricane_id, hurricane_name));
            // rows_to_follow = line_vals[2].trim().parse::<i64>().unwrap();

            continue;
        }

        // let datum = HurricaneDatum::new();
        // if let Some(p) = &mut path {
        //     p.data.push(datum);
        // }

        rows_to_follow -= 1;
    }
}
