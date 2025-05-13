use bevy::prelude::*;
use bevy::ecs::system::Resource;
use crate::game::ShapeType;
use crate::menu::common::{GameState, MenuState};
use crate::ui::confirm_dialog::{ConfirmDialog, ConfirmDialogAction, spawn_confirm_dialog};
use crate::game_plugin::OnGameScreen;

// Resource for player's money
#[derive(Resource, Debug, Default)]
pub struct Money(pub f32);

// Resource for player's wood
#[derive(Resource, Debug, Default)]
pub struct Wood(pub f32);

// Resource for player's iron
#[derive(Resource, Debug, Default)]
pub struct Iron(pub f32);

// Marker component for UI camera
#[derive(Component)]
pub struct UICamera;

// Enum for purchasable items and their costs
#[derive(Debug, Clone, Copy)]
pub enum PurchasableItem {
    Tank,
    Sphere,
    Airplane,
    Mine,
}

impl PurchasableItem {
    pub fn cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 3.0,
            PurchasableItem::Sphere => 2.0,
            PurchasableItem::Airplane => 5.0,
            PurchasableItem::Mine => 100.0,
        }
    }
    
    pub fn wood_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 2.0,
            PurchasableItem::Sphere => 0.0,
            PurchasableItem::Airplane => 0.0,
            PurchasableItem::Mine => 35.0,
        }
    }

    pub fn iron_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 2.0,
            PurchasableItem::Sphere => 0.0,
            PurchasableItem::Airplane => 0.0,
            PurchasableItem::Mine => 0.0,
        }
    }

    pub fn shape_type(&self) -> ShapeType {
        match self {
            PurchasableItem::Tank => ShapeType::Cube,
            PurchasableItem::Sphere => ShapeType::Sphere,
            PurchasableItem::Airplane => ShapeType::Airplane,
            PurchasableItem::Mine => ShapeType::Mine,
        }
    }
}

// Marker components for UI elements
#[derive(Component)]
pub struct MoneyText;
#[derive(Component)]
pub struct WoodText;
#[derive(Component)]
pub struct IronText;
#[derive(Component)]
pub struct SpawnCubeButton;
#[derive(Component)]
pub struct SpawnSphereButton;
#[derive(Component)]
pub struct SpawnAirplaneButton;
#[derive(Component)]
pub struct SpawnMineButton;
#[derive(Component)]
pub struct ExitButton;

pub struct MoneyUiPlugin;

impl Plugin for MoneyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Money>()
            .init_resource::<Wood>()
            .init_resource::<Iron>()
            .insert_resource(Money(10.0))
            .insert_resource(Wood(5.0))
            .insert_resource(Iron(3.0))
            .add_systems(OnEnter(GameState::Game), setup_money_ui)
            .add_systems(Update, update_resources_text.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_spawn_buttons.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_exit_button.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_confirm_dialog.run_if(in_state(GameState::Game)))
            .add_systems(Update, update_wood_from_forest.run_if(in_state(GameState::Game)))
            .add_systems(Update, update_iron_from_mines.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), cleanup_game_entities);
    }
}

const BUTTON_BLUE: Color = Color::rgb(0.1, 0.2, 0.7);

// Setup UI: money text and spawn buttons
fn setup_money_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        },
        OnGameScreen,
    )).with_children(|parent| {
        // Money text
        parent.spawn((
            TextBundle::from_section(
                "Money: 10.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            MoneyText,
        ));
        
        // Wood text
        parent.spawn((
            TextBundle::from_section(
                "Wood: 5.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.0, 0.8, 0.0),
                },
            ),
            WoodText,
        ));
        
        // Iron text
        parent.spawn((
            TextBundle::from_section(
                "Iron: 3.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::rgb(0.0, 0.0, 0.8),
                },
            ),
            IronText,
        ));
        
        // Spawn cube button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(220.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
            SpawnCubeButton,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn tank (-{} $, -{} wood, -{} iron)", PurchasableItem::Tank.cost(), PurchasableItem::Tank.wood_cost(), PurchasableItem::Tank.iron_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Spawn sphere button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: BUTTON_BLUE.into(),
                ..default()
            },
            SpawnSphereButton,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn sphere (-{})", PurchasableItem::Sphere.cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Spawn airplane button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::rgb(0.7, 0.7, 0.7).into(),
                ..default()
            },
            SpawnAirplaneButton,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn airplane (-{})", PurchasableItem::Airplane.cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Spawn mine button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(270.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::rgb(0.0, 0.0, 0.8).into(),
                ..default()
            },
            SpawnMineButton,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn mine (-{} $, -{} wood)", PurchasableItem::Mine.cost(), PurchasableItem::Mine.wood_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Exit button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
                    height: Val::Px(40.0),
                    margin: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
            ExitButton,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                "Exit to Menu",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ));
        });
    });
}

