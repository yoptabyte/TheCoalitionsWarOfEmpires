use bevy::prelude::*;
use crate::game::units::PlayerFaction;
use crate::menu::main_menu::Faction;

// States for the purchase menu
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PurchaseMenuState {
    #[default]
    Closed,
    Open,
}

// Component to mark entities as part of the purchase menu
#[derive(Component)]
pub struct PurchaseMenu;

// Use the existing OnGameScreen component
use crate::game_plugin::OnGameScreen;

// Component for the purchase menu button
#[derive(Component)]
pub struct PurchaseMenuButton;

// Component for the close button
#[derive(Component)]
pub struct CloseButton;

// Components for different unit purchase buttons
#[derive(Component, Clone)]
pub enum UnitPurchaseButton {
    Infantry(usize),  // 0, 1, 2 for the three types
    Tank(usize),      // 0, 1, 2 for the three types
    Aircraft(usize),  // 0, 1, 2 for the three types
    Farm,
    Mine,
    SteelFactory,
    PetrochemicalPlant,
    Trench,
}

// System to spawn the purchase menu button in the top-left corner
pub fn spawn_purchase_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                position_type: PositionType::Absolute,
                left: Val::Px(7.0),
                top: Val::Px(55.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgb(0.2, 0.2, 0.6).into(),
            ..default()
        },
        PurchaseMenuButton,
        OnGameScreen,
    ))
    .with_children(|parent| {
        parent.spawn(
            TextBundle::from_section(
                "+",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            }),
        );
    });
}

// System to handle the purchase menu button click
pub fn handle_purchase_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PurchaseMenuButton>)>,
    mut purchase_menu_state: ResMut<NextState<PurchaseMenuState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            purchase_menu_state.set(PurchaseMenuState::Open);
        }
    }
}

// System to spawn the purchase menu
pub fn spawn_purchase_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
) {
    // Main container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Percent(90.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(60.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgba(0.1, 0.1, 0.2, 0.9).into(),
                ..default()
            },
            PurchaseMenu,
        ))
        .with_children(|parent| {
            // Header with title and close button
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header| {
                    // Title
                    header.spawn(
                        TextBundle::from_section(
                            "Purchase Units & Buildings",
                            TextStyle {
                                font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::left(Val::Px(5.0)),
                            ..default()
                        }),
                    );

                    // Close button
                    header
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.7, 0.2, 0.2).into(),
                                ..default()
                            },
                            CloseButton,
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                    "X",
                                    TextStyle {
                                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                }),
                            );
                        });
                });

            // Section title for Infantry
            parent.spawn(
                TextBundle::from_section(
                    "Infantry",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                }),
            );

            // Infantry buttons row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    // Create infantry buttons based on faction
                    let infantry_names = match player_faction.0 {
                        Faction::Entente => ["Russian Infantry", "British Infantry", "French Infantry"],
                        Faction::CentralPowers => ["German Infantry", "Turkish Infantry", "Austro-Hungarian Infantry"],
                    };

                    for i in 0..3 {
                        row.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(110.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.3, 0.3, 0.7).into(),
                                ..default()
                            },
                            UnitPurchaseButton::Infantry(i),
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                    infantry_names[i],
                                    TextStyle {
                                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                }),
                            );
                        });
                    }
                });

            // Section title for Tanks
            parent.spawn(
                TextBundle::from_section(
                    "Tanks",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                }),
            );

            // Tank buttons row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    // Create tank buttons based on faction
                    let tank_names = match player_faction.0 {
                        Faction::Entente => ["Tsar Tank", "Mark I", "Renault FT"],
                        Faction::CentralPowers => ["Austro-Daimler", "A7V", "Ottoman Tank"],
                    };

                    for i in 0..3 {
                        row.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(110.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.3, 0.5, 0.7).into(),
                                ..default()
                            },
                            UnitPurchaseButton::Tank(i),
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                    tank_names[i],
                                    TextStyle {
                                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                }),
                            );
                        });
                    }
                });

            // Section title for Aircraft
            parent.spawn(
                TextBundle::from_section(
                    "Aircraft",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                }),
            );

            // Aircraft buttons row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    // Create aircraft buttons based on faction
                    let aircraft_names = match player_faction.0 {
                        Faction::Entente => ["Sopwith Camel", "SPAD S.XIII", "Sikorsky"],
                        Faction::CentralPowers => ["Fokker Dr.I", "Albatros D.III", "Gotha G.V"],
                    };

                    for i in 0..3 {
                        row.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(110.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.3, 0.3, 0.8).into(),
                                ..default()
                            },
                            UnitPurchaseButton::Aircraft(i),
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                    aircraft_names[i],
                                    TextStyle {
                                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                }),
                            );
                        });
                    }
                });

            // Section title for Buildings
            parent.spawn(
                TextBundle::from_section(
                    "Buildings",
                    TextStyle {
                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                }),
            );

            // Buildings buttons row (first row)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    // Create building buttons
                    let building_names = ["Farm", "Mine", "Steel Factory"];
                    let building_types = [
                        UnitPurchaseButton::Farm,
                        UnitPurchaseButton::Mine,
                        UnitPurchaseButton::SteelFactory,
                    ];

                    for i in 0..3 {
                        row.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(110.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.4, 0.6, 0.4).into(),
                                ..default()
                            },
                            building_types[i].clone(),
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                    building_names[i],
                                    TextStyle {
                                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                }),
                            );
                        });
                    }
                });

            // Buildings buttons row (second row)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    // Create remaining building buttons
                    let building_names = ["Petrochemical Plant", "Trench"];
                    let building_types = [
                        UnitPurchaseButton::PetrochemicalPlant,
                        UnitPurchaseButton::Trench,
                    ];

                    for i in 0..2 {
                        row.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(110.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: Color::rgb(0.4, 0.6, 0.4).into(),
                                ..default()
                            },
                            building_types[i].clone(),
                        ))
                        .with_children(|button| {
                            button.spawn(
                                TextBundle::from_section(
                                    building_names[i],
                                    TextStyle {
                                        font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                                        font_size: 14.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Auto),
                                    ..default()
                                }),
                            );
                        });
                    }
                });
        });
}

