use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use futures::{channel::mpsc, executor, SinkExt, StreamExt};

mod data;

const CHANNEL_BUFFER_SIZE: usize = 32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (filename, start_year, end_year) = data::env_args();
    let file = data::open_file(&filename)?;

    let (tx, rx) = mpsc::channel::<HurricanePath>(CHANNEL_BUFFER_SIZE);
    tokio::task::spawn_blocking(move || read_file(tx, file, start_year, end_year));

    rx.map(|path| process_hurricane_path(path))
        .for_each(|result| async move {
            println!("{}", result.name);
        })
        .await;

    Ok(())
}

#[derive(Debug, Clone)]
struct HurricanePath {
    id: String,
    name: String,
    data: Vec<HurricaneDatum>,
}

impl HurricanePath {
    pub fn new(id: &str, name: &str) -> HurricanePath {
        HurricanePath {
            id: id.trim().to_string(),
            name: name.trim().to_string(),
            data: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct HurricaneDatum {
    wind_speed: i64,
}

impl HurricaneDatum {
    pub fn new() -> HurricaneDatum {
        HurricaneDatum { wind_speed: 4 }
    }
}

struct HurricaneResult {
    id: String,
    name: String,
}

fn read_file(mut tx: mpsc::Sender<HurricanePath>, file: File, start_year: i64, end_year: i64) {
    let mut path: Option<HurricanePath> = None;
    let mut rows_to_follow = 0;

    for line_result in BufReader::new(file).lines() {
        let line = line_result.unwrap();
        let line_vals: Vec<&str> = line.split(",").collect();

        if rows_to_follow == 0 {
            if let Some(p) = path {
                executor::block_on(tx.send(p)).unwrap();
            }

            // TODO - check that year makes sense here

            let hurricane_id = line_vals[0];
            let hurricane_name = line_vals[1];
            path = Some(HurricanePath::new(hurricane_id, hurricane_name));
            rows_to_follow = line_vals[2].trim().parse::<i64>().unwrap();

            continue;
        }

        let datum = HurricaneDatum::new();
        if let Some(p) = &mut path {
            p.data.push(datum);
        }

        rows_to_follow -= 1;
    }
}

fn process_hurricane_path(path: HurricanePath) -> HurricaneResult {
    HurricaneResult {
        id: path.id,
        name: path.name,
    }
}
