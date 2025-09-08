use bevy::prelude::*;

use crate::menu::{
    common::{GameState, MenuState, despawn_screen},
    main_menu::{main_menu_plugin, menu_action, OnMainMenuScreen},
    settings_menu::{settings_menu_plugin, OnSettingsMenuScreen, OnDisplaySettingsMenuScreen, OnSoundSettingsMenuScreen},
    pause_menu::pause_menu_plugin,
};

use crate::ui::UICamera;

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), (setup_ui_camera_for_menu, set_main_menu_state).chain())
        .add_systems(OnEnter(GameState::Menu), (cleanup_game_entities, reset_game_resources, reset_game_state))
        .add_plugins(main_menu_plugin)
        .add_plugins(settings_menu_plugin)
        .add_plugins(pause_menu_plugin)
        .add_systems(
            Update,
            (menu_action, button_system, force_recreate_menu_if_empty).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnEnter(GameState::Game), cleanup_all_menu_ui)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>)
        .add_systems(OnExit(MenuState::SettingsDisplay), despawn_screen::<OnDisplaySettingsMenuScreen>)
        .add_systems(OnExit(MenuState::SettingsSound), despawn_screen::<OnSoundSettingsMenuScreen>);
}

fn cleanup_all_menu_ui(
    mut commands: Commands,
    all_menu_entities: Query<Entity, With<OnMainMenuScreen>>,
) {
    println!("DEBUG: cleanup_all_menu_ui called - removing {} menu entities", 
             all_menu_entities.iter().count());
    
    // Remove ALL entities with OnMainMenuScreen component (including 3D world model, cameras, UI)
    for entity in all_menu_entities.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}

fn setup_ui_camera_for_menu(
    mut commands: Commands,
    ui_cameras: Query<Entity, With<UICamera>>,
) {
    // Remove ALL existing UI cameras first
    for entity in ui_cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Then spawn exactly ONE UI camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // UI cameras have highest priority
                order: 10,  // Higher priority than 0
                ..default()
            },
            ..default()
        },
        UICamera,
    ));
}

fn cleanup_game_entities(
    mut commands: Commands,
    all_game_entities: Query<Entity, With<crate::game_plugin::OnGameScreen>>,
    // Cleanup all audio entities
    tank_audio_query: Query<Entity, With<crate::systems::movement::TankMovementAudio>>,
    aircraft_audio_query: Query<Entity, With<crate::systems::aircraft::AircraftMovementAudio>>,
    // Cleanup all other audio entities (victory, defeat, combat sounds, etc.)
    victory_audio_query: Query<Entity, With<crate::systems::victory_system::VictoryAudio>>,
    defeat_audio_query: Query<Entity, With<crate::systems::victory_system::DefeatAudio>>,
    // Cleanup all other game-related audio
    all_audio_query: Query<Entity, (With<AudioSink>, Without<crate::menu::main_menu::BackgroundMusic>)>,
) {
    println!("🧹 DEBUG: cleanup_game_entities called - removing {} game entities", 
             all_game_entities.iter().count());
    
    // Remove ALL game entities
    for entity in all_game_entities.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
    
    // Remove ALL movement audio
    for audio_entity in tank_audio_query.iter() {
        commands.entity(audio_entity).despawn();
        info!("🔇 Cleaned up tank movement audio");
    }
    
    for audio_entity in aircraft_audio_query.iter() {
        commands.entity(audio_entity).despawn();
        info!("🔇 Cleaned up aircraft movement audio");
    }
    
    // Remove victory/defeat audio
    for audio_entity in victory_audio_query.iter() {
        commands.entity(audio_entity).despawn();
        info!("🔇 Cleaned up victory audio");
    }
    
    for audio_entity in defeat_audio_query.iter() {
        commands.entity(audio_entity).despawn();
        info!("🔇 Cleaned up defeat audio");
    }
    
    // Remove ALL other audio except menu background music
    for audio_entity in all_audio_query.iter() {
        commands.entity(audio_entity).despawn();
        info!("🔇 Cleaned up game audio");
    }
    
    println!("🔇 DEBUG: All game audio and entities completely cleaned up");
}

