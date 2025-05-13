use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::input::keyboard::KeyCode;
use bevy_mod_picking::prelude::*;
use bevy_hanabi::ParticleEffectBundle;
use bevy_hanabi::ParticleEffect;

use crate::game::{Selectable, SelectedEntity, Ground, MovementOrder, ClickCircle, ClickEffectHandle, Enemy, EnemyTower, Farm};

/// Resource for tracking mouse position in world space
#[derive(Resource, Default)]
pub struct MouseWorldPosition(pub Option<Vec3>);

/// system for selecting an entity
pub fn select_entity_system(
    mut click_events: EventReader<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedEntity>,
    query_selectable: Query<(), (With<Selectable>, Without<Enemy>, Without<EnemyTower>)>,
    query_attackable: Query<Entity, Or<(With<Enemy>, With<EnemyTower>)>>,
    mut camera_movement_state: ResMut<crate::game::CameraMovementState>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        let is_selectable = query_selectable.get(event.target).is_ok();
        let is_attackable = query_attackable.get(event.target).is_ok();
        
        info!("select_entity_system: Click on entity {:?}, is_selectable: {}, is_attackable: {}", 
              event.target, is_selectable, is_attackable);
        
        if is_selectable {
            info!("select_entity_system: Clicked on selectable object {:?}, previously selected: {:?}", event.target, selected_entity.0);
            
            if selected_entity.0 != Some(event.target) {
                selected_entity.0 = Some(event.target);
                camera_movement_state.manual_camera_mode = false;
            }
            
            return;
        }
        
        if is_attackable && selected_entity.0.is_some() {
            info!("select_entity_system: Clicked on target object {:?}, keeping selected: {:?}", event.target, selected_entity.0);
            return;
        }
    }
}

/// system for updating mouse world position
pub fn update_mouse_world_position(
    mut mouse_position: ResMut<MouseWorldPosition>,
    camera: Query<(&Camera, &GlobalTransform), With<crate::game::MainCamera>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        if let Ok((camera, camera_transform)) = camera.get_single() {
            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                // Create a ray from the cursor and project it into 3D world
                // Find where the ray intersects with the y=0 plane
                let plane_normal = Vec3::Y;
                let plane_origin = Vec3::ZERO;
                
                let denominator = plane_normal.dot(*ray.direction);
                if denominator.abs() > 0.0001 {
                    let t = (plane_normal.dot(plane_origin - ray.origin)) / denominator;
                    if t >= 0.0 {
                        let world_position = ray.origin + *ray.direction * t;
                        mouse_position.0 = Some(world_position);
                        return;
                    }
                }
            }
        }
    }
    
    // If we couldn't get a valid position, don't change the existing one
}

/// processing ground clicks
pub fn handle_ground_clicks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    query_selectable: Query<(), With<Selectable>>,
    query_ground: Query<(), With<Ground>>,
    query_enemy: Query<(), With<Enemy>>,
    query_enemy_tower: Query<(), With<EnemyTower>>,
    query_farm: Query<(), With<Farm>>,
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
            // Проверяем, что объект не является фермой, врагом или башней
            if query_enemy.get(entity_to_move).is_err() && 
               query_enemy_tower.get(entity_to_move).is_err() && 
               query_farm.get(entity_to_move).is_err() {
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
                let entity_type = if query_enemy.get(entity_to_move).is_ok() { 
                    "enemy" 
                } else if query_enemy_tower.get(entity_to_move).is_ok() { 
                    "enemy tower" 
                } else { 
                    "farm" 
                };
                info!("handle_ground_clicks: Can't move {} object", entity_type);
            }
        }
    } else if clicked_on_selectable {
        click_circle.position = None;
    }
}