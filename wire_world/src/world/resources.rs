use anyhow::Error;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use crate::world::components::{CellType, Point};
use crate::world::components::CellType::{ELECTRON, EMPTY, WIRE};

#[derive(Resource, Debug, Clone)]
pub struct WorldState {
    pub size: (usize, usize),
    pub map: Vec<Entity>,
    pub exercises: Vec<ExerciseData>,
    pub electron_available: bool,
    pub lock: bool,
}

#[derive(TypeUuid, TypePath, Debug)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct World {
    pub size: (usize, usize),
    pub map: Vec<CellType>,
    pub electron_available: bool,
    pub exercises: Vec<ExerciseData>,
}

#[derive(Debug, Clone)]
pub struct ExerciseData {
    pub description: String,
    pub timeout: usize,
    pub spawns: Vec<(Point, usize)>,
    pub outputs: Vec<(Point, usize, usize)>,
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
        let electron_available: bool = lines.next()
            .ok_or(Error::msg("Not found electron-available flag"))?.parse()?;

        let mut map: Vec<CellType> = Vec::with_capacity(width * height);
        for i in 0..height {
            let line = lines.next().ok_or(Error::msg("File is broken"))?;

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

        let exercise_count: usize = lines.next()
            .ok_or(Error::msg("Not found count of exercises"))?.parse()?;
        let mut exercises: Vec<ExerciseData> = Vec::with_capacity(exercise_count);

        for i in 0..exercise_count {
            let mut description = String::new();
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    description.remove(description.len()-1);
                    break;
                };

                description.push_str(line);
                description.push('\n');
            }

            let timeout: usize = lines.next()
                .ok_or(Error::msg(format!("Not found timeout of exercise {i}")))?
                .parse()?;

            let spawns_count: usize = lines.next()
                .ok_or(Error::msg("Not found count of electron spawns"))?.parse()?;
            let mut spawns: Vec<(Point, usize)> = Vec::with_capacity(spawns_count);
            for _ in 0..spawns_count {
                spawns.push(
                    Self::parse_electron_spawn(lines.next()
                            .ok_or(Error::msg("Not found line with electron spawn"))?
                    )?
                );
            }

            let outputs_count: usize = lines.next()
                .ok_or(Error::msg("Not found count of outputs"))?.parse()?;
            let mut outputs: Vec<(Point, usize, usize)> = Vec::with_capacity(outputs_count);
            for _ in 0..outputs_count {
                outputs.push(
                    Self::parse_output(lines.next()
                        .ok_or(Error::msg("Not found line with output"))?
                    )?
                );
            }
            exercises.push(ExerciseData { description, timeout, spawns, outputs });
        }

        Ok(
            World {
                size: (width, height),
                map,
                exercises,
                electron_available
            }
        )
    }

    pub fn index(&self, point: &Point) -> usize {
        point.1 * self.size.0 + point.0
    }

    pub fn get_cell(&self, point: &Point) -> CellType {
        self.map[self.index(point)].clone()
    }

    fn parse_electron_spawn(line: &str) -> Result<(Point, usize), Error> {
        let mut sizes = line.split(" ");
        let instant: usize = sizes.next()
            .ok_or(Error::msg("Not found instant of electron spawn"))?.parse()?;
        let x: usize = sizes.next()
            .ok_or(Error::msg("Not found x of electron spawn"))?.parse()?;
        let y: usize = sizes.next()
            .ok_or(Error::msg("Not found y of electron spawn"))?.parse()?;

        Ok((Point(x, y), instant))
    }

    fn parse_output(line: &str) -> Result<(Point, usize, usize), Error> {
        let mut sizes = line.split(" ");
        let from: usize = sizes.next()
            .ok_or(Error::msg("Not found from instant of output"))?.parse()?;
        let until: usize = sizes.next()
            .ok_or(Error::msg("Not found until instant of output"))?.parse()?;
        let x: usize = sizes.next()
            .ok_or(Error::msg("Not found x of output"))?.parse()?;
        let y: usize = sizes.next()
            .ok_or(Error::msg("Not found y of output"))?.parse()?;

        Ok((Point(x, y), from, until))
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