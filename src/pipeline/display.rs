use std::{collections::BTreeMap, fmt::Display};

use super::data::HurricaneFinalAnalysis;

const TABLE_WIDTH: usize = 83;

pub fn all_analyses(analysis_map: BTreeMap<usize, HurricaneFinalAnalysis>) {
    display_header();

    for (_, final_analysis) in analysis_map {
        println!("{}", final_analysis);
    }
}

fn display_header() {
    print!(
        "| {:15} | {:20} | {:20} | {:15} |\n",
        "Name", "Landfall", "Max Sustained (kt)", "Max Gust (kt)"
    );

    println!("{}", "-".repeat(TABLE_WIDTH));
}

impl Display for HurricaneFinalAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "| {:15} | {:20} | {:20.0} | {:15.0} |\n",
            self.name,
            self.landfall.format("%Y-%m-%d %H:%M").to_string(),
            self.max_sustained_wind_speed,
            self.max_gust_wind_speed
        )?;

        write!(f, "{}", "-".repeat(TABLE_WIDTH))
    }
}
