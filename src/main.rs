use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rapier3d::prelude::*;


mod game;
mod input;
mod menu;
mod systems;
mod ui;
mod utils;

use game::*;
use input::selection::{ProcessedClicks, handle_enemy_clicks, select_entity_system, handle_ground_clicks, handle_placement_clicks, debug_all_clicks, raycast_unit_selection};
use input::*;
use menu::common::{DisplayQuality, GameState, Volume};
use systems::*;
use ui::menu::menu_plugin;
use ui::splash::splash_plugin;

use utils::*;

/// Marker for UI camera to allow removing it when transitioning to game
#[derive(Component)]
struct UICamera;

#[derive(Resource)]
struct FpsLimiter {
    last_frame_time: std::time::Instant,
    target_frame_duration: std::time::Duration,
}

impl Default for FpsLimiter {
    fn default() -> Self {
        Self {
            last_frame_time: std::time::Instant::now(),
            target_frame_duration: std::time::Duration::from_millis(33), // 30 FPS
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Coalitions War of Empires".to_string(),
                resolution: (1024.0, 768.0).into(),
                present_mode: bevy::window::PresentMode::Immediate, // Disable VSync for custom FPS limit
                ..default()
            }),
            ..default()
        }))
        .add_plugins(HanabiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())

        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>()
                .disable::<DebugPickingPlugin>(),
        )
        .init_resource::<ClickCircle>()
        .init_resource::<SelectedEntity>()
        .init_resource::<CameraSettings>()
        .init_resource::<CameraMovementState>()
        .init_resource::<ProcessedClicks>()
        .init_resource::<systems::AIBehavior>()
        .init_resource::<systems::TurnState>()
        .init_resource::<FpsLimiter>()
        .init_resource::<ui::money_ui::AIMoney>()
        .init_resource::<ui::money_ui::AIWood>()
        .init_resource::<ui::money_ui::AIIron>()
        .init_resource::<ui::money_ui::AISteel>()
        .init_resource::<ui::money_ui::AIOil>()
        .init_resource::<systems::victory_system::VictoryState>()
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, (setup_ui_camera, setup_particle_effect))
        .add_systems(Update, fps_limiter_system)
        .add_systems(
            Update,
            clear_processed_clicks.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            handle_escape_key.run_if(in_state(GameState::Game).or_else(in_state(GameState::Paused))),
        )
        .add_systems(
            Update,
            (
                process_movement_orders,
                ui::health_bars::draw_health_bars,
                aircraft_movement,
                systems::combat::handle_trench_damage,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (debug_all_clicks, select_entity_system, handle_enemy_clicks, raycast_unit_selection).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            handle_placement_clicks.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            handle_ground_clicks.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            handle_attacks.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                game::scene_colliders::add_enemy_scene_colliders,
                game::scene_colliders::add_enemy_deep_scene_colliders,
                game::scene_colliders::handle_child_clicks,
                game::scene_colliders::handle_child_hover,
            ).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                camera_zoom_system,
                camera_right_button_movement,
                camera_follow_selected,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                tower::repair_tower,
                tower::update_tower_health_status,
                tower::spawn_tower_on_keystroke,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            systems::turn_system::update_turn_system.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                systems::victory_system::check_victory_conditions,
                systems::victory_system::handle_victory_timers,
                systems::cheat_system::handle_cheat_keys,
            ).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            systems::ai_economy::ai_resource_generation_system.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            systems::ai_opponent::ai_purchase_system.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                systems::ai_opponent::ai_movement_system,
                systems::ai_opponent::ai_combat_system,
            ).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnEnter(GameState::Game),
            systems::ai_economy::ai_initial_resources_system,
        )
        .add_systems(OnExit(GameState::Game), (reset_placement_state, reset_game_state))
        .add_plugins((
            splash_plugin,
            menu_plugin,
            game::game_plugin,
            ui::money_ui::MoneyUiPlugin,
            ui::ui_plugin,
        ))
        .run();
}

fn setup_ui_camera(mut commands: Commands) {
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

fn reset_placement_state(mut placement_state: ResMut<PlacementState>) {
    placement_state.active = false;
    placement_state.shape_type = None;
}

fn reset_game_state(
    mut money: ResMut<ui::money_ui::Money>,
    mut wood: ResMut<ui::money_ui::Wood>,
    mut iron: ResMut<ui::money_ui::Iron>,
    mut steel: ResMut<ui::money_ui::Steel>,
    mut oil: ResMut<ui::money_ui::Oil>,
    mut ai_money: ResMut<ui::money_ui::AIMoney>,
    mut ai_wood: ResMut<ui::money_ui::AIWood>,
    mut ai_iron: ResMut<ui::money_ui::AIIron>,
    mut ai_steel: ResMut<ui::money_ui::AISteel>,
    mut ai_oil: ResMut<ui::money_ui::AIOil>,
    mut turn_state: ResMut<systems::turn_system::TurnState>,
    mut victory_state: ResMut<systems::victory_system::VictoryState>,
    mut selected_entity: ResMut<SelectedEntity>,
) {
    // Reset player resources to starting values
    money.0 = 100.0;
    wood.0 = 50.0;
    iron.0 = 30.0;
    steel.0 = 10.0;
    oil.0 = 10.0;
    
    // Reset AI resources to starting values
    ai_money.0 = 20.0;
    ai_wood.0 = 15.0;
    ai_iron.0 = 10.0;
    ai_steel.0 = 5.0;
    ai_oil.0 = 5.0;
    
    // Reset turn state
    turn_state.current_player = systems::turn_system::PlayerTurn::Human;
    turn_state.time_left = 30.0; // TURN_DURATION
    turn_state.turn_number = 1;
    
    // Reset victory state
    victory_state.game_ended = false;
    victory_state.victory_timer = None;
    victory_state.defeat_timer = None;
    
    // Clear selected entity
    selected_entity.0 = None;
    
    println!("🔄 Game state reset for new game");
}

fn fps_limiter_system(mut fps_limiter: ResMut<FpsLimiter>) {
    let now = std::time::Instant::now();
    let elapsed = now.duration_since(fps_limiter.last_frame_time);
    
    if elapsed < fps_limiter.target_frame_duration {
        let sleep_time = fps_limiter.target_frame_duration - elapsed;
        std::thread::sleep(sleep_time);
    }
    
    fps_limiter.last_frame_time = std::time::Instant::now();
}

pub mod game_plugin {
    use crate::{
        menu::common::{despawn_screen, GameState},
        systems::aircraft::spawn_initial_aircraft,
    };
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct OnGameScreen;

    #[derive(Resource, Deref, DerefMut)]
    #[allow(dead_code)]
    struct GameTimer(Timer);

    pub fn game_plugin(app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            spawn_initial_aircraft,
        )
        .add_systems(Update, game_system.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }


    fn game_system() {
        todo!("Game logic");
    }
}
