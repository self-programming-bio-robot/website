use std::mem::swap;
use std::ops::{Add, Mul};
use std::time::Duration;

use bevy::asset::{AssetEvent, AssetServer};
use bevy::core_pipeline::clear_color::ClearColorConfig::Custom;
use bevy::log::error;
use bevy::prelude::*;
use bevy::sprite::Anchor::{BottomCenter, TopCenter};
use bevy::text::{BreakLineOn, Text2dBounds};
use bevy_tweening::{Animator, EaseFunction, Lens, RepeatStrategy, Tween};

use crate::control::{ClickEvent, MoveCamera};
use crate::{GameState, LevelState};
use crate::world::CELL_SIZE;
use crate::world::components::{Cell, Change, ChangeExercise, ElectronSpawn, Exercise, ExpectedOutput, NextUpdate, OutputStatus, Point};
use crate::world::components::CellType::{ELECTRON, EMPTY, TAIL, WIRE};
use crate::world::components::OutputStatus::{Fail, Inactive, Success, Waiting};
use crate::world::resources::{Counter, ExerciseData, LevelConfig, World, WorldState};
use crate::world::tweens::{blink_background, Camera2dLens};

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
    mut events: EventWriter<ChangeExercise>,
) {
    for event in levels_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(level) = levels.get(handle) {
                    let world_state = spawn_level(level, &mut commands, &mut events);
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
            let cell_type = changed.0.clone();
            sprite.color = cell_type.clone().base_color();
            cell.cell_type = cell_type.clone();

            match cell_type {
                ELECTRON(_) => {
                    commands.entity(world.map[world.index(&cell.position)])
                        .insert(NextUpdate::default());
                    let cells_around = world.get_cells_around(&cell.position);

                    for cell in cells_around.iter() {
                        commands.entity(*cell).insert(NextUpdate::default());
                    }
                }
                TAIL(_) => {
                    commands.entity(world.map[world.index(&cell.position)])
                        .insert(NextUpdate::default());
                }
                _others => {}
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
            if world.lock { continue; }

            let x = (event.pos.x + half_cell_size) / CELL_SIZE;
            let y = (-event.pos.y + half_cell_size) / CELL_SIZE;

            if x < 0. || x >= world.size.0 as f32 || y < 0. || y >= world.size.1 as f32 {
                continue;
            }

            let x = x.trunc() as usize;
            let y = y.trunc() as usize;
            let cell = world.get_cell(&Point(x, y));
            if let Ok(cell_type) = cells.get(cell) {
                commands.entity(cell).insert(Change(
                    match event.button {
                        MouseButton::Left => match cell_type.cell_type.clone() {
                            WIRE(false) => EMPTY(false),
                            EMPTY(false) => WIRE(false),
                            other => other
                        },
                        _others => match (cell_type.cell_type.clone(), world.electron_available) {
                            (WIRE(fixed), true) => ELECTRON(fixed),
                            other => other.0
                        },
                    })
                );
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
    mut commands: Commands,
    mut counter: ResMut<Counter>,
    mut events: EventWriter<ChangeExercise>,
    outputs: Query<&ExpectedOutput>,
    exercises: Query<(Entity, &Exercise), Changed<Exercise>>,
    camera: Query<Entity, &Camera2d>,
    mut world: Option<ResMut<WorldState>>,
    mut level_state: ResMut<NextState<LevelState>>,
) {
    if let Ok((exercise_id, exercise)) = exercises.get_single() {
        if let Some(mut world) = world {
            let camera = camera.single();

            let statues: Vec<OutputStatus> = outputs.iter()
                .map(|output| output.status.clone()).collect();
            debug!("statues: {:?}", statues);
            if outputs.iter().any(|o| o.status == Fail) || exercise.ticks > exercise.timeout {
                info!("Fail exercise");
                commands.entity(exercise_id).despawn_recursive();
                commands.entity(camera)
                    .insert(blink_background(
                        Duration::from_millis(500),
                        Color::DARK_GRAY,
                        Color::RED,
                    ));
                counter.timer.pause();
                events.send(ChangeExercise(0));
                world.lock = false;
            }

            if outputs.iter().all(|o| o.status == Success) {
                info!("Success exercise");
                commands.entity(exercise_id).despawn_recursive();
                commands.entity(camera)
                    .insert(blink_background(
                        Duration::from_millis(500),
                        Color::DARK_GRAY,
                        Color::LIME_GREEN,
                    ));
                if exercise.id + 1 < world.exercises.len() {
                    events.send(ChangeExercise(exercise.id + 1));
                } else {
                    level_state.set(LevelState::Finish);
                }
                world.lock = false;
            }
        }
    }
}

pub fn outputs_indication(
    outputs: Query<&ExpectedOutput>,
    world: Option<Res<WorldState>>,
    mut cells: Query<(&mut Cell, &mut Sprite)>,
    mut time: Local<i32>,
    exercises: Query<&Exercise>,
) {
    if let Some(world) = world {
        if let Ok(exercise) = exercises.get_single() {
            *time = (*time + 1) % 30;
            for output in outputs.iter() {
                if exercise.ticks >= output.from && exercise.ticks < output.until {
                    let cell = world.get_cell(&output.position);
                    if let Ok((cell, mut sprite)) = cells.get_mut(cell) {
                        sprite.color = if *time < 15 {
                            cell.cell_type.clone().base_color()
                        } else {
                            Color::MIDNIGHT_BLUE
                        }
                    }
                }
            }
        }
    }
}

pub fn change_exercise(
    mut events: EventReader<ChangeExercise>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world: Option<Res<WorldState>>,
    exercises: Query<Entity, With<Exercise>>,
) {
    if let Some(world) = world {
        if let Some(ChangeExercise(exercise_id)) = events.iter().next() {
            info!("Spawn exercise {}", exercise_id);
            for exercise in exercises.iter() {
                commands.entity(exercise).despawn_recursive();
            }

            let field_size = Vec2::new(
                world.size.0 as f32 * CELL_SIZE,
                world.size.1 as f32 * CELL_SIZE,
            );
            let exercise = world.exercises.get(*exercise_id).unwrap();

            let mut exercise_entity = commands.spawn(
                (
                    Exercise {
                        id: exercise_id.clone(),
                        ticks: 0,
                        timeout: exercise.timeout,
                    },
                    SpatialBundle::default(),
                )
            );
            for spawn in exercise.spawns.iter() {
                exercise_entity.with_children(|parent| {
                    parent.spawn(ElectronSpawn {
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

            exercise_entity.with_children(|parent| {
                spawn_description(parent, exercise.description.clone(), &asset_server, field_size);
            });
        }
    }
}

fn spawn_level(
    world: &World,
    mut commands: &mut Commands,
    mut events: &mut EventWriter<ChangeExercise>,
) -> WorldState {
    let mut world_state = WorldState {
        size: world.size,
        map: Vec::with_capacity(world.size.0 * world.size.1),
        exercises: world.exercises.clone(),
        electron_available: world.electron_available,
        lock: false,
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
                        color: cell_type.base_color(),
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

    if world.exercises.len() > 0 {
        events.send(ChangeExercise(0));
    }

    world_state
}

fn spawn_description(
    parent: &mut ChildBuilder,
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

    parent.spawn(Text2dBundle {
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