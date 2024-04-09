use serde::{Deserialize};

pub struct Object {
    pub x: usize,
    pub y: usize,
    pub data: ObjectDefinition,
}

#[derive(Deserialize, Clone)]
pub struct ObjectDefinition {
    pub name: String,
    pub description: String,
    pub actions: Vec<ObjectAction>,
}

#[derive(Deserialize, Clone)]
pub struct ObjectAction {
    pub name: String,
    pub description: String,
    pub requirements: Vec<String>,
}