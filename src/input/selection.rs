use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_hanabi::ParticleEffectBundle;
use bevy_hanabi::ParticleEffect;

use crate::game::{Selectable, SelectedEntity, Ground, MovementOrder, ClickCircle, ClickEffectHandle, Enemy};

/// system for selecting an entity
pub fn select_entity_system(
    mut click_events: EventReader<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedEntity>,
    query_selectable: Query<(), (With<Selectable>, Without<Enemy>)>,
    mut camera_movement_state: ResMut<crate::game::CameraMovementState>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        if query_selectable.get(event.target).is_ok() {
            info!("select_entity_system: Clicked on selectable object {:?}, previously selected: {:?}", event.target, selected_entity.0);
            
            if selected_entity.0 != Some(event.target) {
                selected_entity.0 = Some(event.target);
                camera_movement_state.manual_camera_mode = false;
            }
            
            return;
        }
    }
}

/// processing ground clicks
pub fn handle_ground_clicks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    query_selectable: Query<(), With<Selectable>>,
    query_ground: Query<(), With<Ground>>,
    query_enemy: Query<(), With<Enemy>>,
    mut click_circle: ResMut<ClickCircle>,
    time: Res<Time>,
    click_effect_handle: Res<ClickEffectHandle>,
    selected_entity_res: Res<SelectedEntity>,
) {
    let mut clicked_on_selectable = false;
    let mut clicked_on_ground = false;
    let mut ground_click_position: Option<Vec3> = None;
    
    for event in click_events.read() {
        // processing only left mouse clicks
        if event.button != PointerButton::Primary {
            continue;
        }
        
        if query_selectable.get(event.target).is_ok() {
            clicked_on_selectable = true;
            info!("handle_ground_clicks: clicked on selectable {:?}", event.target);
        }
        
        if query_ground.get(event.target).is_ok() {
            clicked_on_ground = true;
            if let Some(position) = event.hit.position {
                ground_click_position = Some(position);
                info!("handle_ground_clicks: clicked on ground {:?}", position);
            }
        }
    }
    
    if !clicked_on_selectable && clicked_on_ground && selected_entity_res.0.is_some() && ground_click_position.is_some() {
        let target_point = ground_click_position.unwrap();
        
        if let Some(entity_to_move) = selected_entity_res.0 {
            if query_enemy.get(entity_to_move).is_err() {
                info!("handle_ground_clicks: Sending order to move for {:?} to point {:?}", entity_to_move, target_point);
                commands.entity(entity_to_move).insert(MovementOrder(target_point));
                
                click_circle.position = Some(target_point);
                click_circle.spawn_time = Some(time.elapsed_seconds());
                
                commands.spawn((
                    Name::new("click_particles"),
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(click_effect_handle.0.clone()),
                        transform: Transform::from_translation(target_point),
                        ..default()
                    },
                ));
            } else {
                info!("handle_ground_clicks: Can't move enemy object");
            }
        }
    } else if clicked_on_selectable {
        click_circle.position = None;
    }
}