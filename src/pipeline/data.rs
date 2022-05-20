use std::fmt::Display;

pub struct HurricanePath {
    pub index: i64,
    pub id: String,
    pub name: String,
    pub data: Vec<HurricaneDatum>,
}

pub struct HurricaneDatum {
    pub wind_speed: i64,
}

pub struct HurricaneAnalysis {
    pub id: String,
    pub name: String,
}

impl Display for HurricaneAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} analyzed!", self.name)
    }
}
