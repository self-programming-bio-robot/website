use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use crate::car::components::{AIR_FRICTION, Car, CAR_SIZE, KeyboardController, WHEEL_FRICTION};

pub fn car_physics(
    mut cars: Query<&mut Car>,
    mut gizmos: Gizmos,
    time: Res<Time>,
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
            car.position + car.velocity,
            Color::RED
        );
        let velocity = car.velocity.length();
        let f_acceleration: Vec2 = if car.acceleration > 0.0 {
            car.direction * car.acceleration * car.engine_power
        } else if car.brake > 0.0 {
            car.direction * car.brake * -car.brakes_power
        } else {
            Vec2::new(0.0, 0.0)
        };

        let dt = time.delta().as_secs_f32();

        let f_air = -AIR_FRICTION * car.velocity * velocity ;
        let f_wheel = -WHEEL_FRICTION * car.velocity;
        let force = f_acceleration + f_air + f_wheel;
        let a = force / car.mass;
        let a = a * dt;

        if velocity < a.length() && car.brake > 0.0 { // todo: add reverse
            car.velocity = Vec2::new(0.0, 0.0);
        } else {
            car.velocity = car.velocity + a;
        }
        car.velocity = car.direction * car.velocity.length();

        if car.steering_wheel_angle != 0.0 {
            let l = 0.8;
            let r = (car.size.x * l) / (car.steering_wheel_angle * car.max_eversion).sin();
            let w = car.velocity.length() / r;
            let w = w * dt;
            car.direction = Vec2::from_angle(w).rotate(car.direction);

            let perp = car.direction.perp() * car.steering_wheel_angle;
            gizmos.line_2d(
                car.position,
                car.position + perp * 100.0,
                Color::YELLOW
            );
            let fw = car.position + car.direction * car.size.x * l / 2.0;
            gizmos.line_2d(
                fw,
                fw + Vec2::from_angle(car.steering_wheel_angle * car.max_eversion)
                    .rotate(perp) * 100.0,
                Color::YELLOW
            );
        }

        car.position = car.position + car.velocity * dt;
        car.acceleration = 0.0;
        car.brake = 0.0;
        car.steering_wheel_angle = 0.0;
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

pub fn car_controller(
    mut cars: Query<&mut Car, With<KeyboardController>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut car in cars.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            car.acceleration = 1.0;
        } else if keyboard_input.pressed(KeyCode::Down) {
            car.brake = 1.0;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            car.steering_wheel_angle = 1.0;
        } else if keyboard_input.pressed(KeyCode::Right) {
            car.steering_wheel_angle = -1.0;
        }
    }
}