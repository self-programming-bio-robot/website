pub mod level;
pub mod component;

use bevy::app::App;
use bevy::prelude::*;
use crate::GameState;
use crate::ui::level::{button_state, button_system};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Level), level::setup)
            .add_systems(Update, (
                button_state.after(button_system),
                button_system,
            ).run_if(in_state(GameState::Level)))
            .add_systems(OnExit(GameState::Level), level::button_system)
        ;
    }
}

