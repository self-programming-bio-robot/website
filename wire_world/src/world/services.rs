use std::mem::swap;
use std::ops::Mul;

use bevy::asset::{AssetEvent, AssetServer};
use bevy::log::error;
use bevy::prelude::*;
use bevy::sprite::Anchor::{BottomCenter, TopCenter};
use bevy::text::{BreakLineOn, Text2dBounds};

use crate::control::{ClickEvent, MoveCamera};
use crate::GameState;
use crate::world::CELL_SIZE;
use crate::world::components::{Cell, Change, ElectronSpawn, Exercise, ExpectedOutput, NextUpdate, OutputStatus, Point};
use crate::world::components::CellType::{ELECTRON, EMPTY, TAIL, WIRE};
use crate::world::components::OutputStatus::{Fail, Inactive, Success, Waiting};
use crate::world::resources::{Counter, ExerciseData, LevelConfig, World, WorldState};

pub fn init_level(
    mut next_state: ResMut<NextState<GameState>>,
    level_config: Res<LevelConfig>,
    assets: Res<AssetServer>,
    mut counter: ResMut<Counter>,
) {
    counter.timer.pause();
    if let Some(level_name) = level_config.level_name.clone() {
        let _ = assets.load_untyped(level_name);
    } else {
        error!("Level config is undefined");
        next_state.set(GameState::LevelsList);
    }
}

pub fn load_level(
    mut commands: Commands,
    mut levels_events: EventReader<AssetEvent<World>>,
    levels: Res<Assets<World>>,
    mut camera_events: EventWriter<MoveCamera>,
    asset_server: Res<AssetServer>,
) {
    for event in levels_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(level) = levels.get(handle) {
                    let world_state = spawn_level(level, &mut commands, &asset_server);
                    let pos = Vec2::new(
                        CELL_SIZE * level.size.0 as f32,
                        -CELL_SIZE * level.size.1 as f32,
                    ) * 0.5;
                    camera_events.send(MoveCamera {
                        pos,
                        force: true,
                    });
                    commands.insert_resource(world_state);
                }
            }
            _others => {}
        }
    }
}

pub fn find_cell_to_update(
    mut counter: ResMut<Counter>,
    mut commands: Commands,
    cells: Query<(Entity, &Cell)>,
    updating_cells: Query<(Entity, &Cell), With<NextUpdate>>,
    time: Res<Time>,
    world: Option<Res<WorldState>>,
    mut exercises: Query<&mut Exercise>,
) {
    if let Some(world) = world {
        let timer = counter.timer.tick(time.delta());
        if timer.finished() {
            if let Ok(mut exercise) = exercises.get_single_mut() {
                exercise.ticks += 1;
            }

            for (id, cell) in updating_cells.iter() {
                commands.entity(id).remove::<NextUpdate>();

                let count_electron_around = world.get_cells_around(&cell.position)
                    .iter()
                    .map(|id| cells.get(*id).unwrap().1)
                    .filter(|cell|
                        cell.cell_type == ELECTRON(true) || cell.cell_type == ELECTRON(false)
                    )
                    .count();

                match cell.cell_type.clone() {
                    WIRE(fixed) => {
                        if count_electron_around == 1 || count_electron_around == 2 {
                            commands.entity(id).insert(Change(ELECTRON(fixed)));
                        }
                    }
                    TAIL(fixed) => {
                        commands.entity(id).insert(Change(WIRE(fixed)));
                    }
                    ELECTRON(fixed) => {
                        commands.entity(id).insert(Change(TAIL(fixed)));
                    }
                    _others => {}
                }
            }
        }
    }
}

