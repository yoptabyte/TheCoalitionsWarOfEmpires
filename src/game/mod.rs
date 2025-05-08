pub mod components;
pub mod resources;
pub mod setup;
pub use components::*;
pub use resources::*;
pub use setup::*;

use bevy::prelude::*;
use crate::menu::common::GameState;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup::setup)
        .add_systems(Update, game_logic.run_if(in_state(GameState::Game)));
}

fn game_logic() {
    // TODO: Implement game logic
}