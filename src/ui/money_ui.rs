use bevy::prelude::*;
use bevy::ecs::system::Resource;
use crate::game::ShapeType;
use crate::menu::common::GameState;

// Resource for player's money
#[derive(Resource, Debug, Default)]
pub struct Money(pub u32);

// Marker components for UI elements
#[derive(Component)]
pub struct MoneyText;
#[derive(Component)]
pub struct SpawnCubeButton;
#[derive(Component)]
pub struct SpawnSphereButton;

pub struct MoneyUiPlugin;

impl Plugin for MoneyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Money>()
            .insert_resource(Money(10))
            .add_systems(OnEnter(GameState::Game), setup_money_ui)
            .add_systems(Update, update_money_text.run_if(in_state(GameState::Game)))
            .add_systems(Update, handle_spawn_buttons.run_if(in_state(GameState::Game)));
    }
}

const BUTTON_BLUE: Color = Color::rgb(0.1, 0.2, 0.7);

// Setup UI: money text and spawn buttons
fn setup_money_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        }
    ).with_children(|parent| {
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
                "Spawn cube (-2)",
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
                "Spawn sphere (-2)",
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
                if money.0 >= 2 {
                    money.0 -= 2;
                    if is_cube.is_some() {
                        spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Cube);
                    } else if is_sphere.is_some() {
                        spawn_shape(&mut commands, &mut meshes, &mut materials, ShapeType::Sphere);
                    }
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
    ));
} 