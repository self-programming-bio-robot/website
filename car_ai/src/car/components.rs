use bevy::math::Vec2;
use bevy::prelude::*;

pub const CAR_SIZE: Vec2 = Vec2::new(100.0, 40.0);

#[derive(Component)]
pub struct Car {
    pub size: Vec2,
    pub position: Vec2,
    pub direction: Vec2,
    pub velocity: Vec2,
    pub max_speed: f32,
    pub acceleration: f32,
    pub friction: Vec2,
}

impl Default for Car {
    fn default() -> Self {
        Self {
            size: CAR_SIZE,
            position: Vec2::default(),
            velocity: Vec2::new(10.0, 5.0),
            direction: Vec2::new(1.0, 0.0),
            max_speed: 50.0,
            acceleration: 10.0,
            friction: Vec2::new(0.99, 0.3),

        }
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