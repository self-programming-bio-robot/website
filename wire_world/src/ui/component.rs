use bevy::prelude::{Component, Interaction};

#[derive(Component, Debug, PartialEq)]
pub enum LevelActions {
    Menu,
    Pause,
    Play(f32),
}

#[derive(Component, Debug, Default)]
pub struct ButtonState {
    pub prev_interaction: Interaction,
}