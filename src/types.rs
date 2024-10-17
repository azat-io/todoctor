use crate::prepare_blame_data::PreparedBlameData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TodoData {
    pub comment: String,
    pub path: String,
    pub start: usize,
    pub end: usize,
    pub kind: String,
}

#[derive(Debug, Serialize)]
pub struct TodoWithBlame {
    pub blame: PreparedBlameData,
    pub comment: String,
    pub kind: String,
    pub path: String,
    pub line: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoHistory {
    pub count: usize,
    pub date: String,
}
