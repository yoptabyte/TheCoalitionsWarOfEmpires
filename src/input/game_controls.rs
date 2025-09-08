use bevy::prelude::*;
use crate::menu::common::GameState;

pub fn handle_escape_key(
    keys: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Game => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Game);
            }
            _ => {}
        }
    }
}