use std::f32::consts::PI;
use bevy::math::Vec2;
use bevy::prelude::*;

pub const CAR_SIZE: Vec2 = Vec2::new(5.0, 2.0);
pub const AIR_FRICTION: f32 = 0.42;
pub const WHEEL_FRICTION: f32 = AIR_FRICTION * 30.0;

#[derive(Component)]
pub struct Car {
    pub size: Vec2,
    pub mass_center: Vec2,
    pub mass: f32,
    pub position: Vec2,
    pub direction: Vec2,
    pub velocity: Vec2,
    pub torque: f32,
    pub friction: Vec2,
    pub engine_power: f32,
    pub brakes_power: f32,
    pub max_eversion: f32,

    pub acceleration: f32,
    pub brake: f32,
    pub steering_wheel_angle: f32,

    pub applied_forces: Vec<(Vec2, Vec2)>,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            size: CAR_SIZE,
            mass: 1500.0,
            mass_center: Vec2::default(),
            position: Vec2::default(),
            velocity: Vec2::default(),
            torque: 0.0,
            direction: Vec2::new(1.0, 0.0),
            friction: Vec2::new(0.99, 0.3),
            engine_power: 5000.0,
            brakes_power: 10000.0,
            max_eversion: PI / 8.0,
            acceleration: 0.0,
            brake: 0.0,
            steering_wheel_angle: 0.0,
            applied_forces: Vec::new(),
        }
    }
}

impl Car {

    pub fn add_force(&mut self, force: Vec2, at: Vec2) {
        self.applied_forces.push((force, at));
    }

    pub fn add_force_at_center(&mut self, force: Vec2) {
        self.add_force(force, Vec2::new(0.0, 0.0));
    }
}

#[derive(Bundle)]
pub struct CarBundle {
    pub sprite: SpriteBundle,
    pub car: Car,
}

#[derive(Component)]
pub struct KeyboardController {

}

impl CarBundle {
    pub fn create(position: Vec2, color: Color) -> CarBundle {
        let car = Car {
            position,
            ..default()
        };
        CarBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(car.size),
                    ..default()
                },
                ..default()
            },
            car
        }
    }
}