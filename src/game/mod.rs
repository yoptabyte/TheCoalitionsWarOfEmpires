pub mod components;
pub mod resources;
pub mod setup;
pub mod farm;
pub mod mine;
pub use components::*;
pub use resources::*;
pub use farm::*;
pub use mine::*;

use bevy::prelude::*;
use crate::menu::common::GameState;
use crate::ui::money_ui::Iron;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .init_resource::<FarmIncomeTimer>()
        .init_resource::<Iron>()
        .add_systems(
            Update,
            (
                update_farm_income,
                handle_farm_clicks,
                update_farm_visuals,
                draw_farm_status,
                spawn_forest_farm_on_keystroke,
                update_mine_income,
                handle_mine_clicks,
                update_mine_visuals,
                draw_mine_status,
                spawn_mine_on_keystroke,
            )
            .run_if(in_state(GameState::Game))
        );
}

fn game_logic() {
    // TODO: Implement game logic
}