fn reset_game_resources(
    // Игрок ресурсы
    mut money: ResMut<crate::ui::money_ui::Money>,
    mut wood: ResMut<crate::ui::money_ui::Wood>,
    mut steel: ResMut<crate::ui::money_ui::Steel>,
    mut oil: ResMut<crate::ui::money_ui::Oil>,
    // ИИ ресурсы 
    mut ai_money: ResMut<crate::ui::money_ui::AIMoney>,
    mut ai_wood: ResMut<crate::ui::money_ui::AIWood>,
    mut ai_steel: ResMut<crate::ui::money_ui::AISteel>,
    mut ai_oil: ResMut<crate::ui::money_ui::AIOil>,
) {
    // Сбрасываем все игровые ресурсы игрока к начальным значениям
    money.0 = 100.0;
    wood.0 = 50.0;
    steel.0 = 30.0;
    oil.0 = 20.0;
    
    // Сбрасываем все ресурсы ИИ к начальным значениям
    ai_money.0 = 100.0;
    ai_wood.0 = 50.0;
    ai_steel.0 = 30.0;
    ai_oil.0 = 20.0;
    
    println!("💰 DEBUG: All player and AI resources reset to starting values");
}

fn reset_game_state(
    // Состояние игры
    mut turn_state: ResMut<crate::systems::turn_system::TurnState>,
    mut victory_state: ResMut<crate::systems::victory_system::VictoryState>,
    mut selected_entity: ResMut<crate::game::SelectedEntity>,
    // Состояние размещения
    mut placement_state: ResMut<crate::game::PlacementState>,
    // Другие ресурсы которые могли быть изменены
    mut camera_movement_state: ResMut<crate::game::CameraMovementState>,
    mut processed_clicks: ResMut<crate::input::selection::ProcessedClicks>,
    mut click_circle: ResMut<crate::game::ClickCircle>,
    mut notification_state: ResMut<crate::ui::notification_system::NotificationState>,
) {
    // Сбрасываем состояние игры
    turn_state.turn_number = 1;
    turn_state.time_left = 20.0;
    turn_state.current_player = crate::systems::turn_system::PlayerTurn::Human;
    
    // Сбрасываем состояние победы
    victory_state.victory_timer = None;
    victory_state.defeat_timer = None;
    victory_state.game_ended = false;
    
    // Сбрасываем выделенный юнит
    selected_entity.0 = None;
    
    // Сбрасываем состояние размещения
    placement_state.active = false;
    placement_state.shape_type = None;
    placement_state.unit_type_index = None;
    
    // Сбрасываем состояние камеры
    camera_movement_state.manual_camera_mode = false;
    
    // Очищаем обработанные клики
    processed_clicks.processed_ids.clear();
    
    // Сбрасываем клик-сферу
    click_circle.position = None;
    click_circle.spawn_time = None;
    
    // Полностью сбрасываем состояние уведомлений
    *notification_state = crate::ui::notification_system::NotificationState::default();
    
    println!("🔄 DEBUG: All game states fully reset for new game");
}

fn set_main_menu_state(mut menu_state: ResMut<NextState<MenuState>>) {
    // Устанавливаем MenuState::Main при переходе в GameState::Menu
    println!("🔥 DEBUG: Setting MenuState::Main from Game->Menu transition");
    menu_state.set(MenuState::Main);
}

// Force recreate menu system - runs every frame when in Menu state
fn force_recreate_menu_if_empty(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_entities: Query<Entity, With<OnMainMenuScreen>>,
    game_state: Res<State<GameState>>,
    menu_state: Res<State<MenuState>>,
) {
    // Only run when in Menu state and Main menu state
    if *game_state.get() != GameState::Menu || *menu_state.get() != MenuState::Main {
        return;
    }
    
    // If no menu entities exist, recreate the menu
    if menu_entities.is_empty() {
        println!("🚨 DEBUG: Menu is empty, force recreating...");
        crate::menu::main_menu::main_menu_setup(commands, asset_server, menu_entities);
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct SelectedOption; 