pub mod world;
pub mod control;
pub mod ui;

use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands, NextState, Plugin, ResMut, States};
use crate::control::ControlPlugin;
use crate::ui::UiPlugin;
use crate::world::resources::LevelConfig;
use crate::world::WorldPlugin;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    LevelsList,
    Level,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_state::<GameState>()
            .add_plugins(WorldPlugin)
            .add_plugins(ControlPlugin)
            .add_plugins(UiPlugin)
            .add_startup_system(init);
    }
}

pub fn init(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_config: ResMut<LevelConfig>,
) {
    commands.spawn(Camera2dBundle::default());

    level_config.level_name = Some("level1.level".to_owned());
    next_state.set(GameState::Level);
}

