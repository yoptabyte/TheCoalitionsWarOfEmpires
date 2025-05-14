pub mod components;
pub mod resources;
pub mod setup;
pub mod farm;
pub mod mine;
pub mod steel_factory;
pub use components::*;
pub use resources::*;
pub use farm::*;
pub use mine::*;
pub use steel_factory::*;

use bevy::prelude::*;
use crate::menu::common::GameState;
use crate::ui::money_ui::{Iron, Steel};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .init_resource::<FarmIncomeTimer>()
        .init_resource::<Iron>()
        .init_resource::<Steel>()
        // Farm systems
        .add_systems(Update, update_farm_income.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_farm_clicks.run_if(in_state(GameState::Game)))
        .add_systems(Update, update_farm_visuals.run_if(in_state(GameState::Game)))
        .add_systems(Update, draw_farm_status.run_if(in_state(GameState::Game)))
        .add_systems(Update, spawn_forest_farm_on_keystroke.run_if(in_state(GameState::Game)))
        // Mine systems
        .add_systems(Update, update_mine_income.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_mine_clicks.run_if(in_state(GameState::Game)))
        .add_systems(Update, update_mine_visuals.run_if(in_state(GameState::Game)))
        .add_systems(Update, draw_mine_status.run_if(in_state(GameState::Game)))
        .add_systems(Update, spawn_mine_on_keystroke.run_if(in_state(GameState::Game)))
        // Steel factory systems
        .add_systems(Update, update_steel_factory_income.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_steel_factory_clicks.run_if(in_state(GameState::Game)))
        .add_systems(Update, update_steel_factory_visuals.run_if(in_state(GameState::Game)))
        .add_systems(Update, draw_steel_factory_status.run_if(in_state(GameState::Game)))
        .add_systems(Update, spawn_steel_factory_on_keystroke.run_if(in_state(GameState::Game)));
}

fn game_logic() {
    // TODO: Implement game logic
}