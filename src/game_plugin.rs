use crate::{
    menu::common::{despawn_screen, GameState},
    systems::aircraft::spawn_initial_aircraft,
    game::setup,
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
        (setup::setup, spawn_initial_aircraft),
    )
    .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}
