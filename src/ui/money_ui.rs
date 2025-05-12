use bevy::prelude::*;
use bevy::ecs::system::Resource;
use bevy_mod_picking::prelude::*;
use crate::game::ShapeType;
use crate::menu::common::{GameState, MenuState};
use crate::ui::confirm_dialog::{ConfirmDialog, ConfirmDialogAction, spawn_confirm_dialog};
use crate::game_plugin::OnGameScreen;

// Resource for player's money
#[derive(Resource, Debug, Default)]
pub struct Money(pub u32);

// Marker component for UI camera
#[derive(Component)]
pub struct UICamera;

// Enum for purchasable items and their costs
#[derive(Debug, Clone, Copy)]
pub enum PurchasableItem {
    Cube,
    Sphere,
}

impl PurchasableItem {
    pub fn cost(&self) -> u32 {
        match self {
            PurchasableItem::Cube => 3,
            PurchasableItem::Sphere => 2,
        }
    }

    pub fn shape_type(&self) -> ShapeType {
        match self {
            PurchasableItem::Cube => ShapeType::Cube,
            PurchasableItem::Sphere => ShapeType::Sphere,
        }
    }
}

// Marker components for UI elements
#[derive(Component)]
pub struct MoneyText;
#[derive(Component)]
pub struct SpawnCubeButton;
#[derive(Component)]
pub struct SpawnSphereButton;
#[derive(Component)]
pub struct ExitButton;

pub struct MoneyUiPlugin;

impl Plugin for MoneyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Money>()
            .insert_resource(Money(10))
            .add_systems(OnEnter(GameState::Game), setup_money_ui)
            .add_systems(Update, update_money_text.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_spawn_buttons.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_exit_button.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_confirm_dialog.run_if(in_state(GameState::Game)))
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
                "Money: 10",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ),
            MoneyText,
        ));
        // Spawn cube button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(180.0),
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
                format!("Spawn cube (-{})", PurchasableItem::Cube.cost()),
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

// Update money text when money changes
fn update_money_text(money: Res<Money>, mut query: Query<&mut Text, With<MoneyText>>) {
    if money.is_changed() {
        for mut text in &mut query {
            text.sections[0].value = format!("Money: {}", money.0);
        }
    }
}

// Handle button presses for spawning cube/sphere
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
            Option<&SpawnSphereButton>
        ),
        (Changed<Interaction>, Or<(With<SpawnCubeButton>, With<SpawnSphereButton>)>)
    >,
    mut money: ResMut<Money>,
) {
    for (interaction, mut color, entity, is_cube, is_sphere) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let item = if is_cube.is_some() {
                    PurchasableItem::Cube
                } else if is_sphere.is_some() {
                    PurchasableItem::Sphere
                } else {
                    continue;
                };

                if money.0 >= item.cost() {
                    money.0 -= item.cost();
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
                } else {
                    *color = BUTTON_BLUE.into();
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

    // Spawn new UI camera for menu
    commands.spawn((Camera2dBundle::default(), UICamera));
}

fn spawn_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    shape_type: ShapeType,
) {
    let mesh = match shape_type {
        ShapeType::Cube => meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
        ShapeType::Sphere => meshes.add(Mesh::from(shape::UVSphere {
            radius: 0.5,
            sectors: 32,
            stacks: 16,
        })),
    };

    let material = materials.add(StandardMaterial {
        base_color: match shape_type {
            ShapeType::Cube => Color::GREEN,
            ShapeType::Sphere => Color::BLUE,
        },
        ..default()
    });

    // Different shooting parameters for cubes and spheres
    let (cooldown, range, damage) = match shape_type {
        ShapeType::Cube => (1.0, 10.0, 25.0), // Cube: slower but stronger
        ShapeType::Sphere => (0.7, 8.0, 15.0), // Sphere: faster but weaker
    };

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        shape_type,
        crate::game::Selectable,
        crate::game::Health {
            current: 100.0,
            max: 100.0,
        },
        crate::game::CanShoot {
            cooldown,
            last_shot: 0.0,
            range,
            damage,
        },
        bevy_mod_picking::prelude::PickableBundle::default(),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(crate::game::HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<crate::game::HoveredOutline>();
        }),
    ));
} 