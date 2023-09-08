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
        let dt = time.delta().as_secs_f32();

        // apply friction by the side
        let velocity = car.velocity;
        let v = velocity.length();
        if v > 0.0 {
            let friction = car.friction * car.mass * 9.8;
            let f1 = car.direction.dot(car.velocity) / v;
            let f1 = f1 * car.direction * friction.x * v;
            let f2 = car.direction.perp().dot(car.velocity) / v;
            let f2 = f2 * car.direction.perp() * friction.y * v;

            car.add_force_at_center(-f1);
            car.add_force_at_center(-f2);
        }

        let mut force = Vec2::default();
        let mut torque: f32 = 0.0;
        for af in car.applied_forces.iter() {
            force += af.1;
            torque += af.1.perp_dot(af.0);
        }
        car.applied_forces.clear();

        let a = force / car.mass;
        car.velocity = car.velocity + a *  dt;
        car.position = car.position + car.velocity * dt;

        let torque = torque / car.mass;
        car.torque = car.torque + torque * dt;
        car.torque = car.torque * 0.98;

        gizmos.line_2d(
            car.position,
            car.position + Vec2::Y * car.torque,
            Color::YELLOW_GREEN
        );

        car.direction = Vec2::from_angle(-car.torque * dt).rotate(car.direction);
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

pub fn mouse_controller(
    events: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut camera_q: Query<(&Camera, &GlobalTransform, &mut Transform)>,
    mut cars: Query<(Entity, &mut Car, &Sprite, &GlobalTransform)>,
    mut start_point: Local<Option<(Entity,Vec2, Vec2)>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform, mut transform) = camera_q.single_mut();
    let window = windows.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for _ in events.get_pressed() {
            if let Some((entity, global, local)) = start_point.take() {
                gizmos.line_2d(
                    global,
                    world_position,
                    Color::PURPLE
                );

                let force = world_position - global;
                let (_, mut car, _, global) = cars.get_mut(entity).unwrap();
                let force = car.mass * force;
                let angle = Vec2::X.angle_between(car.direction);
                let force_local = Vec2::from_angle(-angle).rotate(force);
                car.add_force(force_local, local);

                let local_position = Vec2::from_angle(angle).rotate(local);
                *start_point = Some((entity, global.translation().truncate() + local_position, local));
            }
        }

        for _ in events.get_just_pressed() {
            for (entity, mut car, sprite, global) in cars.iter_mut() {
                let local_position = world_position - global.translation().truncate();
                let angle = Vec2::X.angle_between(car.direction);
                let local_position = Vec2::from_angle(-angle).rotate(local_position);

                if let Some(size) = sprite.custom_size {
                    let size = size / 2.0;
                    if local_position.x < size.x
                        && local_position.x > -size.x
                        && local_position.y < size.y
                        && local_position.y > -size.y {
                        *start_point = Some((entity, world_position.clone(), local_position.clone()));
                    }
                }
            }
        }

        for _ in events.get_just_released() {
            *start_point = None;
        }
    }
}