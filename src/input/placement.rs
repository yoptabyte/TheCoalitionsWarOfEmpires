use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::components::{PlacementPending, PendingPurchase};
use crate::game::{Ground, ClickCircle, ShapeType};
use crate::input::MouseWorldPosition;
use crate::ui::money_ui::place_shape;

/// System for updating the position of objects waiting for placement
pub fn update_pending_placement_position(
    mouse_world_position: Res<MouseWorldPosition>,
    mut query: Query<(&mut Transform, &ShapeType), With<PlacementPending>>,
) {
    // If there's no mouse position, exit
    if mouse_world_position.0.is_none() {
        return;
    }
    
    let mouse_pos = mouse_world_position.0.unwrap();
    
    for (mut transform, shape_type) in query.iter_mut() {
        // Calculate base height depending on object type
        let base_height = match shape_type {
            ShapeType::Cube => 0.5,
            ShapeType::Infantry => 0.5,
            ShapeType::Airplane => 10.0,
            ShapeType::Tower => 1.5,
            ShapeType::Farm => 0.5,
            ShapeType::Mine => 1.5,
            ShapeType::SteelFactory => 2.0,
            ShapeType::PetrochemicalPlant => 1.75,
            ShapeType::Trench => 0.25,
        };
        
        // Set object position under cursor
        transform.translation = Vec3::new(
            mouse_pos.x,
            base_height,
            mouse_pos.z
        );
    }
}

/// System for handling ground clicks when placing a new object
/// IMPORTANT: This system only handles clicks for placing new objects,
/// not for moving existing ones (which are handled in handle_ground_clicks)
pub fn handle_placement_clicks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    query_ground: Query<(), With<Ground>>,
    pending_placement_query: Query<(Entity, &ShapeType), With<PlacementPending>>,
    mut pending_purchase: ResMut<PendingPurchase>,
    mut placement_state: ResMut<crate::game::PlacementState>,
    mut click_circle: ResMut<ClickCircle>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    player_faction: Res<crate::game::units::PlayerFaction>,
) {
    // First handle the new PlacementState system
    if placement_state.active && placement_state.shape_type.is_some() {
        let mut clicked_on_ground = false;
        let mut ground_click_position: Option<Vec3> = None;
        
        for event in click_events.read() {
            // Only process left clicks
            if event.button != PointerButton::Primary {
                continue;
            }
            
            info!("handle_placement_clicks: Processing PlacementState click event on target {:?}", event.target);
            
            if query_ground.get(event.target).is_ok() {
                clicked_on_ground = true;
                if let Some(position) = event.hit.position {
                    ground_click_position = Some(position);
                    info!("handle_placement_clicks: Ground selected for PlacementState placement at position {:?}", position);
                } else {
                    info!("handle_placement_clicks: Clicked on ground for PlacementState but no position available");
                }
            }
        }
        
        if clicked_on_ground && ground_click_position.is_some() {
            let target_position = ground_click_position.unwrap();
            
            if let Some(shape_type) = placement_state.shape_type.as_ref() {
                info!("handle_placement_clicks: Placing PlacementState object of type {:?} at position {:?}", shape_type, target_position);
                
                // Create object at click position using place_shape function
                place_shape(
                    &mut commands, 
                    shape_type.clone(), 
                    target_position, 
                    &mut meshes, 
                    &mut materials,
                    &asset_server,
                    &player_faction,
                    placement_state.unit_type_index,
                );
                
                // Update click circle display information
                click_circle.position = Some(target_position);
                click_circle.spawn_time = Some(time.elapsed_seconds());
                
                // Reset placement state
                placement_state.active = false;
                placement_state.shape_type = None;
                placement_state.unit_type_index = None;
                
                info!("handle_placement_clicks: PlacementState object placed successfully, placement mode deactivated");
                return; // Exit since purchase has been handled
            }
        }
    }
    // Handle the old PendingPurchase system for backward compatibility
    else if pending_purchase.shape_type.is_some() {
        let mut clicked_on_ground = false;
        let mut ground_click_position: Option<Vec3> = None;
        
        for event in click_events.read() {
            // Only process left clicks
            if event.button != PointerButton::Primary {
                continue;
            }
            
            info!("handle_placement_clicks: Processing placement click event on target {:?}", event.target);
            
            if query_ground.get(event.target).is_ok() {
                clicked_on_ground = true;
                if let Some(position) = event.hit.position {
                    ground_click_position = Some(position);
                    info!("handle_placement_clicks: Ground selected for object placement at position {:?}", position);
                } else {
                    info!("handle_placement_clicks: Clicked on ground for placement but no position information available");
                }
            }
        }
        
        if clicked_on_ground && ground_click_position.is_some() {
            let target_position = ground_click_position.unwrap();
            
            // Use as_ref() to get a reference instead of ownership
            if let Some(shape_type) = pending_purchase.shape_type.as_ref() {
                info!("handle_placement_clicks: Placing new object of type {:?} at position {:?}", shape_type, target_position);
                
                // Create object at click position using simplified function
                place_shape(
                    &mut commands, 
                    shape_type.clone(), 
                    target_position, 
                    &mut meshes, 
                    &mut materials,
                    &asset_server,
                    &player_faction,
                    None, // Old system doesn't have unit_type_index, so use default (0)
                );
                
                // Update click circle display information
                click_circle.position = Some(target_position);
                click_circle.spawn_time = Some(time.elapsed_seconds());
                
                // Reset pending purchase
                pending_purchase.shape_type = None;
                pending_purchase.cost_paid = false;
                
                info!("handle_placement_clicks: Object placed successfully, placement mode deactivated");
                return; // Exit since purchase has been handled
            }
        }
    }
    
    // If there are no objects with PlacementPending component, exit
    // This branch is for backward compatibility
    if pending_placement_query.is_empty() {
        return;
    }

    info!("handle_placement_clicks: Processing click events for {} older-style pending placement objects", pending_placement_query.iter().count());

    let mut clicked_on_ground = false;
    let mut ground_click_position: Option<Vec3> = None;
    
    for event in click_events.read() {
        // Only process left clicks
        if event.button != PointerButton::Primary {
            continue;
        }
        
        info!("handle_placement_clicks: Processing click event on target {:?}", event.target);
        
        if query_ground.get(event.target).is_ok() {
            clicked_on_ground = true;
            if let Some(position) = event.hit.position {
                ground_click_position = Some(position);
                info!("handle_placement_clicks: Clicked on ground at position {:?}", position);
            } else {
                info!("handle_placement_clicks: Clicked on ground but no position information available");
            }
        }
    }
    
    if clicked_on_ground && ground_click_position.is_some() {
        let target_position = ground_click_position.unwrap();
        
        // Get first object waiting for placement
        if let Some((entity, shape_type)) = pending_placement_query.iter().next() {
            info!("handle_placement_clicks: Placing object of type {:?} at position {:?}", shape_type, target_position);
            
            // Remove placement pending component and update position
            commands.entity(entity)
                .remove::<PlacementPending>();
            
            // Set object position
            commands.entity(entity)
                .insert(Transform::from_translation(target_position));
            
            // Update click circle display information
            click_circle.position = Some(target_position);
            click_circle.spawn_time = Some(time.elapsed_seconds());
        }
    } else if !clicked_on_ground {
        info!("handle_placement_clicks: No click on ground detected");
    }
} 