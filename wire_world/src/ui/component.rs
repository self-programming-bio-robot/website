use bevy::prelude::*;

#[derive(Component, Event, Debug, PartialEq, Clone)]
pub enum LevelActions {
    Menu,
    Pause,
    Play(f32),
    Restart,
}

#[derive(Component, Event, Debug, PartialEq, Clone)]
pub enum MenuActions {
    Level(String),
    Scroll(i32),
    Close,
}

#[derive(Component, Debug, Default)]
pub struct ButtonState {
    pub prev_interaction: Interaction,
}

#[derive(Component, Default)]
pub struct LevelUI;

#[derive(Component, Default)]
pub struct LevelFinishUI;

#[derive(Component, Default)]
pub struct LevelMenuUI;

#[derive(Component, Default)]
pub struct LevelsListNode;