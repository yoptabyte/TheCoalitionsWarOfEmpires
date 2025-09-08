use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerTurn {
    Human,
    AI,
}

#[derive(Resource, Debug)]
pub struct TurnState {
    pub current_player: PlayerTurn,
    pub time_left: f32,
    pub turn_number: u32,
}

impl Default for TurnState {
    fn default() -> Self {
        Self {
            current_player: PlayerTurn::Human,
            time_left: TURN_DURATION,
            turn_number: 1,
        }
    }
}

const TURN_DURATION: f32 = 20.0;

pub fn update_turn_system(
    mut turn_state: ResMut<TurnState>,
    time: Res<Time>,
) {
    // Отсчет времени хода
    turn_state.time_left -= time.delta_seconds();
    
    if turn_state.time_left <= 0.0 {
        // Смена игрока
        match turn_state.current_player {
            PlayerTurn::Human => {
                turn_state.current_player = PlayerTurn::AI;
                info!("Turn {} ended! AI turn starts", turn_state.turn_number);
            }
            PlayerTurn::AI => {
                turn_state.current_player = PlayerTurn::Human;
                turn_state.turn_number += 1;
                info!("Turn {} ended! Player turn starts", turn_state.turn_number);
            }
        }
        
        // Сброс таймера
        turn_state.time_left = TURN_DURATION;
    }
}