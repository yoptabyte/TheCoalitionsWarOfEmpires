pub mod gizmos;
pub use gizmos::*;

pub mod menu;

pub mod splash;

pub mod money_ui;

pub mod confirm_dialog;

use bevy::prelude::*;
use crate::menu::common::GameState;
use crate::game_plugin::OnGameScreen;

#[derive(Component)]
pub struct UICamera;

pub fn show_placement_state(
    placement_state: Res<crate::game::PlacementState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    node_query: Query<Entity, With<PlacementStateText>>,
    root_node_query: Query<Entity, With<OnGameScreen>>,
) {
    for entity in node_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    if !placement_state.active || placement_state.shape_type.is_none() {
        return;
    }
    
    let object_type = match placement_state.shape_type.unwrap() {
        crate::game::ShapeType::Cube => "Танк",
        crate::game::ShapeType::Infantry => "Пехота",
        crate::game::ShapeType::Airplane => "Самолет",
        crate::game::ShapeType::Tower => "Башня",
        crate::game::ShapeType::Farm => "Ферма",
        crate::game::ShapeType::Mine => "Шахта",
        crate::game::ShapeType::SteelFactory => "Сталелитейный завод",
        crate::game::ShapeType::PetrochemicalPlant => "Нефтехимический завод",
        crate::game::ShapeType::Trench => "Окоп",
    };
    
    if let Ok(root) = root_node_query.get_single() {
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Выберите место для размещения: {}", object_type),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::RED,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.0),
                    left: Val::Px(10.0),
                    ..default()
                }),
                PlacementStateText,
                OnGameScreen,
            ));
        });
    }
}

#[derive(Component)]
pub struct PlacementStateText;

pub fn ui_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            show_placement_state,
            confirm_dialog::confirm_dialog_button_system,
            confirm_dialog::handle_confirm_dialog_actions,
        ).run_if(in_state(GameState::Game))
    );
}