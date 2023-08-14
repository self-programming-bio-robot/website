use anyhow::Error;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use crate::world::components::{CellType, Point};
use crate::world::components::CellType::{ELECTRON, EMPTY, WIRE};

#[derive(Resource, Debug, Clone)]
pub struct WorldState {
    pub size: (usize, usize),
    pub map: Vec<Entity>,
}

#[derive(TypeUuid, TypePath)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct World {
    pub size: (usize, usize),
    pub map: Vec<CellType>,
}

#[derive(Resource)]
pub struct Counter {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct LevelConfig {
    pub level_name: Option<String>,
}

impl LevelConfig {
    pub fn empty() -> LevelConfig {
        LevelConfig {
            level_name: None
        }
    }
}

impl World {
    pub fn from_string(s: String) -> anyhow::Result<World, Error> {
        let mut lines = s.lines();
        let mut sizes = lines.next().ok_or(Error::msg("Not found size"))?.split(" ");
        let width: usize = sizes.next().ok_or(Error::msg("Not found width"))?.parse()?;
        let height: usize = sizes.next().ok_or(Error::msg("Not found height"))?.parse()?;

        let mut map: Vec<CellType> = Vec::with_capacity(width * height);
        for (i, line) in lines.enumerate() {
            if i >= height {
                break
            }
            for (j, cell) in line.split(" ").enumerate() {
                if j >= width {
                    break
                }
                map.push(match cell {
                    "a" => ELECTRON(false),
                    "w" => WIRE(false),
                    "A" => ELECTRON(true),
                    "W" => WIRE(true),
                    "E" => EMPTY(true),
                    _others => EMPTY(false),
                });
            }
        }

        Ok(
            World {
                size: (width, height),
                map,
            }
        )
    }

    pub fn index(&self, point: &Point) -> usize {
        point.1 * self.size.0 + point.0
    }

    pub fn get_cell(&self, point: &Point) -> CellType {
        self.map[self.index(point)].clone()
    }
}

impl WorldState {
    pub fn index(&self, point: &Point) -> usize {
        point.1 * self.size.0 + point.0
    }

    pub fn get_cell(&self, point: &Point) -> Entity {
        self.map[self.index(point)]
    }

    pub fn get_cells_around(&self, point: &Point) -> Vec<Entity> {
        let mut found = Vec::new();
        const OFFSETS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 1), (1, -1),
            (1, 0), (1, 1)];

        for offset in OFFSETS.iter() {
            let pos = Point(
                ((point.0 as isize + offset.0 + self.size.0 as isize) as usize) % self.size.0,
                ((point.1 as isize + offset.1 + self.size.1 as isize) as usize) % self.size.1,
            );

            found.push(self.get_cell(&pos));
        }

        found
    }
}