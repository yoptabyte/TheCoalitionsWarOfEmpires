pub mod components;
pub mod resources;
pub mod setup;
pub mod farm;
pub use components::*;
pub use resources::*;
pub use setup::*;
pub use farm::*;

use bevy::prelude::*;
use crate::menu::common::GameState;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .init_resource::<FarmIncomeTimer>()
        .add_systems(
            Update,
            (
                update_farm_income,
                handle_farm_clicks,
                update_farm_visuals,
                draw_farm_status,
                spawn_forest_farm_on_keystroke,
            )
            .run_if(in_state(GameState::Game))
        );
}

fn game_logic() {
    // TODO: Implement game logic
}