// Update resources text when they change
fn update_resources_text(
    money: Res<Money>, 
    wood: Res<Wood>,
    iron: Res<Iron>,
    mut query_set: ParamSet<(
        Query<&mut Text, With<MoneyText>>,
        Query<&mut Text, With<WoodText>>, 
        Query<&mut Text, With<IronText>>
    )>,
) {
    if money.is_changed() {
        for mut text in &mut query_set.p0() {
            text.sections[0].value = format!("Money: {:.1}", money.0);
        }
    }
    
    if wood.is_changed() {
        for mut text in &mut query_set.p1() {
            text.sections[0].value = format!("Wood: {:.1}", wood.0);
        }
    }
    
    if iron.is_changed() {
        for mut text in &mut query_set.p2() {
            text.sections[0].value = format!("Iron: {:.1}", iron.0);
        }
    }
}

// System to update wood from forest farms
fn update_wood_from_forest(
    time: Res<Time>,
    mut wood: ResMut<Wood>,
    query: Query<(&crate::game::ForestFarm, &crate::game::FarmActive)>,
) {
    let dt = time.delta_seconds();
    let mut total_wood_income = 0.0;
    
    for (_, active) in query.iter() {
        if active.0 {
            total_wood_income += 0.1 * dt; // 0.1 wood per second
        }
    }
    
    if total_wood_income > 0.0 {
        wood.0 += total_wood_income;
    }
}

// System to update iron from mines
fn update_iron_from_mines(
    time: Res<Time>,
    mut iron: ResMut<Iron>,
    query: Query<(&crate::game::Mine, &crate::game::FarmActive, &crate::game::MineIronRate)>,
) {
    // Skip processing if there are no mines
    if query.is_empty() {
        return;
    }
    
    // Calculate iron generation per second for active mines
    let mut iron_per_second = 0.0;
    for (_, active, iron_rate) in query.iter() {
        if active.0 {
            iron_per_second += iron_rate.0;
        }
    }
    
    // Add iron based on elapsed time
    if iron_per_second > 0.0 {
        // Here we directly update iron outside the farm income timer
        // This prevents large jumps when the timer triggers
        iron.0 += iron_per_second * time.delta_seconds();
    }
}

// Handle button presses for spawning tank/sphere/airplane
fn handle_spawn_buttons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Entity,
            Option<&SpawnCubeButton>,
            Option<&SpawnSphereButton>,
            Option<&SpawnAirplaneButton>,
            Option<&SpawnMineButton>
        ),
        (Changed<Interaction>, Or<(With<SpawnCubeButton>, With<SpawnSphereButton>, With<SpawnAirplaneButton>, With<SpawnMineButton>)>)
    >,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
) {
    for (interaction, mut color, entity, is_cube, is_sphere, is_airplane, is_mine) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let item = if is_cube.is_some() {
                    PurchasableItem::Tank
                } else if is_sphere.is_some() {
                    PurchasableItem::Sphere
                } else if is_airplane.is_some() {
                    PurchasableItem::Airplane
                } else if is_mine.is_some() {
                    PurchasableItem::Mine
                } else {
                    continue;
                };

                // Check if player has enough resources
                if money.0 >= item.cost() && wood.0 >= item.wood_cost() && iron.0 >= item.iron_cost() {
                    money.0 -= item.cost();
                    wood.0 -= item.wood_cost();
                    iron.0 -= item.iron_cost();
                    spawn_shape(&mut commands, &mut meshes, &mut materials, item.shape_type());
                }
                *color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::ORANGE_RED.into();
            }
            Interaction::None => {
                if is_cube.is_some() {
                    *color = Color::DARK_GREEN.into();
                } else if is_sphere.is_some() {
                    *color = BUTTON_BLUE.into();
                } else if is_airplane.is_some() {
                    *color = Color::rgb(0.7, 0.7, 0.7).into();
                } else if is_mine.is_some() {
                    *color = Color::rgb(0.0, 0.0, 0.8).into();
                }
            }
        }
    }
}

// Handle exit button
fn handle_exit_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ExitButton>)>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                spawn_confirm_dialog(&mut commands, &asset_server);
                *color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.8, 0.2, 0.2).into();
            }
            Interaction::None => {
                *color = Color::RED.into();
            }
        }
    }
}

