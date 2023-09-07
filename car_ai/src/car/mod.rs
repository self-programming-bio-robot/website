pub mod components;
pub mod services;

use bevy::prelude::*;
use crate::car::services::{car_controller, car_physics, car_update, mouse_controller};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, car_physics)
            .add_systems(Update, car_update)
            .add_systems(Update, car_controller)
            .add_systems(Update, mouse_controller)
        ;
    }
}
