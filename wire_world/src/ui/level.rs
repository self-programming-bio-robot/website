
use std::time::Duration;

use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::control::ExitGame;
use crate::LevelState;
use crate::ui::component::{ButtonState, LevelActions, LevelFinishUI, LevelUI};
use crate::world::components::ChangeExercise;
use crate::world::resources::{Counter, WorldState};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const SELECTED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut ButtonState, &LevelActions),
        (Changed<Interaction>, With<Button>),
    >,
    mut actions: EventWriter<LevelActions>,
) {
    for (interaction, _color, mut state, action)
    in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                if state.prev_interaction == Interaction::Pressed {
                    actions.send(action.clone());
                }
            }
            _other => {}
        }
        state.prev_interaction = interaction.clone();
    }
}

pub fn button_state(
    mut buttons: Query<(&mut BackgroundColor, &mut ButtonState, &LevelActions), With<Button>>,
    counter: Res<Counter>,
) {
    for (mut color, button_state, action) in buttons.iter_mut() {
        match button_state.prev_interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();

                if *action == LevelActions::Pause && counter.timer.paused() {
                    *color = SELECTED_BUTTON.into();
                } else if let LevelActions::Play(speed) = action {
                    if !counter.timer.paused() && *speed == counter.timer.duration().as_secs_f32() {
                        *color = SELECTED_BUTTON.into();
                    }
                }
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let buttons_handle = asset_server.load("ui/buttons.png");
    let texture_atlas =
        TextureAtlas::from_grid(buttons_handle, Vec2::new(16.0, 16.0),
                                6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    align_self: AlignSelf::Start,
                    ..default()
                },
                ..default()
            },
            LevelUI::default(),
        ))
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Auto,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent| {
                spawn_button(parent, texture_atlas_handle.clone(), 0, LevelActions::Menu);
            });
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        column_gap: Val::Px(5.),
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent| {
                spawn_button(parent, texture_atlas_handle.clone(), 1, LevelActions::Play(1.));
                spawn_button(parent, texture_atlas_handle.clone(), 2, LevelActions::Pause);
                spawn_button(parent, texture_atlas_handle.clone(), 3, LevelActions::Play(0.5));
                spawn_button(parent, texture_atlas_handle.clone(), 4, LevelActions::Play(0.125));
            });
        });
}

pub fn setup_finish_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let buttons_handle = asset_server.load("ui/buttons.png");
    let texture_atlas =
        TextureAtlas::from_grid(buttons_handle, Vec2::new(16.0, 16.0),
                                6, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 32.0,
        color: Color::WHITE,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::BLACK.with_a(0.4).into(),
                ..default()
            },
            LevelFinishUI::default(),
        ))
        .with_children(|parent| {
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(260.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::DARK_GREEN.into(),
                    ..default()
                }
            ).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "SUCCESS",
                            text_style.clone(),
                        )],
                        alignment: TextAlignment::Center,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    ..default()
                });
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(180.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    spawn_button(parent, texture_atlas_handle.clone(), 9, LevelActions::Restart);
                    spawn_button(parent, texture_atlas_handle.clone(), 11, LevelActions::Menu);
                });
            });
        });
}

pub fn delete_ui<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn button_click(
    mut actions: EventReader<LevelActions>,
    mut counter: ResMut<Counter>,
    mut exit: EventWriter<ExitGame>,
    world: Option<ResMut<WorldState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    mut events: EventWriter<ChangeExercise>,
) {
    if let Some(mut world) = world {
        for action in actions.iter() {
            match action {
                LevelActions::Menu => {
                    info!("goto menu");
                    exit.send(ExitGame);
                }
                LevelActions::Pause => {
                    counter.timer.pause();
                    info!("Pause");
                }
                LevelActions::Play(speed) => {
                    counter.timer.unpause();
                    counter.timer.set_duration(Duration::from_secs_f32(*speed));
                    world.lock = world.exercises.len() > 0;
                    info!("set speed {}", speed);
                }
                LevelActions::Restart => {
                    level_state.set(LevelState::Process);
                    counter.timer.pause();
                    events.send(ChangeExercise(0));
                    info!("Reload");
                }
            }
        }
    }
}

fn spawn_button(
    parent: &mut ChildBuilder,
    atlas_handle: Handle<TextureAtlas>,
    index: usize,
    action: LevelActions,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(65.0),
                    height: Val::Px(65.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            ButtonState::default(),
            action
        ))
        .with_children(|parent| {
            parent.spawn(AtlasImageBundle {
                texture_atlas: atlas_handle,
                texture_atlas_image: UiTextureAtlasImage {
                    index,
                    flip_x: false,
                    flip_y: false,
                },
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    ..default()
                },
                ..default()
            });
        });
}