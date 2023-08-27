use std::time::Duration;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use crate::{GameState, LevelState};
use crate::world::components::ChangeExercise;
use crate::world::resources::{Counter, LevelConfig, World};
use crate::world::services::*;
use crate::world::world_loader::WorldLoader;

pub mod components;
pub mod resources;
pub mod services;
pub mod world_loader;
pub mod tweens;

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
            .add_event::<ChangeExercise>()
            .add_systems(OnEnter(GameState::Level), init_level)
            .add_systems(OnExit(GameState::Level), destroy_level)
            .add_systems(Update, (
                load_level,
                handle_clicks,
                handle_outputs.after(change_exercise),
                handle_exercises.after(handle_outputs),
                outputs_indication,
                change_exercise,
            ).run_if(in_state(GameState::Level).and_then(in_state(LevelState::Process))))
            .add_systems(Update, (
                find_cell_to_update,
                update_cells,
                spawn_electron,
            ).run_if(in_state(GameState::Level)))
            ;
    }
}