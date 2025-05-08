use bevy::{
    app::AppExit,
    prelude::*,
};

use super::common::*;
use crate::game;

const CRIMSON: Color = Color::rgb(0.86, 0.08, 0.24);

#[derive(Component)]
pub struct OnMainMenuScreen;

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    
    let button_icon_style = Style {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };
    
    let button_text_style = TextStyle {
        font_size: 33.0,
        ..default()
    };

    let right_icon = asset_server.load("textures/Game Icons/right.png");
    let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
    let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        OnMainMenuScreen,
    ))
    .with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: CRIMSON.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Bevy Game Menu UI",
                    TextStyle {
                        font_size: 67.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            ));
            
            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Play,
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: button_icon_style.clone(),
                    image: right_icon.into(),
                    ..default()
                });
                parent.spawn(TextBundle::from_section(
                    "New Game",
                    TextStyle {
                        font_size: button_text_style.font_size,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));
            });
            
            parent.spawn((
                ButtonBundle {
                    style: button_style.clone(),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Settings,
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: button_icon_style.clone(),
                    image: wrench_icon.into(),
                    ..default()
                });
                parent.spawn(TextBundle::from_section(
                    "Settings",
                    TextStyle {
                        font_size: button_text_style.font_size,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));
            });
            
            parent.spawn((
                ButtonBundle {
                    style: button_style,
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                MenuButtonAction::Quit,
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: button_icon_style,
                    image: exit_icon.into(),
                    ..default()
                });
                parent.spawn(TextBundle::from_section(
                    "Quit",
                    TextStyle {
                        font_size: button_text_style.font_size,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ));
            });
        });
    });
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    menu_state.set(MenuState::Disabled);
                    game_state.set(GameState::Game);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}

pub fn main_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>);
} 