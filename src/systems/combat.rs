use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::{SelectedEntity, Enemy, Health, Projectile, CanShoot, Tower, EnemyTower, ShapeType};

/// system for processing clicks on attackable objects (enemies or towers) and creating a shot
pub fn handle_attacks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut click_events: EventReader<Pointer<Click>>,
    selected_entity: Res<SelectedEntity>,
    query_enemy: Query<Entity, With<Enemy>>,
    query_enemy_tower: Query<Entity, With<EnemyTower>>,
    transform_query: Query<&Transform>,
    can_shoot_query: Query<&CanShoot>,
    time: Res<Time>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        // Подробное логирование клика
        info!("handle_attacks: Click event detail - target: {:?}, hit position: {:?}", 
              event.target, event.hit.position);
        
        let is_enemy = query_enemy.get(event.target).is_ok();
        let is_enemy_tower = query_enemy_tower.get(event.target).is_ok();
        
        info!("handle_attacks: Click detected on entity {:?}, is_enemy: {}, is_enemy_tower: {}", 
              event.target, is_enemy, is_enemy_tower);
        
        let is_valid_target = is_enemy || is_enemy_tower;
        
        if is_valid_target {
            info!("handle_attacks: Valid target detected: {:?}", event.target);
            
            if let Some(shooter_entity) = selected_entity.0 {
                info!("handle_attacks: Selected shooter entity: {:?}", shooter_entity);
                
                if let Ok(can_shoot) = can_shoot_query.get(shooter_entity) {
                    info!("handle_attacks: Shooter has CanShoot component with range: {}", can_shoot.range);
                    
                    let current_time = time.elapsed_seconds();
                    
                    if current_time - can_shoot.last_shot >= can_shoot.cooldown {
                        info!("handle_attacks: Cooldown check passed, attempting to shoot");
                        
                        if let (Ok(shooter_transform), Ok(target_transform)) = (
                            transform_query.get(shooter_entity),
                            transform_query.get(event.target)
                        ) {
                            let shooter_pos = shooter_transform.translation;
                            let target_pos = target_transform.translation;
                            
                            let distance = (target_pos - shooter_pos).length();
                            info!("handle_attacks: Distance to target: {}, range: {}", distance, can_shoot.range);
                            
                            if distance <= can_shoot.range {
                                let target_type = if is_enemy { "enemy" } else { "tower" };
                                info!("handle_attacks: Shooting at {} {:?} from distance {}", target_type, event.target, distance);
                                
                                // Создаем снаряд
                                let projectile_mesh = meshes.add(Mesh::from(Sphere::new(0.1)));
                                commands.spawn((
                                    PbrBundle {
                                        mesh: projectile_mesh,
                                        material: materials.add(Color::rgb(1.0, 0.7, 0.0)),
                                        transform: Transform::from_translation(shooter_pos + Vec3::new(0.0, 0.5, 0.0)),
                                        ..default()
                                    },
                                    Projectile {
                                        target: event.target,
                                        speed: 5.0,
                                        damage: can_shoot.damage,
                                    },
                                    Name::new("projectile"),
                                ));
                                
                                // Обновляем время последнего выстрела
                                commands.entity(shooter_entity).insert(CanShoot {
                                    cooldown: can_shoot.cooldown,
                                    last_shot: current_time,
                                    range: can_shoot.range,
                                    damage: can_shoot.damage,
                                });
                            } else {
                                info!("handle_attacks: Target out of range (distance: {}, range: {})", distance, can_shoot.range);
                            }
                        } else {
                            info!("handle_attacks: Failed to get transforms for shooter or target");
                        }
                    } else {
                        info!("handle_attacks: Weapon on cooldown, remaining: {}", 
                              can_shoot.cooldown - (current_time - can_shoot.last_shot));
                    }
                } else {
                    info!("handle_attacks: Selected entity cannot shoot");
                }
            } else {
                info!("handle_attacks: No shooter entity selected");
            }
        } else {
            info!("handle_attacks: Not a valid attack target: {:?}", event.target);
        }
    }
}

/// system for updating projectile flight and processing hits
pub fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    mut health_query: Query<&mut Health>,
    transform_query: Query<&Transform, Without<Projectile>>,
    query_enemy: Query<(), With<Enemy>>,
    query_enemy_tower: Query<(), With<EnemyTower>>,
    time: Res<Time>,
) {
    for (projectile_entity, mut projectile_transform, projectile) in projectile_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(projectile.target) {
            let target_pos = target_transform.translation;
            let current_pos = projectile_transform.translation;
            
            let direction = target_pos - current_pos;
            
            if direction.length_squared() < 0.1 {
                let is_enemy = query_enemy.get(projectile.target).is_ok();
                let is_enemy_tower = query_enemy_tower.get(projectile.target).is_ok();
                let target_type = if is_enemy { "enemy" } else if is_enemy_tower { "tower" } else { "unknown" };
                
                info!("update_projectiles: Projectile hit target: {:?}, type: {}", projectile.target, target_type);
                
                if let Ok(mut health) = health_query.get_mut(projectile.target) {
                    health.current -= projectile.damage;
                    info!("update_projectiles: Hit target {}, remaining health: {}/{}", target_type, health.current, health.max);
                    
                    if health.current <= 0.0 {
                        info!("update_projectiles: Target {} destroyed!", target_type);
                        commands.entity(projectile.target).despawn_recursive();
                    }
                } else {
                    info!("update_projectiles: Target has no Health component: {:?}", projectile.target);
                }
                
                commands.entity(projectile_entity).despawn();
            } else {
                let movement = direction.normalize() * projectile.speed * time.delta_seconds();
                projectile_transform.translation += movement;
                
                if direction.length_squared() > 0.001 {
                    let target_rotation = Quat::from_rotation_arc(Vec3::Z, direction.normalize());
                    projectile_transform.rotation = target_rotation;
                }
            }
        } else {
            info!("update_projectiles: Target entity no longer exists: {:?}", projectile.target);
            commands.entity(projectile_entity).despawn();
        }
    }
}

/// system for displaying health above objects (can be added in the future)
pub fn display_health_system() {
    todo!("display_health_system");
}

/// System to handle damage from trenches to enemy infantry
pub fn handle_trench_damage(
    trench_query: Query<&Transform, With<crate::game::Trench>>,
    mut enemy_query: Query<(&Transform, &mut Health), (With<Enemy>, With<ShapeType>)>,
    time: Res<Time>,
) {
    // Basic implementation for future addition of full functionality
    // In the future there will be:
    // 1. Checking distance from enemies to trenches
    // 2. If enemy attacks trench (is nearby), trench deals damage
    // 3. Implementation of trench health and destruction

    // Note: full functionality will be added later as specified in requirements
    
    for trench_transform in trench_query.iter() {
        let trench_pos = trench_transform.translation;
        
        // Temporarily just logging trench position info for debugging
        info!("Trench is at position: {:?}", trench_pos);
        
        // Check all Infantry type enemies within 3 units radius of trench
        for (enemy_transform, mut _health) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation;
            let distance = trench_pos.distance(enemy_pos);
            
            if distance < 3.0 {
                // In the future here will be:
                // health.current -= 1.0 * time.delta_seconds();
                info!("Enemy is near trench at position: {:?}, distance: {}", enemy_pos, distance);
            }
        }
    }
} 