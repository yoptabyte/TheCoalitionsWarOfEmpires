use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::components::{Tank as TankMarker, Enemy, Health, HoveredOutline};

/// Marker component for entities that need scene colliders
#[derive(Component)]
pub struct NeedsSceneCollider {
    pub collider_shape: ColliderShape,
}

/// Component to mark a mesh child as belonging to a clickable parent
#[derive(Component)]
pub struct ChildOfClickable {
    pub parent: Entity,
}

#[derive(Clone)]
pub enum ColliderShape {
    Tank,
    Infantry,
    Aircraft,
    Building,
}

/// System to automatically add colliders to Enemy entities with SceneBundle
pub fn add_enemy_scene_colliders(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Children), (With<crate::game::Enemy>, Added<Children>)>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
    child_query: Query<&ChildOfClickable>,
) {
    for (enemy_entity, children) in enemy_query.iter() {
        info!("Processing NEW Enemy entity {} with {} children", enemy_entity.index(), children.len());
        
        // Find all mesh children and add colliders to them
        for &child in children.iter() {
            // Skip if already processed
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok(_) = mesh_query.get(child) {
                info!("Adding collider to Enemy mesh child {}", child.index());
                
                // Add collider and picking to the mesh child
                commands.entity(child).insert((
                    Collider::cuboid(1.0, 1.0, 1.0), // Generic enemy collider
                    PickableBundle::default(),
                    Sensor, // Make it a sensor so it doesn't interfere with physics
                    ChildOfClickable { parent: enemy_entity },
                ));
            }
        }
    }
}

/// System to recursively find and add colliders to ALL Enemy mesh descendants
pub fn add_enemy_deep_scene_colliders(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<crate::game::Enemy>, Added<Children>)>,
    children_query: Query<&Children>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
) {
    for enemy_entity in enemy_query.iter() {
        info!("Processing Enemy entity {} for deep colliders", enemy_entity.index());
        
        // Recursively find all mesh descendants
        let mut mesh_entities = Vec::new();
        find_mesh_descendants(enemy_entity, &children_query, &mesh_query, &mut mesh_entities);
        
        info!("Found {} mesh descendants for Enemy {}", mesh_entities.len(), enemy_entity.index());
        
        for mesh_entity in mesh_entities {
            commands.entity(mesh_entity).insert((
                Collider::cuboid(0.8, 0.8, 0.8), // Enemy mesh collider
                PickableBundle::default(),
                Sensor,
                ChildOfClickable { parent: enemy_entity },
            ));
            
            info!("Added deep mesh collider to {} for Enemy parent {}", mesh_entity.index(), enemy_entity.index());
        }
    }
}

/// System to add colliders to loaded scene meshes
pub fn add_scene_colliders(
    mut commands: Commands,
    needs_collider_query: Query<(Entity, &NeedsSceneCollider, &Children)>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
) {
    for (parent_entity, needs_collider, children) in needs_collider_query.iter() {
        // Find all mesh children
        for &child in children.iter() {
            if let Ok(_) = mesh_query.get(child) {
                // Add collider to mesh child
                let collider = match needs_collider.collider_shape {
                    ColliderShape::Tank => Collider::cuboid(0.8, 0.6, 1.2),
                    ColliderShape::Infantry => Collider::ball(0.5),
                    ColliderShape::Aircraft => Collider::cuboid(1.2, 0.3, 2.0),
                    ColliderShape::Building => Collider::cuboid(1.0, 0.5, 1.0),
                };

                // Add a marker component to the child to identify it as clickable
                commands.entity(child).insert((
                    collider,
                    PickableBundle::default(),
                    ChildOfClickable { parent: parent_entity },
                ));
                
                info!("Added collider to mesh child {} of parent {}", child.index(), parent_entity.index());
            }
        }
        
        // Remove the marker component once processed
        commands.entity(parent_entity).remove::<NeedsSceneCollider>();
    }
}

/// System to recursively search for mesh children in deeply nested scenes
pub fn add_deep_scene_colliders(
    mut commands: Commands,
    needs_collider_query: Query<(Entity, &NeedsSceneCollider), Added<NeedsSceneCollider>>,
    children_query: Query<&Children>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
) {
    for (parent_entity, needs_collider) in needs_collider_query.iter() {
        // Recursively find all mesh descendants
        let mut mesh_entities = Vec::new();
        find_mesh_descendants(parent_entity, &children_query, &mesh_query, &mut mesh_entities);
        
        for mesh_entity in mesh_entities {
            let collider = match needs_collider.collider_shape {
                ColliderShape::Tank => Collider::cuboid(0.8, 0.6, 1.2),
                ColliderShape::Infantry => Collider::ball(0.5),
                ColliderShape::Aircraft => Collider::cuboid(1.2, 0.3, 2.0),
                ColliderShape::Building => Collider::cuboid(1.0, 0.5, 1.0),
            };

            commands.entity(mesh_entity).insert((
                collider,
                PickableBundle::default(),
            ));
            
            info!("Added deep collider to mesh {} for parent {}", mesh_entity.index(), parent_entity.index());
        }
    }
}

fn find_mesh_descendants(
    entity: Entity,
    children_query: &Query<&Children>,
    mesh_query: &Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
    mesh_entities: &mut Vec<Entity>,
) {
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            // Check if this child is a mesh
            if mesh_query.get(child).is_ok() {
                mesh_entities.push(child);
            }
            
            // Recursively search this child's children
            find_mesh_descendants(child, children_query, mesh_query, mesh_entities);
        }
    }
}

/// System to handle clicks on mesh children - just log for now
pub fn handle_child_clicks(
    mut click_events_reader: EventReader<Pointer<Click>>,
    child_query: Query<&ChildOfClickable>,
) {
    for event in click_events_reader.read() {
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            info!("Click detected on mesh child {} (parent: {})", 
                  event.target.index(), child_of_clickable.parent.index());
        }
    }
}

/// System to handle hover events on mesh children and forward them to parent entities
pub fn handle_child_hover(
    mut commands: Commands,
    mut over_events: EventReader<Pointer<Over>>,
    mut out_events: EventReader<Pointer<Out>>,
    child_query: Query<&ChildOfClickable>,
    parent_query: Query<Entity>, // Query to check if parent still exists
) {
    // Handle mouse over events
    for event in over_events.read() {
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            // Check if parent still exists before trying to modify it
            if parent_query.get(child_of_clickable.parent).is_ok() {
                commands.entity(child_of_clickable.parent).insert(HoveredOutline);
            }
        }
    }
    
    // Handle mouse out events
    for event in out_events.read() {
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            // Check if parent still exists before trying to modify it
            if parent_query.get(child_of_clickable.parent).is_ok() {
                commands.entity(child_of_clickable.parent).remove::<HoveredOutline>();
            }
        }
    }
}