use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub filename: String, 
    pub professor_id: i64,

    pub attempts_limit: i64,
    pub weather: String,
    pub seabed_hardness: String,
    pub budget: f64,
    pub geotiff_min_depth: f64,
    pub geotiff_max_depth: f64,
}

pub struct NewProject {
    pub name: String,
    pub description: String,
    pub file: web_sys::File,
    pub attempts_limit: i64,
    pub weather: String,
    pub seabed_hardness: String,
    pub budget: f64,
    pub geotiff_min_depth: f64,
    pub geotiff_max_depth: f64,
}


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: Option<String>,
    pub attempts_limit: i64,
    pub weather: String,
    pub seabed_hardness: String,
    pub budget: f64,
    pub geotiff_min_depth: f64,
    pub geotiff_max_depth: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdminProjectView {
    pub id: usize,
    pub filename: String,
    pub professor_id: i64,
    #[serde(flatten)] 
    pub metadata: ProjectMetadata,
}