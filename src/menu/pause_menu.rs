use super::common::*;
use bevy::{app::AppExit, prelude::*};

#[derive(Component)]
pub struct OnPauseMenuScreen;

#[derive(Component)]
pub struct PauseMenuCamera;

pub fn pause_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let logo = asset_server.load("pic/logo.png");

    // Semi-transparent overlay
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                z_index: ZIndex::Global(1000), // Выше AI turn screen
                ..default()
            },
            OnPauseMenuScreen,
        ))
        .with_children(|parent| {
            // Main container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Logo
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(300.0),
                            margin: UiRect::bottom(Val::Px(30.0)),
                            ..default()
                        },
                        image: UiImage::new(logo),
                        ..default()
                    });

                    // Pause title
                    parent.spawn(TextBundle::from_section(
                        "GAME PAUSED",
                        TextStyle {
                            font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                            font_size: 36.0,
                            color: TEXT_COLOR,
                        }
                    ).with_style(Style {
                        margin: UiRect::bottom(Val::Px(30.0)),
                        ..default()
                    }));

                    let button_style = Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    };

                    let button_text_style = TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 24.0,
                        color: TEXT_COLOR,
                    };

                    // Resume button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Resume,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resume",
                                button_text_style.clone(),
                            ));
                        });

                    // Back to Main Menu button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                button_text_style.clone(),
                            ));
                        });

                    // Quit button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit Game",
                                button_text_style,
                            ));
                        });
                });
        });
}

pub fn pause_menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Resume => {
                    game_state.set(GameState::Game);
                }
                MenuButtonAction::BackToMenu => {
                    println!("DEBUG: Pause menu BackToMenu pressed");
                    game_state.set(GameState::Menu);
                    // НЕ устанавливаем menu_state - это сделает set_main_menu_state
                    // menu_state.set(MenuState::Main);
                }
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                _ => {}
            }
        }
    }
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<OnPauseMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn pause_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Paused), pause_menu_setup)
        .add_systems(
            Update,
            (pause_menu_action, button_system).run_if(in_state(GameState::Paused)),
        )
        .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
}