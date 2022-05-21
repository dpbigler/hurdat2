use std::{collections::HashMap, fmt::Display};

use super::data::HurricaneFinalAnalysis;

pub fn all_analyses(mut analysis_map: HashMap<usize, HurricaneFinalAnalysis>) {
    display_header();

    let mut i = 0;
    while !analysis_map.is_empty() {
        println!("{}", analysis_map.remove(&i).unwrap());
        i += 1;
    }

    display_footer();
}

fn display_header() {
    println!("Header")
}

fn display_footer() {
    println!("Footer")
}

impl Display for HurricaneFinalAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Date: {}, Sustained: {}, Gust: {}",
            self.name, self.landfall_date, self.est_max_sustained_wind, self.est_max_gust_wind
        )
    }
}
