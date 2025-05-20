pub mod aircraft;
pub mod tower;
pub mod combat;
pub mod movement;

pub use aircraft::*;
pub use tower::*;
pub use combat::*;
pub use movement::*;
use bevy::prelude::ResMut;

pub fn clear_processed_clicks(mut processed_clicks: ResMut<crate::input::selection::ProcessedClicks>) {
    processed_clicks.processed_ids.clear();
}