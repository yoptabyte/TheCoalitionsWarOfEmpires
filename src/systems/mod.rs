pub mod aircraft;
pub mod tower;
pub mod combat;
pub mod movement;
pub mod turn_system;
pub mod ai_economy;
pub mod ai_opponent;
pub mod victory_system;
pub mod cheat_system;

pub use aircraft::*;
#[allow(unused_imports)]
pub use tower::*;
pub use combat::*;
pub use movement::*;
pub use turn_system::*;
pub use ai_opponent::*;
pub use victory_system::*;
pub use cheat_system::*;

use bevy::prelude::ResMut;

pub fn clear_processed_clicks(mut processed_clicks: ResMut<crate::input::selection::ProcessedClicks>) {
    processed_clicks.processed_ids.clear();
}