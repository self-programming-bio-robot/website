use std::time::Duration;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::GameState;
use crate::world::resources::{Counter, LevelConfig, World, WorldState};
use crate::world::services::{find_cell_to_update, handle_clicks, init_level, load_level, update_cells};
use crate::world::world_loader::WorldLoader;

pub mod components;
pub mod resources;
pub mod services;
pub mod world_loader;

pub const CELL_SIZE: f32 = 40.;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<World>()
            .add_asset_loader(WorldLoader)
            .insert_resource(Counter {
                timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating)
            })
            .insert_resource(LevelConfig::empty())
            .add_systems(OnEnter(GameState::Level), init_level)
            .add_systems(Update, (
                load_level,
                find_cell_to_update,
                update_cells,
                handle_clicks
            ).run_if(in_state(GameState::Level)))
            ;
    }
}