pub fn update_cells(
    mut cells: Query<(Entity, &mut Cell, &mut Sprite, &Change), With<Change>>,
    mut commands: Commands,
    world: Option<Res<WorldState>>,
) {
    if let Some(world) = world {
        for (_id, mut cell, mut sprite, changed) in cells.iter_mut() {
            cell.cell_type = changed.0.clone();
            sprite.color = match cell.cell_type.clone() {
                ELECTRON(_) => {
                    commands.entity(world.map[world.index(&cell.position)]).insert(NextUpdate::default());
                    let cells_around = world.get_cells_around(&cell.position);

                    for cell in cells_around.iter() {
                        commands.entity(*cell).insert(NextUpdate::default());
                    }

                    Color::YELLOW
                }
                WIRE(_) => Color::BLACK,
                TAIL(_) => {
                    commands.entity(world.map[world.index(&cell.position)]).insert(NextUpdate::default());
                    Color::RED
                }
                _others => Color::LIME_GREEN,
            };
        }
    }
}

pub fn handle_clicks(
    mut commands: Commands,
    mut click_events: EventReader<ClickEvent>,
    world: Option<Res<WorldState>>,
    cells: Query<&mut Cell>,
) {
    if let Some(world) = world {
        let half_cell_size = CELL_SIZE / 2.;

        for event in click_events.iter() {
            let x = (event.pos.x + half_cell_size) / CELL_SIZE;
            let y = (-event.pos.y + half_cell_size) / CELL_SIZE;

            if x < 0. || x >= world.size.0 as f32 || y < 0. || y >= world.size.1 as f32 {
                continue;
            }

            let x = x.trunc() as usize;
            let y = y.trunc() as usize;
            let cell = world.get_cell(&Point(x, y));
            if let Ok(cell_type) = cells.get(cell) {
                if !cell_type.is_fixed() {
                    commands.entity(cell).insert(Change(
                        match event.button {
                            MouseButton::Left => WIRE(false),
                            _others => ELECTRON(false),
                        })
                    );
                }
            }
        }
    }
}

pub fn spawn_electron(
    mut commands: Commands,
    exercises: Query<&Exercise, Changed<Exercise>>,
    spawns: Query<&ElectronSpawn>,
    world: Option<Res<WorldState>>,
    cells: Query<&mut Cell>,
) {
    if let Some(world) = world {
        if let Ok(exercise) = exercises.get_single() {
            info!("exercise tick {}", exercise.ticks);

            for spawn in spawns.iter() {
                info!("spawn is excepting {}", spawn.instant);
                if spawn.instant == exercise.ticks {
                    let cell = world.get_cell(&spawn.position);
                    if let Ok(cell_type) = cells.get(cell) {
                        commands.entity(cell).insert(Change(ELECTRON(cell_type.is_fixed())));
                    }
                }
            }
        }
    }
}

pub fn handle_outputs(
    mut outputs: Query<&mut ExpectedOutput>,
    exercises: Query<&Exercise, Changed<Exercise>>,
    world: Option<Res<WorldState>>,
    cells: Query<&mut Cell>,
) {
    if let Some(world) = world {
        if let Ok(exercise) = exercises.get_single() {
            for mut output in outputs.iter_mut() {
                if exercise.ticks < output.from {
                    output.status = Inactive;
                } else if exercise.ticks >= output.from && exercise.ticks < output.until {
                    if output.status == Inactive {
                        output.status = Waiting;
                    }
                    let cell = world.get_cell(&output.position);
                    if let Ok(cell) = cells.get(cell) {
                        if let ELECTRON(_) = cell.cell_type {
                            output.status = Success;
                        }
                    }
                } else if exercise.ticks >= output.until && output.status != Success {
                    output.status = Fail;
                }
            }
        }
    }
}

pub fn handle_exercises(
    mut counter: ResMut<Counter>,
    outputs: Query<&ExpectedOutput>,
    exercises: Query<&Exercise, Changed<Exercise>>,
) {
    if let Ok(exercise) = exercises.get_single() {
        let statues: Vec<OutputStatus> = outputs.iter()
            .map(|output| output.status.clone()).collect();
        debug!("statues: {:?}", statues);
        if outputs.iter().any(|o| o.status == Fail) || exercise.ticks > exercise.timeout {
            info!("Fail exercise");
            counter.timer.pause();
        }
        if outputs.iter().all(|o| o.status == Success) {
            info!("Success exercise");
            counter.timer.pause();
        }
    }
}

