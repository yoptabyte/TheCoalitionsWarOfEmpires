use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_hanabi::ParticleEffectBundle;
use bevy_hanabi::ParticleEffect;
use bevy_rapier3d::prelude::*;

use crate::game::{Selectable, SelectedEntity, Ground, MovementOrder, ClickCircle, ClickEffectHandle, Enemy, EnemyTower, Farm, Mine, SteelFactory, PetrochemicalPlant, ShapeType, Health, LinkedToEnemy, Tank, Aircraft};
use crate::game::units::infantry::Infantry;
use crate::systems::turn_system::{TurnState, PlayerTurn};

/// Resource for tracking mouse position in world space
#[derive(Resource, Default)]
pub struct MouseWorldPosition(#[allow(dead_code)] pub Option<Vec3>);

/// Resource for tracking processed clicks
#[derive(Resource, Default)]
pub struct ProcessedClicks {
    /// IDs of clicks that have already been processed
    pub processed_ids: Vec<PointerId>,
}

/// Debug system to log all click events regardless of what they hit
pub fn debug_all_clicks(
    mut click_events: EventReader<Pointer<Click>>,
    turn_state: Res<TurnState>,
) {
    if turn_state.current_player != PlayerTurn::Human {
        return;
    }
    
    for event in click_events.read() {
        info!("🔥🔥🔥 DEBUG ALL CLICKS: Got click event - button: {:?}, target: {:?}, position: {:?}", 
              event.button, event.target, event.hit.position);
    }
}

/// Alternative unit selection system using mouse input and raycasting
pub fn raycast_unit_selection(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<crate::game::MainCamera>>,
    rapier_context: Res<RapierContext>,
    mut selected_entity: ResMut<SelectedEntity>,
    query_selectable: Query<(), (With<Selectable>, Without<Enemy>, Without<EnemyTower>)>,
    turn_state: Res<TurnState>,
    mut camera_movement_state: ResMut<crate::game::CameraMovementState>,
) {
    if turn_state.current_player != PlayerTurn::Human {
        return;
    }

    // Check for mouse click
    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        if let Some(cursor_position) = window.cursor_position() {
            if let Ok((camera, camera_transform)) = camera_q.get_single() {
                info!("🔥 RAYCAST: Mouse clicked at screen position {:?}", cursor_position);
                
                // Convert screen coordinates to world ray
                if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    info!("🔥 RAYCAST: Created ray from {:?} in direction {:?}", ray.origin, ray.direction);
                    
                    // Perform raycast
                    let hit = rapier_context.cast_ray(
                        ray.origin,
                        *ray.direction,
                        f32::MAX,
                        true,
                        QueryFilter::default()
                    );
                    
                    if let Some((entity, _toi)) = hit {
                        info!("🔥 RAYCAST: Hit entity {:?}", entity);
                        
                        // Check if the hit entity is selectable
                        if query_selectable.get(entity).is_ok() {
                            info!("🔥 RAYCAST: ✅ Entity {:?} is selectable! Selecting it.", entity);
                            selected_entity.0 = Some(entity);
                            camera_movement_state.manual_camera_mode = false;
                        } else {
                            info!("🔥 RAYCAST: ❌ Entity {:?} is not selectable", entity);
                        }
                    } else {
                        info!("🔥 RAYCAST: No entity hit");
                    }
                } else {
                    info!("🔥 RAYCAST: Could not create ray from cursor position");
                }
            } else {
                info!("🔥 RAYCAST: Could not get camera");
            }
        } else {
            info!("🔥 RAYCAST: No cursor position");
        }
    }
}

