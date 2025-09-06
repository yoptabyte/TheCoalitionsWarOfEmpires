use super::common::*;
use bevy::{app::AppExit, prelude::*};

// Faction selection components
#[derive(Component)]
pub struct FactionSelection;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Faction {
    Entente,
    CentralPowers,
}

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct WorldModel;

#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct LogoContainer;

#[derive(Component)]
pub struct ButtonContainer;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum MenuAnimationState {
    #[default]
    Idle,
    Animating,
}

#[derive(Resource)]
pub struct AnimationTimer {
    pub timer: Timer,
}

pub fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

pub fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world_model = asset_server.load("textures/WorldWar.glb#Scene0");
    let logo = asset_server.load("pic/logo.png");

    commands.insert_resource(ClearColor(Color::hex("#0A1E3E").unwrap()));

    // Spawn the 3D world model first with natural coloring
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

    // Simple, reliable lighting for the 3D model
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1000.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 100.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OnMainMenuScreen,
    ));

    // Add a second light from another angle to ensure visibility
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 800.0,
                shadows_enabled: false,
                ..default()
            },
            transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OnMainMenuScreen,
    ));

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 25.0)
                .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
            camera: Camera { ..default() },
            ..default()
        },
        MenuCamera,
        OnMainMenuScreen,
    ));

    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(55.0),
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
        font_size: 28.0,
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
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(500.0),
                            height: Val::Auto,
                            position_type: PositionType::Absolute,
                            top: Val::Px(55.0), // Offset the logo downwards
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..default()
                    },
                    LogoContainer,
                ))
                .with_children(|parent| {
                    parent.spawn(ImageBundle {
                        style: Style {
                            width: Val::Px(520.0),
                            margin: UiRect::all(Val::Px(0.0)),
                            ..default()
                        },
                        image: UiImage::new(logo),
                        ..default()
                    });
                });

            // Button container offset to the left
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart, // Align to the left
                            position_type: PositionType::Absolute, // Absolute positioning for buttons
                            left: Val::Percent(10.0),              // Slight shift to the left
                            top: Val::Vh(50.0),
                            ..default()
                        },
                        ..default()
                    },
                    ButtonContainer,
                ))
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

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<OnMainMenuScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn animate_menu_transition(
    mut logo_query: Query<&mut Style, With<LogoContainer>>,
    mut button_query: Query<&mut Style, (With<ButtonContainer>, Without<LogoContainer>)>,
    mut world_model_query: Query<&mut Transform, With<WorldModel>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
    mut animation_state: ResMut<NextState<MenuAnimationState>>,
    _menu_state: ResMut<NextState<MenuState>>,
    _game_state: ResMut<NextState<GameState>>,
    windows: Query<&Window>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    faction_query: Query<Entity, With<FactionSelection>>,
) {
    // Update the timer
    timer.timer.tick(time.delta());

    // Get window height for calculations
    let window_height = windows.single().height();

    if let Ok(mut logo_style) = logo_query.get_single_mut() {
        if let Ok(mut button_style) = button_query.get_single_mut() {
            // Calculate animation progress (0.0 to 1.0)
            let progress = timer.timer.fraction();

            // Animate logo down (move from top to bottom) - slower movement
            if let Val::Px(top) = logo_style.top {
                let new_top = top + (window_height * 0.05 * progress);
                logo_style.top = Val::Px(new_top);
            }

            // Animate buttons up (move from middle to top, off-screen) - slower movement
            if let Val::Vh(top) = button_style.top {
                let new_top = top - (15.0 * progress);
                button_style.top = Val::Vh(new_top);
            }
            
            // Rotate the world model on Y-axis
            if let Ok(mut transform) = world_model_query.get_single_mut() {
                // Calculate rotation angle based on progress (180 degrees total rotation)
                let rotation_angle = std::f32::consts::FRAC_PI_3 * -progress;
                
                // Keep the original X rotation and add Y rotation
                let original_rotation = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
                let y_rotation = Quat::from_rotation_x(rotation_angle);
                
                // Combine rotations
                transform.rotation = y_rotation * original_rotation;
            }

            // When animation is complete
            if timer.timer.finished() {
                // Reset animation state
                animation_state.set(MenuAnimationState::Idle);

                // Check if faction selection UI is already spawned
                if faction_query.iter().count() == 0 {
                    // Spawn faction selection UI
                    spawn_faction_selection(&mut commands, &asset_server);
                }

                // Don't transition to game state yet - wait for faction selection
                // The game state transition will happen after faction selection
            }
        }
    }
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut animation_state: ResMut<NextState<MenuAnimationState>>,
    mut animation_timer: ResMut<AnimationTimer>,
    current_animation_state: Res<State<MenuAnimationState>>,
) {
    // Don't process button clicks during animation
    if *current_animation_state.get() == MenuAnimationState::Animating {
        return;
    }

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    // Instead of immediately changing to game state,
                    // start the animation
                    animation_state.set(MenuAnimationState::Animating);
                    animation_timer.timer.reset();
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

// Function to spawn the faction selection UI
pub fn spawn_faction_selection(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let entente_image = asset_server.load("pic/Entente.png");
    let central_powers_image = asset_server.load("pic/CentralPowers.png");

    // Root node for faction selection
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
            },
            FactionSelection,
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // Container for the two faction images
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(90.0),
                        height: Val::Auto,
                        justify_content: JustifyContent::SpaceAround,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    background_color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Entente faction image
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(440.0),
                                height: Val::Px(420.0),
                                margin: UiRect::all(Val::Px(30.0)),
                                ..default()
                            },
                            image: UiImage::new(entente_image),
                            background_color: Color::rgba(1.0, 1.0, 1.0, 0.9).into(),
                            ..default()
                        },
                        Faction::Entente,
                    ));

                    // Central Powers faction image
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(440.0),
                                height: Val::Px(420.0),
                                margin: UiRect::all(Val::Px(30.0)),
                                ..default()
                            },
                            image: UiImage::new(central_powers_image),
                            background_color: Color::rgba(1.0, 1.0, 1.0, 0.9).into(),
                            ..default()
                        },
                        Faction::CentralPowers,
                    ));
                });
        });

    // No longer spawning the world map here - it's already spawned in main_menu_setup
}

