use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::picking_core::PickSet;

mod menu;
mod game;
mod input;
mod systems;
mod utils;
mod ui;

use menu::common::{GameState, DisplayQuality, Volume};
use ui::menu::menu_plugin;
use ui::splash::splash_plugin;
use game::*;
use input::*;
use systems::*;
use utils::*;
use ui::*;

/// Маркер для UI камеры, чтобы можно было ее удалить при переходе в игру
#[derive(Component)]
struct UICamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>()
                .disable::<DebugPickingPlugin>()
        )
        .init_resource::<ClickCircle>()
        .init_resource::<SelectedEntity>()
        .init_resource::<CameraSettings>()
        .init_resource::<CameraMovementState>()
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, (setup_ui_camera, setup_particle_effect))
        .add_systems(
            Update, 
            (
                process_movement_orders,
                draw_click_circle,
                draw_movement_lines,
                select_entity_system.after(PickSet::Last),
                handle_ground_clicks.after(select_entity_system),
                handle_enemy_clicks.after(select_entity_system),
                update_projectiles,
                draw_hover_outline,
                draw_health_bars,
                camera_zoom_system,
                camera_right_button_movement.after(camera_zoom_system),
                camera_follow_selected.after(camera_zoom_system),
            ).run_if(in_state(GameState::Game))
        )
        .add_plugins((splash_plugin, menu_plugin, game::game_plugin))
        .run();
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), UICamera));
}

mod game_plugin {
    use bevy::prelude::*;
    use crate::{
        menu::common::{GameState, DisplayQuality, Volume, despawn_screen},
    };

    #[derive(Component)]
    struct OnGameScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(Update, game_system.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }

    fn game_setup(
        commands: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<StandardMaterial>>,
    ) {
        crate::game::setup::setup(
            commands,
            meshes,
            materials,
        );
    }

    fn game_system() {
        todo!("Game logic");
    }
}