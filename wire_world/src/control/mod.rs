
use bevy::app::{App, AppExit};
use bevy::prelude::*;
use bevy::prelude::MouseButton;
use bevy::utils::HashMap;
use crate::{GameState, LevelState};

pub struct ControlPlugin;

#[derive(Event)]
pub struct ClickEvent {
    pub pos: Vec2,
    pub button: MouseButton,
}

#[derive(Default)]
pub struct MouseButtonsState {
    pub left: bool,
    pub right: bool,
    pub moved: HashMap<MouseButton, bool>,
    pub started_from: HashMap<MouseButton, Vec2>,
}

#[derive(Event)]
pub struct MoveCamera {
    pub pos: Vec2,
    pub force: bool,
}

#[derive(Event)]
pub struct ExitGame;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<MoveCamera>()
            .add_event::<ExitGame>()
            .add_systems(Update, (
                handle_click, set_camera_position
            ).run_if(in_state(GameState::Level).and_then(in_state(LevelState::Process))))
        ;
    }
}

pub fn handle_click(
    events: Res<Input<MouseButton>>,
    mut click_events: EventWriter<ClickEvent>,
    windows: Query<&Window>,
    mut camera_q: Query<(&Camera, &GlobalTransform, &mut Transform)>,
    mut prev_state: Local<MouseButtonsState>,
) {
    let (camera, camera_transform, mut transform) = camera_q.single_mut();
    let window = windows.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for button in events.get_just_released() {
            if !prev_state.moved.get(button).unwrap_or(&false) {
                click_events.send(ClickEvent {
                    pos: world_position,
                    button: button.clone()
                });
            }
        }
        for button in events.get_just_pressed() {
            prev_state.started_from.insert(button.clone(), world_position);
            prev_state.moved.insert(button.clone(), false);
        }
        for button in events.get_pressed() {
            let dt = prev_state.started_from.get(button)
                .map(|from| *from - world_position)
                .unwrap_or(Vec2::default());

            if dt.length() > 2.0 {
                transform.translation += Vec3::from((dt, 0.));
                prev_state.moved.insert(button.clone(), true);
            }

            prev_state.started_from.insert(button.clone(), world_position + dt);
        }
    }
}

pub fn set_camera_position(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    mut events: EventReader<MoveCamera>,
) {
    for event in events.iter() {
        let mut camera_transform = camera_q.single_mut();

        camera_transform.translation += Vec3::from((event.pos, 0.));
    }
}

pub fn handle_exit(
    mut exit_game_events: EventReader<ExitGame>,
    mut exit_events: EventWriter<AppExit>,
) {
    for _ in exit_game_events.iter() {
        exit_events.send(AppExit::default())
    }
}