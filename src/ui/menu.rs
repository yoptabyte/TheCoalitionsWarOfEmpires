use bevy::prelude::*;

use crate::menu::{
    common::{GameState, MenuState, DisplayQuality, Volume, despawn_screen},
    main_menu::{main_menu_plugin, menu_action, menu_setup, OnMainMenuScreen},
    settings_menu::{settings_menu_plugin, OnSettingsMenuScreen, OnDisplaySettingsMenuScreen, OnSoundSettingsMenuScreen},
};

use crate::UICamera;

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_plugins(main_menu_plugin)
        .add_plugins(settings_menu_plugin)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnEnter(GameState::Game), cleanup_all_menu_ui);
}

fn cleanup_all_menu_ui(
    mut commands: Commands,
    ui_elements: Query<Entity, With<Node>>,
    ui_cameras: Query<Entity, With<UICamera>>,
) {
    for entity in ui_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    for camera in ui_cameras.iter() {
        commands.entity(camera).despawn_recursive();
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct SelectedOption; 