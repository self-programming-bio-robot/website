pub mod world;
pub mod control;
pub mod ui;


use bevy::app::App;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::DefaultPlugins;
use bevy::prelude::*;

use bevy_tweening::*;
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum LevelState {
    #[default]
    Process,
    Finish,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    canvas: Some("#render".to_string()),
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(TweeningPlugin)
            .add_state::<GameState>()
            .add_plugins(WorldPlugin)
            .add_plugins(ControlPlugin)
            .add_plugins(UiPlugin)
            .add_systems(Startup, init)
            .add_systems(
                Update,
                component_animator_system::<Camera2d>.in_set(AnimationSystem::AnimationUpdate),
            )
        ;
    }
}

pub fn init(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut level_config: ResMut<LevelConfig>,
) {
    commands.spawn(
        (
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::DARK_GRAY)
                },
                ..default()
            },
        ));

    level_config.level_name = Some("level1.level".to_owned());
    next_state.set(GameState::Level);
}

impl GamePlugin {
    pub fn start() {
        App::new()
            .add_plugins(GamePlugin)
            .run();
    }
}