fn spawn_level(
    world: &World,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> WorldState {
    let mut world_state = WorldState {
        size: world.size,
        map: Vec::with_capacity(world.size.0 * world.size.1),
    };

    for y in 0..world.size.1 {
        for x in 0..world.size.0 {
            let pos = Point(x, y);
            let cell_type = world.get_cell(&pos);
            let cell = Cell {
                position: pos,
                cell_type: cell_type.clone(),
            };

            let entity = commands.spawn((
                cell,
                SpriteBundle {
                    sprite: Sprite {
                        color: match cell_type.clone() {
                            ELECTRON(_) => Color::YELLOW,
                            WIRE(_) => Color::BLACK,
                            TAIL(_) => Color::RED,
                            _others => Color::LIME_GREEN,
                        },
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        Vec3::new(
                            CELL_SIZE.mul(x as f32),
                            -CELL_SIZE.mul(y as f32),
                            0.,
                        )
                    ),
                    ..default()
                },
            )).id();

            world_state.map.push(entity);
        }
    }

    for x in 0..world.size.0 {
        for y in 0..world.size.1 {
            let point = Point(x, y);
            match world.get_cell(&point).clone() {
                ELECTRON(_) => {
                    commands.entity(world_state.map[world.index(&point)])
                        .insert(NextUpdate::default());
                    let cells_around = world_state.get_cells_around(&point);

                    for cell in cells_around.iter() {
                        commands.entity(*cell).insert(NextUpdate::default());
                    }
                }
                TAIL(_) => {
                    commands.entity(world_state.map[world.index(&point)])
                        .insert(NextUpdate::default());
                }
                _others => {}
            }
        }
    }

    let field_size = Vec2::new(world.size.0 as f32 * CELL_SIZE, world.size.1 as f32 * CELL_SIZE);
    if let Some(exercise) = world.exercises.get(0) {
        spawn_exercise(exercise, commands, asset_server, field_size);
    }

    world_state
}

fn spawn_exercise(
    exercise: &ExerciseData,
    mut commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    field_size: Vec2,
) {
    let mut exercise_entity = commands.spawn(
        (
            Exercise {
                ticks: 0,
                timeout: exercise.timeout,
            },
            SpatialBundle::default(),
        )
    );
    for spawn in exercise.spawns.iter() {
        exercise_entity.with_children(|parrent| {
            parrent.spawn(ElectronSpawn {
                position: spawn.0.clone(),
                instant: spawn.1,
            });
        });
    }

    for output in exercise.outputs.iter() {
        exercise_entity.with_children(|parrent| {
            parrent.spawn(ExpectedOutput {
                position: output.0.clone(),
                from: output.1,
                until: output.2,
                status: Inactive,
            });
        });
    }

    exercise_entity.with_children(|parrent| {
        spawn_description(parrent, exercise.description.clone(), asset_server, field_size);
    });
}

fn spawn_description(
    parrent: &mut ChildBuilder,
    text: String,
    asset_server: &Res<AssetServer>,
    field_size: Vec2,
) {
    const FONT_SIZE: f32 = 32.0;
    let lines = text.lines().count() as f32;

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: FONT_SIZE,
        color: Color::WHITE,
    };

    parrent.spawn(Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(
                text,
                text_style.clone(),
            )],
            alignment: TextAlignment::Center,
            linebreak_behavior: BreakLineOn::NoWrap,
        },
        text_2d_bounds: Text2dBounds {
            size: Vec2::new(field_size.x, FONT_SIZE * lines),
        },
        text_anchor: BottomCenter,
        transform: Transform::from_translation(Vec3::new(field_size.x / 2.0, FONT_SIZE, 1.0)),
        ..default()
    });
}