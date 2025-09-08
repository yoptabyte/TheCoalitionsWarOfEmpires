use bevy::prelude::*;

use crate::menu::{
    common::{GameState, MenuState, despawn_screen},
    main_menu::{main_menu_plugin, menu_action, OnMainMenuScreen},
    settings_menu::{settings_menu_plugin, OnSettingsMenuScreen, OnDisplaySettingsMenuScreen, OnSoundSettingsMenuScreen},
    pause_menu::pause_menu_plugin,
};

use crate::game_plugin::OnGameScreen;

use crate::UICamera;

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), (setup_ui_camera_for_menu, set_main_menu_state))
        .add_plugins(main_menu_plugin)
        .add_plugins(settings_menu_plugin)
        .add_plugins(pause_menu_plugin)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::Menu)),
        )
        // Temporarily disabled: .add_systems(OnEnter(GameState::Game), cleanup_all_menu_ui)
        .add_systems(OnEnter(GameState::Menu), cleanup_game_ui)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>)
        .add_systems(OnExit(MenuState::SettingsDisplay), despawn_screen::<OnDisplaySettingsMenuScreen>)
        .add_systems(OnExit(MenuState::SettingsSound), despawn_screen::<OnSoundSettingsMenuScreen>);
}

fn cleanup_all_menu_ui(
    mut commands: Commands,
    ui_elements: Query<Entity, With<Node>>,
) {
    println!("DEBUG: cleanup_all_menu_ui called - removing {} UI elements", ui_elements.iter().count());
    for entity in ui_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_ui_camera_for_menu(
    mut commands: Commands,
    ui_cameras: Query<Entity, With<UICamera>>,
) {
    // Remove ALL existing UI cameras first
    for entity in ui_cameras.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Then spawn exactly ONE UI camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // UI cameras have highest priority
                order: 10,  // Higher priority than 0
                ..default()
            },
            ..default()
        },
        UICamera,
    ));
}

fn cleanup_game_ui(
    mut commands: Commands,
    game_ui_elements: Query<Entity, With<OnGameScreen>>,
) {
    // Удаляем все элементы игрового UI при переходе в меню
    println!("🧹 DEBUG: cleanup_game_ui called - removing {} game UI elements", game_ui_elements.iter().count());
    for entity in game_ui_elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn set_main_menu_state(mut menu_state: ResMut<NextState<MenuState>>) {
    // Устанавливаем MenuState::Main при переходе в GameState::Menu
    println!("🔥 DEBUG: Setting MenuState::Main from Game->Menu transition");
    menu_state.set(MenuState::Main);
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