// System to handle the close button click
pub fn handle_close_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
    mut purchase_menu_state: ResMut<NextState<PurchaseMenuState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            purchase_menu_state.set(PurchaseMenuState::Closed);
        }
    }
}

// System to handle unit purchase button clicks
pub fn handle_unit_purchase(
    interaction_query: Query<(&Interaction, &UnitPurchaseButton), (Changed<Interaction>, With<Button>)>,
    mut placement_state: ResMut<crate::game::PlacementState>,
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // Set the placement state based on the button type
            placement_state.active = true;
            
            match button_type {
                UnitPurchaseButton::Infantry(_) => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::Infantry);
                },
                UnitPurchaseButton::Tank(_) => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::Cube);
                },
                UnitPurchaseButton::Aircraft(_) => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::Airplane);
                },
                UnitPurchaseButton::Farm => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::Farm);
                },
                UnitPurchaseButton::Mine => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::Mine);
                },
                UnitPurchaseButton::SteelFactory => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::SteelFactory);
                },
                UnitPurchaseButton::PetrochemicalPlant => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::PetrochemicalPlant);
                },
                UnitPurchaseButton::Trench => {
                    placement_state.shape_type = Some(crate::game::components::ShapeType::Trench);
                },
            }
            
            // Menu remains open after selection
        }
    }
}

// System to despawn the purchase menu
pub fn despawn_purchase_menu(
    mut commands: Commands,
    query: Query<Entity, With<PurchaseMenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Plugin to register all purchase menu systems

pub struct PurchaseMenuPlugin;

impl Plugin for PurchaseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PurchaseMenuState>()
            .add_systems(OnEnter(crate::menu::common::GameState::Game), spawn_purchase_button)
            // OnGameScreen components are automatically despawned when leaving game state
            .add_systems(Update, handle_purchase_button.run_if(in_state(crate::menu::common::GameState::Game)))
            .add_systems(OnEnter(PurchaseMenuState::Open), spawn_purchase_menu)
            .add_systems(
                Update,
                (handle_close_button, handle_unit_purchase).run_if(in_state(PurchaseMenuState::Open))
            )
            .add_systems(OnExit(PurchaseMenuState::Open), despawn_purchase_menu);
    }
}
