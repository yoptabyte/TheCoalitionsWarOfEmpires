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
pub mod asset_loader;
pub mod scene_colliders;
pub mod click_colliders;
pub use components::*;
pub use farm::*;
pub use mine::*;
pub use petrochemical_plant::*;
pub use resources::*;
pub use steel_factory::*;
pub use trench::*;
pub use click_colliders::*;
// Make units module accessible
// Units will be accessed through the module path

use crate::menu::common::GameState;
use crate::ui::money_ui::{Iron, Oil, Steel, AIMoney, AIWood, AIIron, AISteel, AIOil};
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .init_resource::<FarmIncomeTimer>()
        .init_resource::<Iron>()
        .init_resource::<Steel>()
        .init_resource::<Oil>()
        // AI Resources
        .init_resource::<AIMoney>()
        .init_resource::<AIWood>()
        .init_resource::<AIIron>()
        .init_resource::<AISteel>()
        .init_resource::<AIOil>()
        .init_resource::<TrenchCost>()
        .init_resource::<PlacementState>()
        .init_resource::<units::PlayerFaction>()
        .add_plugins(units::UnitsPlugin)
        .add_plugins(asset_loader::LazyAssetPlugin)
        // Farm systems
        .add_systems(Update, update_farm_income.run_if(in_state(GameState::Game)))
        .add_systems(Update, handle_farm_clicks.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            update_farm_visuals.run_if(in_state(GameState::Game)),
        )

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
            spawn_petrochemical_plant_on_keystroke.run_if(in_state(GameState::Game)),
        )
        // Scene collider systems
        .add_systems(
            Update,
            (
                scene_colliders::add_scene_colliders, 
                scene_colliders::add_deep_scene_colliders,
                scene_colliders::add_enemy_scene_colliders,
                scene_colliders::add_enemy_deep_scene_colliders,
                scene_colliders::add_parent_unit_colliders,
                scene_colliders::add_player_unit_scene_colliders,
                scene_colliders::add_precise_player_unit_colliders,
                scene_colliders::add_player_primitive_unit_colliders,
                scene_colliders::update_player_colliders_on_mesh_load,
                scene_colliders::handle_child_clicks,
                scene_colliders::handle_child_hover,
                click_colliders::add_debug_click_colliders,
            ).run_if(in_state(GameState::Game)),
        )
        // Trench systems
        .add_systems(
            Update,
            update_trench_construction.run_if(in_state(GameState::Game)),
        )

        .add_systems(
            Update,
            spawn_trench_on_keystroke.run_if(in_state(GameState::Game)),
        );
}

