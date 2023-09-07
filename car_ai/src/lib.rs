pub mod car;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use crate::car::CarPlugin;
use crate::car::components::{CAR_SIZE, CarBundle, KeyboardController};

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(CarPlugin)
            .add_systems(Startup, init)
        ;
    }
}

impl MainPlugin {
    pub fn start() {
        App::new()
            .add_plugins(MainPlugin)
            .run();
    }
}

pub fn init(
    mut commands: Commands,
) {
    info!("start");
    commands.spawn(
        (
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: ClearColorConfig::Custom(Color::DARK_GRAY)
                },
                projection: OrthographicProjection {
                    scale: 0.01,
                    ..default()
                },
                ..default()
            },
        ));
    commands.spawn((
        CarBundle::create(Vec2::new(0.0, 0.0), Color::BLUE),
        KeyboardController {},
    ));
}