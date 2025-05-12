pub mod gizmos;
pub use gizmos::*;

pub mod menu;
pub use menu::*;

pub mod splash;
pub use splash::*;

pub mod money_ui;
pub use money_ui::*;

pub mod confirm_dialog;
pub use confirm_dialog::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct UICamera;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(Update, (
        confirm_dialog::confirm_dialog_button_system,
        confirm_dialog::handle_confirm_dialog_actions,
    ));
}