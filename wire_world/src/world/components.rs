use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Cell {
    pub position: Point,
    pub cell_type: CellType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CellType {
    EMPTY,
    WIRE,
    ELECTRON,
    TAIL
}

#[derive(Component, Default)]
pub struct NextUpdate;

#[derive(Component)]
pub struct Change(pub CellType);

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Point(pub usize, pub usize);
