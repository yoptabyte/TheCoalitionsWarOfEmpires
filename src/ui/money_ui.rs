use bevy::prelude::*;
use bevy::ecs::system::Resource;
use crate::game::ShapeType;
use crate::menu::common::{GameState, MenuState};
use crate::ui::confirm_dialog::{ConfirmDialog, ConfirmDialogAction, spawn_confirm_dialog};
use crate::game_plugin::OnGameScreen;
use bevy_mod_picking::prelude::*;

// Resource for player's money
#[derive(Resource, Debug, Default)]
pub struct Money(pub f32);

// Resource for player's wood
#[derive(Resource, Debug, Default)]
pub struct Wood(pub f32);

// Resource for player's iron
#[derive(Resource, Debug, Default)]
pub struct Iron(pub f32);

// Resource for player's steel
#[derive(Resource, Debug, Default)]
pub struct Steel(pub f32);

// Resource for player's oil
#[derive(Resource, Debug, Default)]
pub struct Oil(pub f32);

// Resource to track game time
#[derive(Resource, Debug, Default)]
pub struct GameTime {
    pub seconds: f32,
}

// Marker component for UI camera
#[derive(Component)]
pub struct UICamera;

// Enum for purchasable items and their costs
#[derive(Debug, Clone, Copy)]
pub enum PurchasableItem {
    Tank,
    Infantry,
    Airplane,
    Mine,
    SteelFactory,
    PetrochemicalPlant,
    Trench,
}

