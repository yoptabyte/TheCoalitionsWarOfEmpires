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

// AI Resources - separate economy for AI
#[derive(Resource, Debug, Default)]
pub struct AIMoney(pub f32);

#[derive(Resource, Debug, Default)]
pub struct AIWood(pub f32);

#[derive(Resource, Debug, Default)]
pub struct AIIron(pub f32);

#[derive(Resource, Debug, Default)]
pub struct AISteel(pub f32);

#[derive(Resource, Debug, Default)]
pub struct AIOil(pub f32);

// Resource to track game time
#[derive(Resource, Debug, Default)]
pub struct GameTime {
    pub seconds: f32,
}

// Marker component for UI camera
#[derive(Component)]
pub struct UICamera;

// Enum for purchasable items and their costs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PurchasableItem {
    Tank,
    Infantry,
    Airplane,
    Mine,
    SteelFactory,
    PetrochemicalPlant,
}

impl PurchasableItem {
    pub fn cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 45.0,      // Средняя стоимость танков
            PurchasableItem::Infantry => 16.0,  // Средняя стоимость пехоты  
            PurchasableItem::Airplane => 40.0,  // Средняя стоимость самолетов
            PurchasableItem::Mine => 30.0,      // Стоимость шахты
            PurchasableItem::SteelFactory => 40.0,  // Стоимость сталелитейного завода
            PurchasableItem::PetrochemicalPlant => 50.0, // Стоимость нефтезавода
        }
    }
    
    pub fn wood_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 5.0,       // Средние требования танков
            PurchasableItem::Infantry => 0.0,   // Пехота не требует дерева
            PurchasableItem::Airplane => 9.0,   // Средние требования самолетов
            PurchasableItem::Mine => 8.0,       // Требования шахты
            PurchasableItem::SteelFactory => 12.0, // Требования завода
            PurchasableItem::PetrochemicalPlant => 15.0, // Требования нефтезавода
        }
    }

    pub fn iron_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 7.0,       // Средние требования танков
            PurchasableItem::Infantry => 0.0,   // Пехота не требует железа
            PurchasableItem::Airplane => 4.0,   // Средние требования самолетов
            PurchasableItem::Mine => 0.0,       // Шахта не требует железа
            PurchasableItem::SteelFactory => 15.0, // Требования завода
            PurchasableItem::PetrochemicalPlant => 10.0, // Требования нефтезавода
        }
    }
    
    pub fn steel_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 6.0,       // Средние требования танков
            PurchasableItem::Infantry => 0.0,   // Пехота не требует стали
            PurchasableItem::Airplane => 5.0,   // Средние требования самолетов
            PurchasableItem::Mine => 0.0,       // Шахта не требует стали
            PurchasableItem::SteelFactory => 0.0, // Завод не требует стали
            PurchasableItem::PetrochemicalPlant => 8.0, // Требования нефтезавода
        }
    }

    pub fn oil_cost(&self) -> f32 {
        match self {
            PurchasableItem::Tank => 11.0,      // Средние требования танков
            PurchasableItem::Infantry => 0.0,   // Пехота не требует нефти
            PurchasableItem::Airplane => 17.0,  // Средние требования самолетов
            PurchasableItem::Mine => 0.0,       // Шахта не требует нефти
            PurchasableItem::SteelFactory => 0.0, // Завод не требует нефти
            PurchasableItem::PetrochemicalPlant => 0.0, // Нефтезавод не требует нефти
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
struct SteelText;
#[derive(Component)]
struct OilText;
#[derive(Component)]
struct GameTimeText;

// AI Resources display components
#[derive(Component)]
struct AIMoneyText;

#[derive(Component)]
struct AIWoodText;

#[derive(Component)]
struct AIIronText;

#[derive(Component)]
struct AISteelText;

#[derive(Component)]
struct AIOilText;
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
            .insert_resource(Money(45.0))
            .insert_resource(Wood(5.0))
            .insert_resource(Iron(3.0))
            .insert_resource(Steel(0.0))
            .insert_resource(Oil(0.0))
            .insert_resource(GameTime { seconds: 0.0 })
            .add_systems(OnEnter(GameState::Game), setup_money_ui)
            .add_systems(Update, update_resources_text.run_if(in_state(GameState::Game)))
            .add_systems(Update, update_ai_resources_text.run_if(in_state(GameState::Game)))
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
    // Add a black bar under the resources
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
                "Money: 45.0",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
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
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
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
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
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
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
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
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
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

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Time: 0:00",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
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

    // UI container for right-side buttons (only exit button)
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
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
    });

    // AI Resources Display (right side)
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "AI Money: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(1.0, 0.5, 0.5), // Red tint for AI
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            right: Val::Px(10.0),
            ..default()
        }),
        AIMoneyText,
        OnGameScreen,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "AI Wood: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.8, 0.4, 0.2),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(85.0),
            right: Val::Px(10.0),
            ..default()
        }),
        AIWoodText,
        OnGameScreen,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "AI Iron: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.7, 0.7, 0.7),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(110.0),
            right: Val::Px(10.0),
            ..default()
        }),
        AIIronText,
        OnGameScreen,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "AI Steel: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.5, 0.5, 0.8),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(135.0),
            right: Val::Px(10.0),
            ..default()
        }),
        AISteelText,
        OnGameScreen,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "AI Oil: 0.0",
                TextStyle {
                    font: asset_server.load("fonts/GrenzeGotisch-Light.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(0.2, 0.2, 0.2),
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(160.0),
            right: Val::Px(10.0),
            ..default()
        }),
        AIOilText,
        OnGameScreen,
    ));
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
    _time: Res<Time>,
    mut wood: ResMut<Wood>,
    mut money: ResMut<Money>,
    query: Query<(&crate::game::ForestFarm, &crate::game::FarmActive)>,
) {
    let dt = _time.delta_seconds();
    let mut total_wood_income = 0.0;
    let mut total_money_income = 0.0;
    
    for (_, active) in query.iter() {
        if active.0 {
            total_wood_income += 0.5 * dt; // 0.5 wood per second (faster!)
            total_money_income += 0.5 * dt; // 0.5 money per second from farms
        }
    }
    
    if total_wood_income > 0.0 {
        wood.0 += total_wood_income;
    }
    
    if total_money_income > 0.0 {
        money.0 += total_money_income;
    }
}

