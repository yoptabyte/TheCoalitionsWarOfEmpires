use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_hanabi::ParticleEffectBundle;
use bevy_hanabi::ParticleEffect;
use bevy_rapier3d::prelude::*;

use crate::game::{Selectable, SelectedEntity, Ground, MovementOrder, ClickCircle, ClickEffectHandle, Enemy, EnemyTower, Farm, Mine, SteelFactory, PetrochemicalPlant, ShapeType};
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

/// system for selecting an entity
pub fn select_entity_system(
    mut click_events: EventReader<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedEntity>,
    query_selectable: Query<(), (With<Selectable>, Without<Enemy>, Without<EnemyTower>, Without<Farm>, Without<Mine>, Without<SteelFactory>, Without<PetrochemicalPlant>)>,
    query_attackable: Query<Entity, Or<(With<Enemy>, With<EnemyTower>)>>,
    mut camera_movement_state: ResMut<crate::game::CameraMovementState>,
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
    mut click_events: EventReader<Pointer<Click>>,
    query_ground: Query<(), With<Ground>>,
    mut click_circle: ResMut<ClickCircle>,
    time: Res<Time>,
    click_effect_handle: Res<ClickEffectHandle>,
    mut placement_state: ResMut<crate::game::PlacementState>,
    mut processed_clicks: ResMut<ProcessedClicks>,
    turn_state: Res<TurnState>,
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
        
        // Create object based on its type at click position
        match shape_type {
            ShapeType::Cube => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.8, 0.7, 0.6),
                            ..default()
                        }),
                        transform: Transform::from_translation(target_point + Vec3::new(0.0, 0.5, 0.0)),
                        ..default()
                    },
                    shape_type,
                    crate::game::components::Selectable,
                    crate::game::components::HoveredOutline,
                    crate::game::components::Health {
                        current: 100.0,
                        max: 100.0,
                    },
                    crate::game::components::CanShoot {
                        cooldown: 1.0,
                        last_shot: 0.0,
                        range: 10.0,
                        damage: 10.0,
                    },
                    crate::game::components::Tank,
                    RigidBody::Dynamic,
                    Collider::cuboid(0.5, 0.5, 0.5), // Коллайдер танка
                    LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y, // Заблокируем вращение и движение по Y
                    Restitution::coefficient(0.0), // Без отскока
                    Friction::coefficient(0.8), // Трение
                    PickableBundle::default(),
                    Name::new("Tank"),
                ));
            }
            ShapeType::Infantry => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(Sphere::new(0.5))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.2, 0.5, 0.8),
                            ..default()
                        }),
                        transform: Transform::from_translation(target_point + Vec3::new(0.0, 0.5, 0.0)),
                        ..default()
                    },
                    shape_type,
                    crate::game::Selectable,
                    crate::game::HoveredOutline,
                    crate::game::Health {
                        current: 60.0,
                        max: 60.0,
                    },
                    crate::game::CanShoot {
                        cooldown: 0.8,
                        last_shot: 0.0,
                        range: 12.0,
                        damage: 8.0,
                    },
                    RigidBody::Dynamic,
                    Collider::ball(0.5), // Коллайдер пехоты
                    LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y, // Заблокируем вращение и движение по Y
                    Restitution::coefficient(0.0), // Без отскока
                    Friction::coefficient(0.8), // Трение
                    PickableBundle::default(),
                    Name::new("Infantry"),
                ));
            }
            ShapeType::Airplane => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 4.0))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.8, 0.8, 0.8),
                            ..default()
                        }),
                        transform: Transform::from_translation(target_point + Vec3::new(0.0, 10.0, 0.0)),
                        ..default()
                    },
                    shape_type,
                    crate::game::components::Selectable,
                    crate::game::components::HoveredOutline,
                    crate::game::components::Aircraft {
                        height: 10.0,
                        speed: 5.0,
                    },
                    crate::game::components::Health {
                        current: 75.0,
                        max: 75.0,
                    },
                    crate::game::components::CanShoot {
                        cooldown: 0.5,
                        last_shot: 0.0,
                        range: 20.0,
                        damage: 15.0,
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(1.0, 0.25, 2.0), // Коллайдер самолета
                    LockedAxes::ROTATION_LOCKED, // Заблокируем только вращение, самолеты могут двигаться по Y
                    Restitution::coefficient(0.0), // Без отскока
                    Friction::coefficient(0.0), // Без трения в воздухе
                    PickableBundle::default(),
                ));
            }
            ShapeType::Tower => {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(Cuboid::new(1.5, 3.0, 1.5))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.5, 0.5, 0.5),
                            ..default()
                        }),
                        transform: Transform::from_translation(target_point + Vec3::new(0.0, 1.5, 0.0)),
                        ..default()
                    },
                    shape_type,
                    crate::game::components::Selectable,
                    crate::game::components::HoveredOutline,
                    PickableBundle::default(),
                    crate::game::components::Tower {
                        height: 3.0,
                    },
                    crate::game::components::Health {
                        current: 200.0,
                        max: 200.0,
                    },
                    crate::game::components::CanShoot {
                        cooldown: 2.0,
                        last_shot: 0.0,
                        range: 25.0,
                        damage: 20.0,
                    },
                ));
            }
            ShapeType::Farm => {
                crate::game::farm::spawn_forest_farm(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    target_point,
                );
            }
            ShapeType::Mine => {
                crate::game::mine::spawn_inactive_mine(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    target_point,
                );
            }
            ShapeType::SteelFactory => {
                crate::game::steel_factory::spawn_inactive_steel_factory(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    target_point,
                );
            }
            ShapeType::PetrochemicalPlant => {
                crate::game::petrochemical_plant::spawn_inactive_petrochemical_plant(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    target_point,
                );
            }
            ShapeType::Trench => {
                crate::game::spawn_constructing_trench(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    target_point,
                );
            }
        }
        
        // Reset placement mode after successful spawn
        placement_state.active = false;
        placement_state.shape_type = None;
        
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