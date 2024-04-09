use serde::Deserialize;

pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CellDefinition {
    pub id: usize,
    pub description: String,
}