// System to update iron from mines
fn update_iron_from_mines(
    _time: Res<Time>,
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
        iron.0 += iron_per_second * _time.delta_seconds();
    }
}

// System to update the game time
fn update_game_time(
    _time: Res<Time>,
    mut game_time: ResMut<GameTime>,
    mut query: Query<&mut Text, With<GameTimeText>>,
) {
    // Update the game time
    game_time.seconds += _time.delta_seconds();
    
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
    _commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
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
            Option<&SpawnPetrochemicalPlantButton>
        ),
        (Changed<Interaction>, Or<(With<SpawnCubeButton>, With<SpawnInfantryButton>, With<SpawnAirplaneButton>, With<SpawnMineButton>, With<SpawnSteelFactoryButton>, With<SpawnPetrochemicalPlantButton>)>)
    >,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    mut steel: ResMut<Steel>,
    mut oil: ResMut<Oil>,
    mut placement_state: ResMut<crate::game::PlacementState>,
    _time: Res<Time>,
) {
    for (interaction, mut color, _entity, is_cube, is_infantry, is_airplane, is_mine, is_steel_factory, is_petrochemical_plant) in &mut interaction_query {
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
                } else {
                    continue;
                };

                // Check if player has enough resources
                info!("🔥 OLD UI: Button pressed for item {:?}", item);
                if can_afford_item(item, &money, &wood, &iron, &steel, &oil) {
                    // Set the object placement state for units
                    info!("🔥 OLD UI: Setting placement state active for {:?}", item.shape_type());
                    placement_state.active = true;
                    placement_state.shape_type = Some(item.shape_type());
                    
                    // Deduct resources in advance
                    deduct_resources(item, &mut money, &mut wood, &mut iron, &mut steel, &mut oil);
                    
                    info!("Placement mode activated for {:?}", item.shape_type());
                } else {
                    info!("Not enough resources to purchase {:?}! Need: Money: {}, Wood: {}, Iron: {}, Steel: {}, Oil: {}", 
                          item, item.cost(), item.wood_cost(), item.iron_cost(), item.steel_cost(), item.oil_cost());
                }

                *color = Color::GRAY.into();
            }
            Interaction::Hovered => {
                *color = Color::ORANGE_RED.into();
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
    mut oil: ResMut<Oil>,
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
                    money.0 = 45.0;
                    wood.0 = 5.0;
                    iron.0 = 3.0;
                    steel.0 = 0.0;
                    oil.0 = 0.0;
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

    // Don't despawn UI camera - it will be managed by the menu system

    // Despawn all point lights
    for entity in light_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn all directional lights (including sun)
    for entity in directional_light_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // UI camera will be managed by the menu system
}

