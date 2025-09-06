pub mod aircraft;
pub mod tower;
pub mod combat;
pub mod movement;
pub mod turn_system;
pub mod ai_opponent;

pub use aircraft::*;
#[allow(unused_imports)]
pub use tower::*;
pub use combat::*;
pub use movement::*;
pub use turn_system::*;
pub use ai_opponent::*;
use bevy::prelude::ResMut;

pub fn clear_processed_clicks(mut processed_clicks: ResMut<crate::input::selection::ProcessedClicks>) {
    processed_clicks.processed_ids.clear();
}