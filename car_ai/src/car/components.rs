use bevy::math::Vec2;
use bevy::prelude::*;

pub const CAR_SIZE: Vec2 = Vec2::new(100.0, 40.0);

#[derive(Component)]
pub struct Car {
    velocity: Vec2,
    max_speed: f32,
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
        CarBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(CAR_SIZE.clone()),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
                ..default()
            },
            car: Car {
                velocity: Vec2::default(),
                max_speed: 40.0,
            }
        }
    }
}