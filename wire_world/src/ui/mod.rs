pub mod level;
pub mod component;

use bevy::app::App;
use bevy::prelude::*;
use crate::{GameState, LevelState};
use crate::ui::component::{LevelActions, LevelFinishUI, LevelUI};
use crate::ui::level::{button_click, button_state, button_system};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<LevelState>()
            .add_event::<LevelActions>()
            .add_systems(OnEnter(GameState::Level), level::setup)
            .add_systems(Update, (
                button_state.after(button_system),
                button_system,
                button_click,
            ).run_if(in_state(GameState::Level)))
            .add_systems(OnExit(GameState::Level), (
                level::delete_ui::<LevelUI>,
                level::delete_ui::<LevelFinishUI>
                ))
            .add_systems(OnExit(LevelState::Process), level::delete_ui::<LevelUI>)
            .add_systems(OnExit(LevelState::Finish), level::delete_ui::<LevelFinishUI>)
            .add_systems(OnEnter(LevelState::Finish), level::setup_finish_screen)
            .add_systems(OnExit(LevelState::Finish), level::setup)
        ;
    }
}

