use std::time::Duration;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui::widget::UiImageSize;
use crate::control::ExitGame;
use crate::ui::component::{ButtonState, LevelActions};
use crate::world::resources::Counter;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut ButtonState, &LevelActions),
        (Changed<Interaction>, With<Button>),
    >,
    mut counter: ResMut<Counter>,
    mut exit: EventWriter<ExitGame>
) {
    for (interaction, mut color, mut state, action)
    in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                if (state.prev_interaction == Interaction::Pressed) {
                    button_click(action, counter.as_mut(), &mut exit);
                }
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
        state.prev_interaction = interaction.clone();
    }
}

fn button_click(
    action: &LevelActions,
    counter: &mut Counter,
    exit: &mut EventWriter<ExitGame>
) {
    match action {
        LevelActions::Menu => {
            info!("goto menu");
            exit.send(ExitGame);
        },
        LevelActions::Pause => {
            counter.timer.pause();
            info!("Pause");
        },
        LevelActions::Play(speed) => {
            counter.timer.unpause();
            counter.timer.set_duration(Duration::from_secs_f32(*speed));
            info!("set speed {}", speed);
        },
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
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Start,
                ..default()
            },
            ..default()
        })
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
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
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