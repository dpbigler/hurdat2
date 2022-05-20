use std::collections::HashMap;

use super::data::HurricaneAnalysis;

pub fn all_analyses(mut analysis_map: HashMap<i64, HurricaneAnalysis>) {
    display_header();

    let mut i: i64 = 0;
    while !analysis_map.is_empty() {
        let analysis = analysis_map.remove(&i).unwrap();
        println!("{}", analysis);
    }

    display_footer();
}

fn display_header() {
    println!("Header")
}

fn display_footer() {
    println!("Footer")
}
