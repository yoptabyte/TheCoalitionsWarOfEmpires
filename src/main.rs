use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::picking_core::PickSet;

mod game;
mod input;
mod systems;
mod utils;
mod ui;

use game::*;
use input::*;
use systems::*;
use utils::*;
use ui::*;

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
        .add_systems(Startup, (setup, setup_particle_effect))
        .add_systems(Update, (
            process_movement_orders,
            draw_click_circle,
            draw_movement_lines,
            select_entity_system.after(PickSet::Last),
            handle_ground_clicks.after(select_entity_system),
            draw_hover_outline,
            camera_zoom_system,
            camera_right_button_movement,
            camera_follow_selected.after(camera_zoom_system),
        ))
        .run();
}