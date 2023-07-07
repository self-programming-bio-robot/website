use bevy::app::App;
use bevy::prelude::*;
use bevy::prelude::MouseButton;
use crate::GameState;

pub struct ControlPlugin;

#[derive(Event)]
pub struct ClickEvent {
    pub pos: Vec2,
    pub button: MouseButton,
}

#[derive(Event)]
pub struct MoveCamera {
    pub pos: Vec2,
    pub force: bool,
}

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<MoveCamera>()
            .add_systems(Update, (
                handle_click, set_camera_position
            ).run_if(in_state(GameState::Level)))
        ;
    }
}

pub fn handle_click(
    mut events: Res<Input<MouseButton>>,
    mut click_events: EventWriter<ClickEvent>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        for button in events.get_pressed() {
            click_events.send(ClickEvent {
                pos: world_position,
                button: button.clone()
            });
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