// Handle confirm dialog
fn handle_confirm_dialog(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &ConfirmDialogAction), (Changed<Interaction>, With<ConfirmDialogAction>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    dialog_query: Query<Entity, With<ConfirmDialog>>,
) {
    for (interaction, action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                ConfirmDialogAction::Yes => {
                    // First despawn the dialog
                    if let Ok(dialog) = dialog_query.get_single() {
                        commands.entity(dialog).despawn_recursive();
                    }
                    
                    // Reset resources to initial values
                    money.0 = 10.0;
                    wood.0 = 5.0;
                    iron.0 = 3.0;
                    
                    // Then set states in the correct order
                    menu_state.set(MenuState::Main);
                    game_state.set(GameState::Menu);
                }
                ConfirmDialogAction::No => {
                    // Just close the dialog
                    if let Ok(dialog) = dialog_query.get_single() {
                        commands.entity(dialog).despawn_recursive();
                    }
                }
            }
        }
    }
}

// Cleanup all game entities when exiting game state
fn cleanup_game_entities(
    mut commands: Commands,
    game_entities: Query<Entity, (With<ShapeType>, Without<Camera>)>,
    ui_elements: Query<Entity, With<Node>>,
    dialog_query: Query<Entity, With<ConfirmDialog>>,
    ground_query: Query<Entity, With<crate::game::Ground>>,
    camera_query: Query<Entity, With<crate::game::MainCamera>>,
    ui_camera_query: Query<Entity, With<UICamera>>,
    light_query: Query<Entity, With<PointLight>>,
    directional_light_query: Query<Entity, With<DirectionalLight>>,
) {
    // Despawn all game entities (shapes)
    for entity in game_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn all UI elements
    for entity in ui_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn any remaining confirm dialogs
    for entity in dialog_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn ground
    for entity in ground_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn game camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn UI camera
    for entity in ui_camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all point lights
    for entity in light_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all directional lights (including sun)
    for entity in directional_light_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Spawn new UI camera for menu
    commands.spawn((Camera2dBundle::default(), UICamera));
}

fn spawn_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    shape_type: ShapeType,
) {
    match shape_type {
        ShapeType::Cube => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.8, 0.7, 0.6),
                        ..default()
                    }),
                    transform: Transform::from_xyz(5.0, 0.5, 0.0),
                    ..default()
                },
                shape_type,
                crate::game::components::Selectable,
                crate::game::components::HoveredOutline,
                crate::game::components::MovementOrder(Vec3::ZERO),
                crate::game::components::Health {
                    current: 100.0,
                    max: 100.0,
                },
                crate::game::components::CanShoot {
                    cooldown: 1.0,
                    last_shot: 0.0,
                    range: 10.0,
                    damage: 10.0,
                },
                crate::game::components::Tank,
                Name::new("Tank"),
            ));
        }
        ShapeType::Sphere => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Sphere::new(0.5))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.2, 0.5, 0.8),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                },
                shape_type,
                crate::game::components::Selectable,
                crate::game::components::HoveredOutline,
                crate::game::components::MovementOrder(Vec3::ZERO),
                crate::game::components::Health {
                    current: 50.0,
                    max: 50.0,
                },
                crate::game::components::CanShoot {
                    cooldown: 0.7,
                    last_shot: 0.0,
                    range: 15.0,
                    damage: 8.0,
                },
            ));
        }
        ShapeType::Airplane => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 4.0))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.8, 0.8, 0.8),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 10.0, 0.0),
                    ..default()
                },
                shape_type,
                crate::game::components::Selectable,
                crate::game::components::HoveredOutline,
                crate::game::components::MovementOrder(Vec3::ZERO),
                crate::game::components::Aircraft {
                    height: 10.0,
                    speed: 5.0,
                },
                crate::game::components::Health {
                    current: 75.0,
                    max: 75.0,
                },
                crate::game::components::CanShoot {
                    cooldown: 0.5,
                    last_shot: 0.0,
                    range: 20.0,
                    damage: 15.0,
                },
            ));
        }
        ShapeType::Tower => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(1.5, 3.0, 1.5))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.5, 0.5, 0.5),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 1.5, 0.0),
                    ..default()
                },
                shape_type,
                crate::game::components::Selectable,
                crate::game::components::HoveredOutline,
                crate::game::components::Tower {
                    height: 3.0,
                },
                crate::game::components::Health {
                    current: 200.0,
                    max: 200.0,
                },
                crate::game::components::CanShoot {
                    cooldown: 2.0,
                    last_shot: 0.0,
                    range: 25.0,
                    damage: 20.0,
                },
            ));
        }
        ShapeType::Farm => {
            crate::game::farm::spawn_forest_farm(
                commands,
                meshes,
                materials,
                Vec3::new(0.0, 0.0, 0.0),
            );
        }
        ShapeType::Mine => {
            crate::game::mine::spawn_inactive_mine(
                commands,
                meshes,
                materials,
                Vec3::new(0.0, 0.0, 0.0),
            );
        }
    }
} 