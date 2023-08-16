use bevy::prelude::*;
use crate::world::components::CellType::*;

#[derive(Component, Debug, Clone)]
pub struct Cell {
    pub position: Point,
    pub cell_type: CellType,
}

#[derive(Component, Debug, Clone)]
pub struct ElectronSpawn {
    pub position: Point,
    pub instant: usize,
}

#[derive(Component, Debug, Clone)]
pub struct ExpectedOutput {
    pub position: Point,
    pub from: usize,
    pub until: usize,
    pub status: OutputStatus,
}

#[derive(Component, Debug, Clone)]
pub struct Exercise {
    // pub description: Entity,
    pub ticks: usize,
    pub timeout: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CellType {
    EMPTY(bool),
    WIRE(bool),
    ELECTRON(bool),
    TAIL(bool)
}

impl Cell {
    pub fn is_fixed(&self) -> bool {
        match self.cell_type.clone() {
            EMPTY(fixed) => fixed,
            WIRE(fixed) => fixed,
            ELECTRON(fixed) => fixed,
            TAIL(fixed) => fixed,
        }
    }
}

#[derive(Component, Default)]
pub struct NextUpdate;

#[derive(Component)]
pub struct Change(pub CellType);

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Point(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum OutputStatus {
    Inactive,
    Waiting,
    Success,
    Fail,
}