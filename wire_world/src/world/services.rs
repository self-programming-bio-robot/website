use std::ops::Mul;
use bevy::asset::{AssetEvent, AssetServer};
use bevy::log::error;
use bevy::prelude::{Added, Assets, Camera2dBundle, Changed, Color, Commands, default, Entity, EventReader, EventWriter, Handle, info, MouseButton, NextState, Query, Res, ResMut, Sprite, SpriteBundle, Time, Transform, Vec2, Vec3, With};
use crate::control::{ClickEvent, MoveCamera};
use crate::GameState;
use crate::world::CELL_SIZE;
use crate::world::components::{Cell, Change, NextUpdate, Point};
use crate::world::components::CellType::{ELECTRON, EMPTY, TAIL, WIRE};
use crate::world::resources::{Counter, LevelConfig, World, WorldState};

pub fn init_level(
    mut next_state: ResMut<NextState<GameState>>,
    level_config: Res<LevelConfig>,
    assets: Res<AssetServer>,
) {
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
) {
    for event in levels_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(level) = levels.get(handle) {
                    let world_state = spawn_level(level, &mut commands);
                    let pos = Vec2::new(
                        CELL_SIZE * level.size.0 as f32,
                        -CELL_SIZE * level.size.1 as f32,
                    ) * 0.5;
                    camera_events.send(MoveCamera {
                        pos,
                        force: true
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
) {
    if let Some(world) = world {
        let timer = counter.timer.tick(time.delta());
        if timer.finished() {
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

fn spawn_level(
    world: &World,
    mut commands: &mut Commands,
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

    world_state
}