/// system for selecting an entity
pub fn select_entity_system(
    mut click_events: EventReader<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedEntity>,
    query_selectable: Query<(), (With<Selectable>, Without<Enemy>, Without<EnemyTower>)>,
    query_attackable: Query<Entity, Or<(With<Enemy>, With<EnemyTower>)>>,
    query_enemy_targetable: Query<Entity, (With<Enemy>, With<Health>)>, // Enemy units/buildings that can be targeted
    mut camera_movement_state: ResMut<crate::game::CameraMovementState>,
    turn_state: Res<TurnState>,
    // Add queries to debug what components entities actually have
    debug_query: Query<(Option<&Selectable>, Option<&Enemy>, Option<&Tank>, Option<&Infantry>, Option<&Aircraft>)>,
) {
    // Блокируем все клики во время хода ИИ
    if turn_state.current_player != PlayerTurn::Human {
        return;
    }
    
    // Log if we have any click events at all
    let click_count = click_events.len();
    if click_count > 0 {
        info!("🔥 select_entity_system: Received {} click events", click_count);
    }
    
    for event in click_events.read() {
        info!("🔥 RAW CLICK EVENT: button: {:?}, target: {:?}, position: {:?}", 
              event.button, event.target, event.hit.position);
              
        if event.button != PointerButton::Primary {
            info!("🔥 Ignoring non-primary button click: {:?}", event.button);
            continue;
        }
        
        // Debug: Check what components this entity has
        if let Ok((selectable, enemy, tank, infantry, aircraft)) = debug_query.get(event.target) {
            info!("🔥 CLICK DEBUG: Entity {:?} has - Selectable: {:?}, Enemy: {:?}, Tank: {:?}, Infantry: {:?}, Aircraft: {:?}", 
                  event.target, selectable.is_some(), enemy.is_some(), tank.is_some(), infantry.is_some(), aircraft.is_some());
        } else {
            info!("🔥 CLICK DEBUG: Entity {:?} - could not query components", event.target);
        }
        
        let is_selectable = query_selectable.get(event.target).is_ok();
        let is_attackable = query_attackable.get(event.target).is_ok();
        let is_enemy_targetable = query_enemy_targetable.get(event.target).is_ok();
        
        info!("select_entity_system: Click on entity {:?}, is_selectable: {}, is_attackable: {}, is_enemy_targetable: {}", 
              event.target, is_selectable, is_attackable, is_enemy_targetable);
        
        if is_selectable {
            info!("select_entity_system: ✅ Clicked on selectable object {:?}, previously selected: {:?}", event.target, selected_entity.0);
            
            if selected_entity.0 != Some(event.target) {
                selected_entity.0 = Some(event.target);
                camera_movement_state.manual_camera_mode = false;
            }
            
            return;
        }
        
        // Allow targeting enemy units/buildings for combat
        if (is_attackable || is_enemy_targetable) && selected_entity.0.is_some() {
            info!("select_entity_system: Clicked on enemy target {:?}, keeping selected: {:?}", event.target, selected_entity.0);
            // Don't return here - let combat system handle the attack!
        }
    }
}

/// System for handling clicks on enemy entities using bevy_mod_picking
pub fn handle_enemy_clicks(
    mut click_events: EventReader<Pointer<Click>>,
    enemy_query: Query<(Entity, &Transform, &Health), With<Enemy>>,
    collider_query: Query<&LinkedToEnemy>,
    selected_entity: Res<SelectedEntity>,
    turn_state: Res<TurnState>,
) {
    // Блокируем все клики во время хода ИИ
    if turn_state.current_player != PlayerTurn::Human {
        return;
    }

    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        let mut target_enemy: Option<(Entity, &Transform, &Health)> = None;
        
        // Проверяем, кликнули ли напрямую по Enemy entity с Health
        if let Ok(enemy_data) = enemy_query.get(event.target) {
            target_enemy = Some(enemy_data);
        }
        // Проверяем, кликнули ли по клик-коллайдеру
        else if let Ok(linked) = collider_query.get(event.target) {
            if let Ok(enemy_data) = enemy_query.get(linked.0) {
                target_enemy = Some(enemy_data);
                info!("Click intercepted by collider, targeting linked enemy {:?}", linked.0);
            }
        }
        
        if let Some((enemy_entity, enemy_transform, enemy_health)) = target_enemy {
            info!("Clicked on enemy entity {:?} at position {:?} with health {:.1}/{:.1}", 
                  enemy_entity, enemy_transform.translation, enemy_health.current, enemy_health.max);
            
            if let Some(selected) = selected_entity.0 {
                info!("Selected unit {:?} will target enemy {:?}", selected, enemy_entity);
            } else {
                info!("No unit selected to attack enemy {:?}", enemy_entity);
            }
        }
    }
}

/// system for updating mouse world position
#[allow(dead_code)]
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
                
                if denominator.abs() > 0.00001 {
                    let t = (plane_normal.dot(plane_origin - ray.origin)) / denominator;
                    
                    if t >= 0.0 {
                        let world_position = ray.origin + *ray.direction * t;
                        
                        info!("Mouse world position: {:?}, ray dir: {:?}, camera pos: {:?}", 
                              world_position, ray.direction, camera_transform.translation());
                        
                        mouse_position.0 = Some(world_position);
                        return;
                    } else {
                        info!("Ray intersection with ground plane is behind camera, t={}", t);
                    }
                } else {
                    info!("Ray is nearly parallel to ground plane, denominator={}", denominator);
                }
            }
        }
    }
    
    // If we couldn't get a valid position, don't change the existing one
}

