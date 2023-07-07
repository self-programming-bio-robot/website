pub mod level;

use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Level), level::setup)
            .add_systems(Update, level::button_system.run_if(in_state(GameState::Level)))
            .add_systems(OnExit(GameState::Level), level::button_system)
        ;
    }
}

