use std::{collections::BTreeMap, fmt::Display};

use super::data::HurricaneFinalAnalysis;

pub fn all_analyses(analysis_map: BTreeMap<usize, HurricaneFinalAnalysis>) {
    display_header();

    for (_, final_analysis) in analysis_map {
        println!("{}", final_analysis);
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
            self.name, self.landfall_date, self.max_sustained_wind_speed, self.max_gust_wind_speed
        )
    }
}
