use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub project_id: i64,
}

#[derive(Serialize)]
pub struct NewStudent {
    pub name: String,
    pub project_id: i64,
}