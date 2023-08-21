use bevy::prelude::*;

#[derive(Component, Event, Debug, PartialEq, Clone)]
pub enum LevelActions {
    Menu,
    Pause,
    Play(f32),
    Restart,
}

#[derive(Component, Debug, Default)]
pub struct ButtonState {
    pub prev_interaction: Interaction,
}

#[derive(Component, Default)]
pub struct LevelUI;

#[derive(Component, Default)]
pub struct LevelFinishUI;