#[allow(dead_code)]
fn spawn_shape(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    shape_type: ShapeType,
    asset_server: &AssetServer,
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
                &asset_server,
            );
        }
        ShapeType::Mine => {
            crate::game::mine::spawn_active_mine(
                commands,
                meshes,
                materials,
                Vec3::new(-15.0, 0.0, 0.0),
                asset_server,
            );
        }
        ShapeType::SteelFactory => {
            crate::game::steel_factory::spawn_active_steel_factory(
                commands,
                meshes,
                materials,
                Vec3::new(15.0, 0.0, 0.0),
                asset_server,
            );
        }
        ShapeType::PetrochemicalPlant => {
            crate::game::petrochemical_plant::spawn_active_petrochemical_plant(
                commands,
                meshes,
                materials,
                Vec3::new(10.0, 0.0, -5.0),
                asset_server,
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

// Update AI resources text when they change
fn update_ai_resources_text(
    ai_money: Res<AIMoney>, 
    ai_wood: Res<AIWood>,
    ai_iron: Res<AIIron>,
    ai_steel: Res<AISteel>,
    ai_oil: Res<AIOil>,
    mut query_set: ParamSet<(
        Query<&mut Text, With<AIMoneyText>>,
        Query<&mut Text, With<AIWoodText>>, 
        Query<&mut Text, With<AIIronText>>,
        Query<&mut Text, With<AISteelText>>,
        Query<&mut Text, With<AIOilText>>
    )>,
) {
    // Update AI Money text
    if ai_money.is_changed() {
        if let Ok(mut text) = query_set.p0().get_single_mut() {
            text.sections[0].value = format!("AI Money: {:.1}", ai_money.0);
        }
    }

    // Update AI Wood text
    if ai_wood.is_changed() {
        if let Ok(mut text) = query_set.p1().get_single_mut() {
            text.sections[0].value = format!("AI Wood: {:.1}", ai_wood.0);
        }
    }

    // Update AI Iron text
    if ai_iron.is_changed() {
        if let Ok(mut text) = query_set.p2().get_single_mut() {
            text.sections[0].value = format!("AI Iron: {:.1}", ai_iron.0);
        }
    }

    // Update AI Steel text
    if ai_steel.is_changed() {
        if let Ok(mut text) = query_set.p3().get_single_mut() {
            text.sections[0].value = format!("AI Steel: {:.1}", ai_steel.0);
        }
    }

    // Update AI Oil text
    if ai_oil.is_changed() {
        if let Ok(mut text) = query_set.p4().get_single_mut() {
            text.sections[0].value = format!("AI Oil: {:.1}", ai_oil.0);
        }
    }
}

// Helper function to check if player can afford an item
pub fn can_afford_item(
    item: PurchasableItem,
    money: &Money,
    wood: &Wood,
    iron: &Iron,
    steel: &Steel,
    oil: &Oil,
) -> bool {
    money.0 >= item.cost() && 
    wood.0 >= item.wood_cost() && 
    iron.0 >= item.iron_cost() && 
    steel.0 >= item.steel_cost() && 
    oil.0 >= item.oil_cost()
}

// Helper function to check if AI can afford an item
pub fn can_afford_item_ai(
    item: PurchasableItem,
    money: &AIMoney,
    wood: &AIWood,
    iron: &AIIron,
    steel: &AISteel,
    oil: &AIOil,
) -> bool {
    money.0 >= item.cost() && 
    wood.0 >= item.wood_cost() && 
    iron.0 >= item.iron_cost() && 
    steel.0 >= item.steel_cost() && 
    oil.0 >= item.oil_cost()
}

// Helper function to deduct resources from player
pub fn deduct_resources(
    item: PurchasableItem,
    money: &mut Money,
    wood: &mut Wood,
    iron: &mut Iron,
    steel: &mut Steel,
    oil: &mut Oil,
) {
    money.0 -= item.cost();
    wood.0 -= item.wood_cost();
    iron.0 -= item.iron_cost();
    steel.0 -= item.steel_cost();
    oil.0 -= item.oil_cost();
}

// Helper function to deduct resources from AI
pub fn deduct_resources_ai(
    item: PurchasableItem,
    money: &mut AIMoney,
    wood: &mut AIWood,
    iron: &mut AIIron,
    steel: &mut AISteel,
    oil: &mut AIOil,
) {
    money.0 -= item.cost();
    wood.0 -= item.wood_cost();
    iron.0 -= item.iron_cost();
    steel.0 -= item.steel_cost();
    oil.0 -= item.oil_cost();
}

// Helper function to place shapes for player
pub fn place_shape(
    commands: &mut Commands,
    shape_type: crate::game::components::ShapeType,
    position: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &AssetServer,
    player_faction: &Res<crate::game::units::PlayerFaction>,
) {
    info!("🔥🔥🔥 place_shape: FUNCTION CALLED!!! shape_type {:?} at position {:?} faction {:?}", shape_type, position, player_faction.0);
    use crate::game::components::*;
    use bevy_rapier3d::prelude::*;
    use bevy_mod_picking::prelude::*;
    
    match shape_type {
        ShapeType::Mine => {
            crate::game::mine::spawn_active_mine(
                commands,
                meshes,
                materials,
                position,
                asset_server,
            );
        },
        ShapeType::SteelFactory => {
            crate::game::steel_factory::spawn_active_steel_factory(
                commands,
                meshes,
                materials,
                position,
                asset_server,
            );
        },
        ShapeType::PetrochemicalPlant => {
            crate::game::petrochemical_plant::spawn_active_petrochemical_plant(
                commands,
                meshes,
                materials,
                position,
                asset_server,
            );
        },
        ShapeType::Cube => {
            use crate::menu::main_menu::Faction;
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let tank_type_index = rng.gen_range(0..3);
            
            let (model_path, scale) = match player_faction.0 {
                Faction::Entente => {
                    match tank_type_index {
                        0 => ("models/entente/tanks/tsar_tank.glb#Scene0", 0.1), // tsar_tank уменьшен в 4 раза
                        1 => ("models/entente/tanks/mark1.glb#Scene0", 0.08), // mark1 уменьшен в 5 раз
                        _ => ("models/entente/tanks/renault_ft17.glb#Scene0", 0.4), // renault остается нормальным
                    }
                },
                Faction::CentralPowers => {
                    match tank_type_index {
                        0 => ("models/central_powers/tanks/panzerwagen.glb#Scene0", 0.08), // panzerwagen уменьшен в 5 раз
                        1 => ("models/central_powers/tanks/a7v.glb#Scene0", 0.08), // a7v уменьшен в 5 раз
                        _ => ("models/central_powers/tanks/steam_wheel_tank.glb#Scene0", 0.08), // steam_wheel остается как был
                    }
                },
            };
            
            info!("🔥 TANK: Loading model from path: {} with scale: {}", model_path, scale);
            let entity_id = commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(scale)),
                    ..default()
                },
                ShapeType::Cube,
                crate::game::components::Selectable,
                crate::game::components::Tank,
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
                RigidBody::Dynamic,
                Collider::cuboid(5.0, 5.0, 5.0), // Очень большой коллайдер для танков
                Sensor, // Невидимый коллайдер для кликов
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
                Restitution::coefficient(0.0),
                Friction::coefficient(0.8),
                bevy_mod_picking::prelude::PickableBundle::default(),
                Name::new("Player Tank"),
            )).id();
            info!("🔥 TANK SPAWNED: Entity {:?} at position {:?} with Selectable component", entity_id, position);
        },
        ShapeType::Airplane => {
            use crate::menu::main_menu::Faction;
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let aircraft_type_index = rng.gen_range(0..3);
            
            let model_path = match player_faction.0 {
                Faction::Entente => {
                    match aircraft_type_index {
                        0 => "models/entente/airplanes/sopwith_camel.glb#Scene0",
                        1 => "models/entente/airplanes/breguet_14.glb#Scene0",
                        _ => "models/entente/airplanes/ilya_muromets.glb#Scene0",
                    }
                },
                Faction::CentralPowers => {
                    match aircraft_type_index {
                        0 => "models/central_powers/airplanes/fokker.glb#Scene0",
                        1 => "models/central_powers/airplanes/albatros.glb#Scene0",
                        _ => "models/central_powers/airplanes/red_baron.glb#Scene0",
                    }
                },
            };
            info!("🔥 AIRCRAFT: Loading model from path: {}", model_path);
            let entity_id = commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(position + Vec3::new(0.0, 10.0, 0.0))
                        .with_scale(Vec3::splat(0.6)),
                    ..default()
                },
                ShapeType::Airplane,
                crate::game::components::Selectable,
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
                RigidBody::Fixed,
                Collider::cuboid(7.0, 4.0, 8.0), // Очень большой коллайдер для самолетов
                Sensor, // Невидимый коллайдер для кликов
                LockedAxes::all(),
                bevy_mod_picking::prelude::PickableBundle::default(),
                Name::new("Player Aircraft"),
            )).id();
            info!("🔥 AIRCRAFT SPAWNED: Entity {:?} at position {:?} with Selectable component", entity_id, position);
        },
        ShapeType::Infantry => {
            use crate::menu::main_menu::Faction;
            use crate::game::units::infantry::{InfantryType, EntenteInfantryType, CentralPowersInfantryType, Infantry, InfantryAttributes};
            use crate::game::components::{Health, CanShoot, Selectable, HoveredOutline};
            use bevy_rapier3d::prelude::*;
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let infantry_type_index = rng.gen_range(0..3);
            
            let infantry_type = match player_faction.0 {
                Faction::Entente => {
                    match infantry_type_index {
                        0 => InfantryType::Entente(EntenteInfantryType::Russian),
                        1 => InfantryType::Entente(EntenteInfantryType::British),
                        _ => InfantryType::Entente(EntenteInfantryType::French),
                    }
                },
                Faction::CentralPowers => {
                    match infantry_type_index {
                        0 => InfantryType::CentralPowers(CentralPowersInfantryType::German),
                        1 => InfantryType::CentralPowers(CentralPowersInfantryType::Turkish),
                        _ => InfantryType::CentralPowers(CentralPowersInfantryType::AustroHungarian),
                    }
                },
            };
            
            // Get model path based on infantry type
            let model_path = match infantry_type {
                InfantryType::Entente(entente_type) => {
                    match entente_type {
                        EntenteInfantryType::Russian => "models/infantry/russian_soldier.glb#Scene0",
                        EntenteInfantryType::British => "models/infantry/british_soldier.glb#Scene0",
                        EntenteInfantryType::French => "models/infantry/french_soldier.glb#Scene0",
                    }
                },
                InfantryType::CentralPowers(central_type) => {
                    match central_type {
                        CentralPowersInfantryType::German => "models/infantry/german_soldier.glb#Scene0",
                        CentralPowersInfantryType::Turkish => "models/infantry/turkish_soldier.glb#Scene0",
                        CentralPowersInfantryType::AustroHungarian => "models/infantry/austrian_soldier.glb#Scene0",
                    }
                },
            };
            
            // Get stats for this infantry type
            let stats = infantry_type.get_stats();
            
            info!("🔥 INFANTRY: Loading model from path: {}", model_path);
            let entity_id = commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(position)
                        .with_scale(Vec3::splat(0.5)),
                    ..default()
                },
                Infantry,
                InfantryAttributes {
                    infantry_type,
                },
                stats,
                Health {
                    current: stats.health,
                    max: stats.max_health,
                },
                CanShoot {
                    cooldown: 1.0 / stats.attack_speed,
                    last_shot: 0.0,
                    range: 10.0,
                    damage: stats.attack_damage,
                },
                ShapeType::Infantry,
                Selectable,
                HoveredOutline,
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
                Collider::ball(3.0), // Очень большой коллайдер для пехоты
                Sensor, // Невидимый коллайдер для кликов
                bevy_mod_picking::prelude::PickableBundle::default(),
                Name::new("Player Infantry"),
            )).id();
            info!("🔥 INFANTRY SPAWNED: Entity {:?} at position {:?} with Selectable component", entity_id, position);
        },
        _ => {
            info!("Placement for {:?} not implemented yet", shape_type);
        }
    }
} 