use std::collections::HashMap;

use super::data::{HurricaneAnalysis, HurricanePath};

pub fn reduce(
    mut analysis_1: HashMap<i64, HurricaneAnalysis>,
    analysis_2: HashMap<i64, HurricaneAnalysis>,
) -> HashMap<i64, HurricaneAnalysis> {
    for (i, analysis) in analysis_2 {
        analysis_1.insert(i, analysis);
    }
    analysis_1
}

pub fn process(path: HurricanePath) -> HashMap<i64, HurricaneAnalysis> {
    let mut map = HashMap::new();
    let analysis = HurricaneAnalysis {
        id: path.id,
        name: path.name,
    };

    map.insert(path.index, analysis);
    map
}
