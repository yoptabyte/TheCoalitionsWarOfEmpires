pub mod components;
pub mod game;
pub mod farm;
pub mod mine;
pub mod petrochemical_plant;
pub mod resources;
pub mod setup;
pub mod steel_factory;
pub mod trench;
pub mod units;
pub use components::*;
pub use farm::*;
pub use mine::*;
pub use petrochemical_plant::*;
pub use resources::*;
pub use steel_factory::*;
pub use trench::*;
// Make units module accessible
// Units will be accessed through the module path

use crate::menu::common::GameState;
use crate::ui::money_ui::{Iron, Oil, Steel};
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .init_resource::<FarmIncomeTimer>()
        .init_resource::<Iron>()
        .init_resource::<Steel>()
        .init_resource::<Oil>()
        .init_resource::<TrenchCost>()
        .init_resource::<PlacementState>()
        .init_resource::<units::PlayerFaction>()
        .add_plugins(units::UnitsPlugin)
        // Farm systems
        .add_systems(Update, update_farm_income.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_farm_clicks.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            update_farm_visuals.run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, draw_farm_status.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            spawn_forest_farm_on_keystroke.run_if(in_state(GameState::Game)),
        )
        // Mine systems
        .add_systems(Update, update_mine_income.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_mine_clicks.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            update_mine_visuals.run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, draw_mine_status.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            spawn_mine_on_keystroke.run_if(in_state(GameState::Game)),
        )
        // Steel factory systems
        .add_systems(
            Update,
            update_steel_factory_income.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            handle_steel_factory_clicks.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            update_steel_factory_visuals.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            draw_steel_factory_status.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            spawn_steel_factory_on_keystroke.run_if(in_state(GameState::Game)),
        )
        // Petrochemical plant systems
        .add_systems(
            Update,
            update_petrochemical_plant_income.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            handle_petrochemical_plant_clicks.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            update_petrochemical_plant_visuals.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            draw_petrochemical_plant_status.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            spawn_petrochemical_plant_on_keystroke.run_if(in_state(GameState::Game)),
        )
        // Trench systems
        .add_systems(
            Update,
            update_trench_construction.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            draw_trench_construction_progress.run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            spawn_trench_on_keystroke.run_if(in_state(GameState::Game)),
        );
}

#[allow(dead_code)]
fn game_logic() {
    // TODO: Implement game logic
}
