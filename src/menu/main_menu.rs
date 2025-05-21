use bevy::{app::AppExit, prelude::*};
use super::common::*;

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct WorldModel;

#[derive(Component)]
pub struct MenuCamera;

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world_model = asset_server.load("textures/WorldWar.glb#Scene0");
    let logo = asset_server.load("pic/logo.png");

    // Set background color for the entire scene
    commands.insert_resource(ClearColor(Color::hex("#0A1E3E").unwrap()));

    // Spawn the 3D world model first
    commands.spawn((
        SceneBundle {
            scene: world_model,
            transform: Transform::from_xyz(0.0, 8.0, 0.0)
                .with_scale(Vec3::splat(7.0))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
            ..default()
        },
        WorldModel,
        OnMainMenuScreen,
    ));

    // Add lighting for the 3D model
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 600.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OnMainMenuScreen,
    ));

    // Single 3D camera that can see both 3D model and UI
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 25.0)
                .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
            camera: Camera {
                // Default priority
                ..default()
            },
            ..default()
        },
        MenuCamera,
        OnMainMenuScreen,
    ));

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
        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
        font_size: 33.0,
        color: TEXT_COLOR,
    };

    let right_icon = asset_server.load("textures/Game Icons/right.png");
    let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
    let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");

    // UI root node with transparent background
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                // Transparent background to let the 3D model be visible
                background_color: Color::NONE.into(),
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // Logo container at the top
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        position_type: PositionType::Absolute,
                        top: Val::Px(50.0), // Offset the logo downwards
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                },))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(800.0), // Further increase the logo size
                            margin: UiRect::all(Val::Px(0.0)),
                            ..default()
                        },
                        image: UiImage::new(logo),
                        ..default()
                    });
                });

            // Button container offset to the left
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart, // Align to the left
                        position_type: PositionType::Absolute, // Absolute positioning for buttons
                        left: Val::Percent(5.0), // Shift further to the left
                        top: Val::Vh(40.0), // Vertical position
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
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
                                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                    font_size: button_text_style.font_size,
                                    color: TEXT_COLOR,
                                },
                            ));
                        });

                    parent
                        .spawn((
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
                                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                    font_size: button_text_style.font_size,
                                    color: TEXT_COLOR,
                                },
                            ));
                        });

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
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: exit_icon.into(),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                    font_size: button_text_style.font_size,
                                    color: TEXT_COLOR,
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
    app.add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnEnter(GameState::Menu), main_menu_setup);
}