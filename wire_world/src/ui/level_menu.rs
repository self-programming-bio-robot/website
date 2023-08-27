use std::time::Duration;
use bevy::prelude::*;

use bevy_tweening::{Animator, EaseFunction, RepeatStrategy, Tween};

use bevy_tweening::lens::UiPositionLens;
use crate::{GameState, LevelDescription, LEVELS};
use crate::control::ExitGame;
use crate::ui::component::{ButtonState, LevelMenuUI, LevelsListNode, MenuActions};

use crate::world::resources::{LevelConfig};


pub fn spawn_level_menu(
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

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(25.0),
                top: Val::Px(25.0),
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        ButtonState::default(),
        MenuActions::Close,
        LevelMenuUI::default(),
    )).with_children(|builder| {
        builder.spawn(AtlasImageBundle {
            texture_atlas: texture_atlas_handle.clone(),
            texture_atlas_image: UiTextureAtlasImage {
                index: 10,
                flip_y: false,
                flip_x: false,
            },
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        });
    });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            LevelMenuUI::default(),
        ))
        .with_children(|builder| {
            let text_style = TextStyle {
                font: font.clone(),
                font_size: 128.0,
                color: Color::WHITE,
            };

            builder.spawn(TextBundle::from_section(
                "Level Menu",
                text_style.clone(),
            ));

            builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            }).with_children(|builder| {
                builder.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(300.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    background_color: Color::rgb(0.6, 0.6, 0.6).into(),
                    ..default()
                }).with_children(|builder| {
                    builder.spawn((
                        ButtonBundle {
                            style: Style {
                                flex_basis: Val::Px(64.0),
                                flex_shrink: 0.0,
                                height: Val::Px(300.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ButtonState::default(),
                        MenuActions::Scroll(1),
                    )).with_children(|builder| {
                        builder.spawn(AtlasImageBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            texture_atlas_image: UiTextureAtlasImage {
                                index: 1,
                                flip_y: false,
                                flip_x: true,
                            },
                            transform: Transform::from_scale(Vec3::splat(2.0)),
                            ..default()
                        });
                    });
                    builder.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(80.0),
                            flex_grow: 1.0,
                            flex_shrink: 1.0,
                            height: Val::Px(300.0),
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Start,
                            justify_content: JustifyContent::Start,
                            overflow: Overflow::clip(),
                            ..default()
                        },
                        ..default()
                    }).with_children(|builder| {
                        builder.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Auto,
                                    left: Val::Px(0.0),
                                    flex_grow: 0.0,
                                    flex_shrink: 0.0,
                                    height: Val::Px(300.0),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Start,
                                    justify_content: JustifyContent::Start,
                                    ..default()
                                },
                                ..default()
                            },
                            LevelsListNode::default(),
                            Animator::new(Tween::new(
                                EaseFunction::CubicOut,
                                Duration::from_millis(100),
                                UiPositionLens {
                                    start: UiRect::left(Val::Px(0.0)),
                                    end: UiRect::left(Val::Px(0.0)),
                                },
                            )),
                        )).with_children(|builder| {
                            for level in LEVELS {
                                spawn_level_button(builder, font.clone(), level);
                            }
                        });
                    });
                    builder.spawn((
                        ButtonBundle {
                            style: Style {
                                flex_basis: Val::Px(64.0),
                                flex_shrink: 0.0,
                                height: Val::Px(300.0),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ButtonState::default(),
                        MenuActions::Scroll(-1),
                    )).with_children(|builder| {
                        builder.spawn(AtlasImageBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            texture_atlas_image: UiTextureAtlasImage {
                                index: 1,
                                flip_y: false,
                                flip_x: false,
                            },
                            transform: Transform::from_scale(Vec3::splat(2.0)),
                            ..default()
                        });
                    });
                });
            });
        });
}

fn spawn_level_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    description: LevelDescription,
) {
    builder.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(250.0),
                flex_shrink: 0.0,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                margin: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            ..default()
        },
        ButtonState::default(),
        MenuActions::Level(description.file_name.to_owned()),
    )).with_children(|builder| {
        let primary_style = TextStyle {
            font: font.clone(),
            font_size: 48.0,
            color: Color::WHITE,
        };
        let secondary_style = TextStyle {
            font: font.clone(),
            font_size: 32.0,
            color: Color::WHITE,
        };

        builder.spawn(TextBundle::from_section(
            description.size,
            secondary_style.clone(),
        ));
        builder.spawn(TextBundle::from_section(
            description.title.to_owned(),
            primary_style.clone(),
        ));
        builder.spawn(TextBundle::from_section(
            format!("{} exercises", description.exercise_count),
            secondary_style.clone(),
        ));
    });
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut ButtonState, &MenuActions),
        (Changed<Interaction>, With<Button>),
    >,
    mut actions: EventWriter<MenuActions>,
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
    mut buttons: Query<(&mut BackgroundColor, &mut ButtonState), With<Button>>,
) {
    for (mut color, button_state) in buttons.iter_mut() {
        match button_state.prev_interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn button_click(
    mut actions: EventReader<MenuActions>,
    mut exit: EventWriter<ExitGame>,
    mut level_config: ResMut<LevelConfig>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level_list: Query<(Entity, &mut Style, &mut Animator<Style>, &Node, &Parent), With<LevelsListNode>>,
    nodes: Query<&Node>,
) {
    for action in actions.iter() {
        info!("{:?}", action);
        match action.clone() {
            MenuActions::Level(file_name) => {
                level_config.level_name = Some(file_name);
                game_state.set(GameState::Level);
            }
            MenuActions::Scroll(delta) => {
                let (_id, level_list, mut animator, node, parent)
                    = level_list.single_mut();
                let parent = nodes.get(parent.get()).unwrap();

                if let Val::Px(px) = level_list.left {
                    let tween = Tween::new(
                        EaseFunction::CubicOut,
                        Duration::from_millis(300),
                        UiPositionLens {
                            start: UiRect::left(level_list.left),
                            end: UiRect::left(Val::Px(px + (delta as f32) * 300.0)),
                        },
                    );
                    if px % 300.0 == 0.0 {
                        if px == 0.0 && delta == 1
                            || -px > node.size().x - parent.size().x && delta == -1 {
                            animator.set_tweenable(tween.with_repeat_count(2)
                                .with_repeat_strategy(RepeatStrategy::MirroredRepeat));
                        } else {
                            animator.set_tweenable(tween);
                        }
                    }
                }
            }
            MenuActions::Close => {
                exit.send(ExitGame);
            }
        }
    }
}