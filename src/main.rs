use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_picking::picking_core::PickSet;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

mod game;
mod input;
mod menu;
mod systems;
mod ui;
mod utils;

use game::*;
use input::selection::ProcessedClicks;
use input::*;
use menu::common::{DisplayQuality, GameState, Volume};
use systems::*;
use ui::menu::menu_plugin;
use ui::splash::splash_plugin;
use ui::*;
use utils::*;

/// Marker for UI camera to allow removing it when transitioning to game
#[derive(Component)]
struct UICamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Coalitions War of Empires".to_string(),
                resolution: (1024.0, 768.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(HanabiPlugin)
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
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, (setup_ui_camera, setup_particle_effect))
        .add_systems(
            Update,
            (
                clear_processed_clicks,
                process_movement_orders,
                draw_click_circle,
                draw_movement_lines,
                select_entity_system.after(PickSet::Last),
                handle_placement_clicks,
                handle_ground_clicks.after(handle_placement_clicks),
                handle_attacks.after(select_entity_system),
                update_projectiles,
                draw_hover_outline,
                draw_health_bars,
                camera_zoom_system,
                camera_right_button_movement.after(camera_zoom_system),
                camera_follow_selected.after(camera_zoom_system),
                aircraft_movement,
                tower::repair_tower,
                tower::update_tower_health_status,
                tower::spawn_tower_on_keystroke,
                systems::combat::handle_trench_damage,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), reset_placement_state)
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
                // Set higher priority so UI renders on top
                order: 1,
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
            (game_setup, spawn_initial_aircraft),
        )
        .add_systems(Update, game_system.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }

    fn game_setup(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) {
        crate::game::setup::setup(commands, meshes, materials);
    }

    fn game_system() {
        todo!("Game logic");
    }
}