impl PurchasableItem {
    pub fn cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 3.0,
            PurchasableItem::Infantry => 2.0,
            PurchasableItem::Airplane => 5.0,
            PurchasableItem::Mine => 7.0,
            PurchasableItem::SteelFactory => 10.0,
            PurchasableItem::PetrochemicalPlant => 10.0,
            PurchasableItem::Trench => 3.0,
        }
    }
    
    pub fn wood_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 2.0,
            PurchasableItem::Infantry => 0.0,
            PurchasableItem::Airplane => 0.0,
            PurchasableItem::Mine => 3.0,
            PurchasableItem::SteelFactory => 2.0,
            PurchasableItem::PetrochemicalPlant => 5.0,
            PurchasableItem::Trench => 3.0,
        }
    }

    pub fn iron_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 2.0,
            PurchasableItem::Infantry => 0.0,
            PurchasableItem::Airplane => 0.0,
            PurchasableItem::Mine => 3.0,
            PurchasableItem::SteelFactory => 2.0,
            PurchasableItem::PetrochemicalPlant => 0.0,
            PurchasableItem::Trench => 0.0,
        }
    }
    
    pub fn steel_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 3.0,
            PurchasableItem::Infantry => 0.0,
            PurchasableItem::Airplane => 2.0,
            PurchasableItem::Mine => 0.0,
            PurchasableItem::SteelFactory => 0.0,
            PurchasableItem::PetrochemicalPlant => 5.0,
            PurchasableItem::Trench => 0.0,
        }
    }

    pub fn oil_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 5.0,
            PurchasableItem::Infantry => 0.0,
            PurchasableItem::Airplane => 5.0,
            PurchasableItem::Mine => 0.0,
            PurchasableItem::SteelFactory => 0.0,
            PurchasableItem::PetrochemicalPlant => 0.0,
            PurchasableItem::Trench => 0.0,
        }
    }

    pub fn shape_type(&self) -> ShapeType {
        match self {
            PurchasableItem::Tank => ShapeType::Cube,
            PurchasableItem::Infantry => ShapeType::Infantry,
            PurchasableItem::Airplane => ShapeType::Airplane,
            PurchasableItem::Mine => ShapeType::Mine,
            PurchasableItem::SteelFactory => ShapeType::SteelFactory,
            PurchasableItem::PetrochemicalPlant => ShapeType::PetrochemicalPlant,
            PurchasableItem::Trench => ShapeType::Trench,
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
pub struct SteelText;
#[derive(Component)]
pub struct OilText;
#[derive(Component)]
pub struct GameTimeText;
#[derive(Component)]
pub struct SpawnCubeButton;
#[derive(Component)]
pub struct SpawnInfantryButton;
#[derive(Component)]
pub struct SpawnAirplaneButton;
#[derive(Component)]
pub struct SpawnMineButton;
#[derive(Component)]
pub struct SpawnSteelFactoryButton;
#[derive(Component)]
pub struct SpawnPetrochemicalPlantButton;
#[derive(Component)]
pub struct ExitButton;
#[derive(Component)]
pub struct SpawnTrenchButton;

pub struct MoneyUiPlugin;

impl Plugin for MoneyUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Money>()
            .init_resource::<Wood>()
            .init_resource::<Iron>()
            .init_resource::<Steel>()
            .init_resource::<Oil>()
            .init_resource::<GameTime>()
            .insert_resource(Money(10.0))
            .insert_resource(Wood(5.0))
            .insert_resource(Iron(3.0))
            .insert_resource(Steel(0.0))
            .insert_resource(Oil(0.0))
            .insert_resource(GameTime { seconds: 0.0 })
            .add_systems(OnEnter(GameState::Game), setup_money_ui)
            .add_systems(Update, update_resources_text.run_if(in_state(GameState::Game)))
            .add_systems(Update, update_game_time.run_if(in_state(GameState::Game)))
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
    // Добавляем черную полосу под ресурсами
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            ..default()
        },
        OnGameScreen,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Money: 10.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(1.0, 0.9, 0.0),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        MoneyText,
        OnGameScreen,
    ));
    
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Wood: 5.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.5, 0.3, 0.0),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(220.0),
            ..default()
        }),
        WoodText,
        OnGameScreen,
    ));
    
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Iron: 3.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(380.0),
            ..default()
        }),
        IronText,
        OnGameScreen,
    ));
    
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Steel: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.3, 0.3, 0.8),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(520.0),
            ..default()
        }),
        SteelText,
        OnGameScreen,
    ));
    
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Oil: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.1, 0.9),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(660.0),
            ..default()
        }),
        OilText,
        OnGameScreen,
    ));

    // Добавляем отображение времени
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Time: 0:00",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(800.0),
            ..default()
        }),
        GameTimeText,
        OnGameScreen,
    ));

    // UI container for right-side buttons
    commands.spawn(
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                margin: UiRect::all(Val::Px(8.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }
    ).with_children(|parent| {
        // Spawn cube button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::DARK_GREEN.into(),
                ..default()
            },
            SpawnCubeButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn cube (-{})", PurchasableItem::Tank.cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Spawn infantry button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BUTTON_BLUE.into(),
                ..default()
            },
            SpawnInfantryButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn infantry (-{})", PurchasableItem::Infantry.cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
        
        // Spawn airplane button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.7, 0.7, 0.7).into(),
                ..default()
            },
            SpawnAirplaneButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn airplane (-{} $, -{} steel, -{} oil)", 
                    PurchasableItem::Airplane.cost(), 
                    PurchasableItem::Airplane.steel_cost(),
                    PurchasableItem::Airplane.oil_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Spawn mine button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.0, 0.0, 0.8).into(),
                ..default()
            },
            SpawnMineButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn mine (-{} $, -{} wood)", PurchasableItem::Mine.cost(), PurchasableItem::Mine.wood_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Spawn steel factory button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.6, 0.3, 0.1).into(),
                ..default()
            },
            SpawnSteelFactoryButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn steel factory (-{} $, -{} wood, -{} iron)", PurchasableItem::SteelFactory.cost(), PurchasableItem::SteelFactory.wood_cost(), PurchasableItem::SteelFactory.iron_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Spawn petrochemical plant button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgb(0.0, 0.0, 0.8).into(),
                ..default()
            },
            SpawnPetrochemicalPlantButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn petrochemical plant (-{} $, -{} wood, -{} steel)", 
                    PurchasableItem::PetrochemicalPlant.cost(), 
                    PurchasableItem::PetrochemicalPlant.wood_cost(), 
                    PurchasableItem::PetrochemicalPlant.steel_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Spawn trench button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BUTTON_BLUE.into(),
                ..default()
            },
            SpawnTrenchButton,
            OnGameScreen,
        )).with_children(|b| {
            b.spawn(TextBundle::from_section(
                format!("Spawn trench (-{}$, -{}🪵)", PurchasableItem::Trench.cost(), PurchasableItem::Trench.wood_cost()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Exit button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
                    font_size: 20.0,
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
    steel: Res<Steel>,
    oil: Res<Oil>,
    mut query_set: ParamSet<(
        Query<&mut Text, With<MoneyText>>,
        Query<&mut Text, With<WoodText>>, 
        Query<&mut Text, With<IronText>>,
        Query<&mut Text, With<SteelText>>,
        Query<&mut Text, With<OilText>>
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
    
    if steel.is_changed() {
        for mut text in &mut query_set.p3() {
            text.sections[0].value = format!("Steel: {:.1}", steel.0);
        }
    }
    
    if oil.is_changed() {
        for mut text in &mut query_set.p4() {
            text.sections[0].value = format!("Oil: {:.1}", oil.0);
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

// System to update the game time
fn update_game_time(
    time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut query: Query<&mut Text, With<GameTimeText>>,
) {
    // Update the game time
    game_time.seconds += time.delta_seconds();
    
    // Get minutes and seconds
    let minutes = (game_time.seconds / 60.0) as u32;
    let seconds = (game_time.seconds % 60.0) as u32;
    
    // Update the text
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Time: {}:{:02}", minutes, seconds);
    }
}

// Handle button presses for spawning tank/infantry/airplane
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
            Option<&SpawnInfantryButton>,
            Option<&SpawnAirplaneButton>,
            Option<&SpawnMineButton>,
            Option<&SpawnSteelFactoryButton>,
            Option<&SpawnPetrochemicalPlantButton>,
            Option<&SpawnTrenchButton>
        ),
        (Changed<Interaction>, Or<(With<SpawnCubeButton>, With<SpawnInfantryButton>, With<SpawnAirplaneButton>, With<SpawnMineButton>, With<SpawnSteelFactoryButton>, With<SpawnPetrochemicalPlantButton>, With<SpawnTrenchButton>)>)
    >,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    mut steel: ResMut<Steel>,
    mut oil: ResMut<Oil>,
    mut placement_state: ResMut<crate::game::PlacementState>,
    time: Res<Time>,
) {
    for (interaction, mut color, entity, is_cube, is_infantry, is_airplane, is_mine, is_steel_factory, is_petrochemical_plant, is_trench) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let item = if is_cube.is_some() {
                    PurchasableItem::Tank
                } else if is_infantry.is_some() {
                    PurchasableItem::Infantry
                } else if is_airplane.is_some() {
                    PurchasableItem::Airplane
                } else if is_mine.is_some() {
                    PurchasableItem::Mine
                } else if is_steel_factory.is_some() {
                    PurchasableItem::SteelFactory
                } else if is_petrochemical_plant.is_some() {
                    PurchasableItem::PetrochemicalPlant
                } else if is_trench.is_some() {
                    PurchasableItem::Trench
                } else {
                    continue;
                };

                // Check if player has enough resources
                if money.0 >= item.cost() && wood.0 >= item.wood_cost() && iron.0 >= item.iron_cost() && steel.0 >= item.steel_cost() && oil.0 >= item.oil_cost() {
                    // Устанавливаем состояние размещения объекта
                    placement_state.active = true;
                    placement_state.shape_type = Some(item.shape_type());
                    
                    // Снимаем ресурсы заранее
                    money.0 -= item.cost();
                    wood.0 -= item.wood_cost();
                    iron.0 -= item.iron_cost();
                    steel.0 -= item.steel_cost();
                    oil.0 -= item.oil_cost();
                    
                    info!("Placement mode activated for {:?}", item.shape_type());
                }

                // Обработка размещения окопа отдельно (сохраняем оригинальный код)
                if is_trench.is_some() {
                    if money.0 >= item.cost() && wood.0 >= item.wood_cost() {
                        money.0 -= item.cost();
                        wood.0 -= item.wood_cost();
                        
                        // Определяем положение для окопа
                        let seed = time.elapsed_seconds_f64().fract() as f32;
                        let x = (seed * 100.0).sin() * 10.0 - 5.0;
                        let z = (seed * 100.0).cos() * 10.0 - 5.0;
                        let trench_position = Vec3::new(x, 0.0, z);
                        
                        info!("Spawning new trench at position: {:?}", trench_position);
                        
                        crate::game::spawn_constructing_trench(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            trench_position,
                        );
                    } else {
                        info!("Not enough resources to build a trench!");
                    }
                }

                *color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::ORANGE_RED.into();
                
                if is_trench.is_some() {
                    *color = BUTTON_BLUE.into();
                }
            }
            Interaction::None => {
                if is_cube.is_some() {
                    *color = Color::DARK_GREEN.into();
                } else if is_infantry.is_some() {
                    *color = BUTTON_BLUE.into();
                } else if is_airplane.is_some() {
                    *color = Color::rgb(0.7, 0.7, 0.7).into();
                } else if is_mine.is_some() {
                    *color = Color::rgb(0.0, 0.0, 0.8).into();
                } else if is_steel_factory.is_some() {
                    *color = Color::rgb(0.6, 0.3, 0.1).into();
                } else if is_petrochemical_plant.is_some() {
                    *color = Color::rgb(0.0, 0.0, 0.8).into();
                } else if is_trench.is_some() {
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
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    mut steel: ResMut<Steel>,
    mut game_time: ResMut<GameTime>,
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
                    steel.0 = 0.0;
                    game_time.seconds = 0.0;
                    
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
        ShapeType::Infantry => {
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
                crate::game::Selectable,
                crate::game::HoveredOutline,
                crate::game::MovementOrder(Vec3::ZERO),
                PickableBundle::default(),
                crate::game::Health {
                    current: 60.0,
                    max: 60.0,
                },
                crate::game::CanShoot {
                    cooldown: 0.8,
                    last_shot: 0.0,
                    range: 12.0,
                    damage: 8.0,
                },
                Name::new("Infantry"),
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
                Vec3::new(-15.0, 0.0, 0.0),
            );
        }
        ShapeType::SteelFactory => {
            crate::game::steel_factory::spawn_inactive_steel_factory(
                commands,
                meshes,
                materials,
                Vec3::new(15.0, 0.0, 0.0),
            );
        }
        ShapeType::PetrochemicalPlant => {
            crate::game::petrochemical_plant::spawn_inactive_petrochemical_plant(
                commands,
                meshes,
                materials,
                Vec3::new(10.0, 0.0, -5.0),
            );
        }
        ShapeType::Trench => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 1.5))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.5, 0.35, 0.15),
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 0.25, 0.0),
                    ..default()
                },
                shape_type,
                Name::new("Trench"),
            ));
        }
    }
} 