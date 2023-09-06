pub mod components;
pub mod services;

use bevy::prelude::*;
use crate::car::services::{car_physics, car_update};

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, car_physics)
            .add_systems(Update, car_update)
        ;
    }
}
