use std::{env, fs::File};

const BAD_USER_INPUT_YEAR_MESSAGE: &'static str =
    "Start and end years must be integers between 1851 and 2021, inclusive";

const BAD_USER_INPUT_ENV_VARS_MESSAGE: &'static str = r#"
Incorrect number of variables specified.
Please run with hurdat2 filename and the start and end years
over which you would like to run the analysis.
e.g.,
hurdat2 <filename> <start year> <end year>

<filename> must be relative to current directory
<start year> and <end year> must be between 1851 and 2021, inclusive.
"#;

/// Responsible for retrieving, parsing, and validating
/// environment variable passed by the user.
/// e.g.,
/// hurdat2 <filename> <start year> <end year>
/// <filename> must be relative to current directory
/// <start year> and <end year> must be between 1851 and 2021, inclusive.
pub fn env_args() -> (String, i64, i64) {
    match &env::args().collect::<Vec<String>>()[1..] {
        [filename, start_year, end_year] => {
            let filename = filename.to_owned();
            let start_year = parse_year(start_year);
            let end_year = parse_year(end_year);

            (filename, start_year, end_year)
        }
        _ => panic!("{}", BAD_USER_INPUT_ENV_VARS_MESSAGE),
    }
}

pub fn open_file(filename: &str) -> File {
    let file_path = env::current_dir()
        .expect("Failed to read current directory")
        .join(filename);
    match File::open(&file_path) {
        Err(e) => panic!("Couldn't read {}: {}", &file_path.to_str().unwrap(), e),
        Ok(file) => file,
    }
}

fn parse_year(user_input: &String) -> i64 {
    let year = user_input
        .parse::<i64>()
        .expect(BAD_USER_INPUT_YEAR_MESSAGE);

    if year < 1851 || year > 2021 {
        panic!("{}", BAD_USER_INPUT_YEAR_MESSAGE);
    }

    year
}
