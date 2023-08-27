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
    pub expectation: bool,
    pub position: Point,
    pub from: usize,
    pub until: usize,
    pub status: OutputStatus,
}

#[derive(Component, Debug, Clone)]
pub struct Exercise {
    pub id: usize,
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

#[derive(Event)]
pub struct ChangeExercise(pub usize);

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

impl CellType {
    pub fn base_color(self) -> Color {
        match self {
            ELECTRON(_) => Color::YELLOW,
            WIRE(true) => Color::BLACK,
            WIRE(false) => Color::DARK_GRAY,
            TAIL(_) => Color::RED,
            EMPTY(true) => Color::DARK_GREEN,
            _others => Color::LIME_GREEN,
        }
    }
}

impl Into<String> for Point {
    fn into(self) -> String {
        format!("{} x {}", self.0, self.1)
    }
}