// System for faction hover - now adds much brighter highlighting effect when hovering over faction images
pub fn faction_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<Faction>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *background_color = Color::rgba(1.0, 1.0, 1.0, 1.0).into();
            }
            Interaction::None => {
                // Dimmer when not hovering for better contrast
                *background_color = Color::rgba(0.7, 0.7, 0.7, 0.9).into(); // More dimmed
            }
            Interaction::Pressed => {
                // Feedback when pressed
                *background_color = Color::rgba(1.0, 1.0, 1.0, 1.0).into(); // Bright blue glow when pressed
            }
        }
    }
}

// System to handle faction selection
pub fn faction_selection_system(
    mut interaction_query: Query<(&Interaction, &Faction), (Changed<Interaction>, With<Button>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_faction: ResMut<crate::game::units::PlayerFaction>,
) {
    for (interaction, faction) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // When a faction is selected, transition to game state
            menu_state.set(MenuState::Disabled);
            game_state.set(GameState::Game);

            // Store the selected faction in the PlayerFaction resource
            player_faction.0 = *faction;
            info!("Player selected faction: {:?}", faction);
        }
    }
}

pub fn main_menu_plugin(app: &mut App) {
    app.init_state::<MenuAnimationState>()
        .insert_resource(AnimationTimer {
            timer: Timer::from_seconds(2.0, TimerMode::Once),
        })
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(MenuState::Main)),
        )
        .add_systems(
            Update,
            animate_menu_transition
                .run_if(in_state(MenuAnimationState::Animating))
                .run_if(in_state(MenuState::Main)),
        )
        .add_systems(
            Update,
            (faction_hover_system, faction_selection_system).run_if(in_state(MenuState::Main)),
        )
        .add_systems(OnExit(MenuState::Main), despawn_main_menu);
}