/// processing ground clicks for moving existing objects
pub fn handle_ground_clicks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    query_ground: Query<(), With<Ground>>,
    mut click_circle: ResMut<ClickCircle>,
    time: Res<Time>,
    click_effect_handle: Res<ClickEffectHandle>,
    selected_entity_res: Res<SelectedEntity>,
    turn_state: Res<TurnState>,
) {
    // Блокируем все клики во время хода ИИ
    if turn_state.current_player != PlayerTurn::Human {
        return;
    }
    let mut clicked_on_ground = false;
    let mut ground_click_position: Option<Vec3> = None;
    
    for event in click_events.read() {
        // processing only left mouse clicks
        if event.button != PointerButton::Primary {
            continue;
        }
        
        if query_ground.get(event.target).is_ok() {
            clicked_on_ground = true;
            if let Some(position) = event.hit.position {
                ground_click_position = Some(position);
                info!("handle_ground_clicks: clicked on ground at position {:?}", position);
            }
        }
    }
    
    // Logic for determining ground click for movement
    if clicked_on_ground && selected_entity_res.0.is_some() && ground_click_position.is_some() {
        let target_point = ground_click_position.unwrap();
        
        if let Some(entity_to_move) = selected_entity_res.0 {
            info!("handle_ground_clicks: Sending order to move for {:?} to point {:?}", entity_to_move, target_point);
            
            // Check if entity still exists before trying to move it
            if let Some(mut entity_commands) = commands.get_entity(entity_to_move) {
                // Send movement command
                entity_commands.insert(MovementOrder(target_point));
                
                // Update click circle display info
                click_circle.position = Some(target_point);
                click_circle.spawn_time = Some(time.elapsed_seconds());
                
                // Create particle effect at click location
                commands.spawn((
                    Name::new("click_particles"),
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(click_effect_handle.0.clone()),
                        transform: Transform::from_translation(target_point),
                        ..default()
                    },
                ));
            } else {
                info!("handle_ground_clicks: Entity {:?} no longer exists, cannot move", entity_to_move);
            }
        }
    }
}

/// System for handling clicks during object placement
pub fn handle_placement_clicks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut click_events: EventReader<Pointer<Click>>,
    query_ground: Query<(), With<Ground>>,
    mut click_circle: ResMut<ClickCircle>,
    time: Res<Time>,
    click_effect_handle: Res<ClickEffectHandle>,
    mut placement_state: ResMut<crate::game::PlacementState>,
    mut processed_clicks: ResMut<ProcessedClicks>,
    turn_state: Res<TurnState>,
    player_faction: Res<crate::game::units::PlayerFaction>,
) {
    // Блокируем все клики во время хода ИИ
    if turn_state.current_player != PlayerTurn::Human {
        return;
    }
    // If placement mode is not active, exit
    if !placement_state.active || placement_state.shape_type.is_none() {
        return;
    }

    let mut clicked_on_ground = false;
    let mut ground_click_position: Option<Vec3> = None;
    
    for event in click_events.read() {
        // process only left mouse button clicks
        if event.button != PointerButton::Primary {
            continue;
        }
        
        if query_ground.get(event.target).is_ok() {
            clicked_on_ground = true;
            if let Some(position) = event.hit.position {
                ground_click_position = Some(position);
                info!("handle_placement_clicks: clicked on ground at position {:?}", position);
                
                // Remember click event ID to avoid processing it again
                processed_clicks.processed_ids.push(event.pointer_id);
            }
        }
    }
    
    // Place object if clicked on ground
    if clicked_on_ground && ground_click_position.is_some() {
        let target_point = ground_click_position.unwrap();
        let shape_type = placement_state.shape_type.unwrap();
        
        info!("handle_placement_clicks: Placing object of type {:?} at position {:?}", shape_type, target_point);
        
        // Use proper place_shape function to create objects with 3D models
        info!("🔥🔥🔥 handle_placement_clicks (selection.rs): ABOUT TO CALL place_shape!!!");
        
        // Use player faction from system resources for proper model selection
        info!("🔥 Calling place_shape with shape_type: {:?}", shape_type);
        crate::ui::money_ui::place_shape(
            &mut commands,
            shape_type,
            target_point,
            &mut meshes,
            &mut materials,
            &asset_server,
            &player_faction,
        );
        info!("🔥 place_shape call completed!");
        
        // Reset placement mode after successful spawn
        placement_state.active = false;
        placement_state.shape_type = None;
        
        /*
        OLD PRIMITIVE CREATION CODE REMOVED - was creating Cuboid/Sphere primitives instead of 3D models
        The place_shape() function above now handles all object creation properly.
        
        */
        
        // Create particle effect at click location
        commands.spawn((
            Name::new("placement_particles"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(click_effect_handle.0.clone()),
                transform: Transform::from_translation(target_point),
                ..default()
            },
        ));
        
        // Set position for click circle display
        click_circle.position = Some(target_point);
        click_circle.spawn_time = Some(time.elapsed_seconds());
    }
}