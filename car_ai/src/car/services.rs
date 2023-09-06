use bevy::math::{Vec2, Vec3, Vec3Swizzles};
use bevy::prelude::{Color, Gizmos, Quat, Query, Transform};
use crate::car::components::Car;

pub fn car_physics(
    mut cars: Query<&mut Car>,
    mut gizmos: Gizmos,
) {
    for mut car in cars.iter_mut() {
        // todo: extract to debug method
        gizmos.line_2d(
            car.position,
            car.position + car.direction * car.size.x,
            Color::GREEN
        );
        gizmos.line_2d(
            car.position,
            car.position + car.velocity * 10.0,
            Color::RED
        );
        let mut velocity = car.velocity;
        velocity *= car.direction * car.friction;
        car.direction = car.direction.lerp(velocity.normalize(), 0.05);
        car.velocity = velocity;
        car.position += velocity;
    }
}

pub fn car_update(
    mut cars: Query<(&Car, &mut Transform)>,
) {
    for (car, mut transform) in cars.iter_mut() {
        transform.translation = Vec3::new(car.position.x, car.position.y, 0.0);
        transform.rotation = Quat::from_rotation_arc_2d(Vec2::new(1.0, 0.0), car.direction);
    }
}