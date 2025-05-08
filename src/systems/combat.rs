use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::{SelectedEntity, Enemy, Health, Projectile, CanShoot};

/// system for processing clicks on enemy objects and creating a shot
pub fn handle_enemy_clicks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut click_events: EventReader<Pointer<Click>>,
    selected_entity: Res<SelectedEntity>,
    query_enemy: Query<Entity, With<Enemy>>,
    transform_query: Query<&Transform>,
    can_shoot_query: Query<&CanShoot>,
    time: Res<Time>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        if query_enemy.get(event.target).is_ok() {
            if let Some(shooter_entity) = selected_entity.0 {
                if let Ok(can_shoot) = can_shoot_query.get(shooter_entity) {
                    let current_time = time.elapsed_seconds();
                    
                    if current_time - can_shoot.last_shot >= can_shoot.cooldown {
                        if let (Ok(shooter_transform), Ok(target_transform)) = (
                            transform_query.get(shooter_entity),
                            transform_query.get(event.target)
                        ) {
                            let shooter_pos = shooter_transform.translation;
                            let target_pos = target_transform.translation;
                            
                            let distance = (target_pos - shooter_pos).length();
                            if distance <= can_shoot.range {
                                info!("handle_enemy_clicks: Shooting at enemy {:?} from distance {}", event.target, distance);
                                
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
                                
                                commands.entity(shooter_entity).insert(CanShoot {
                                    cooldown: can_shoot.cooldown,
                                    last_shot: current_time,
                                    range: can_shoot.range,
                                    damage: can_shoot.damage,
                                });
                            } else {
                                info!("handle_enemy_clicks: Target out of range (distance: {}, range: {})", distance, can_shoot.range);
                            }
                        }
                    } else {
                        info!("handle_enemy_clicks: Weapon on cooldown, remaining: {}", 
                              can_shoot.cooldown - (current_time - can_shoot.last_shot));
                    }
                }
            }
        }
    }
}

/// system for updating projectile flight and processing hits
pub fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    mut health_query: Query<&mut Health>,
    transform_query: Query<&Transform, Without<Projectile>>,
    time: Res<Time>,
) {
    for (projectile_entity, mut projectile_transform, projectile) in projectile_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(projectile.target) {
            let target_pos = target_transform.translation;
            let current_pos = projectile_transform.translation;
            
            let direction = target_pos - current_pos;
            
            if direction.length_squared() < 0.1 {
                if let Ok(mut health) = health_query.get_mut(projectile.target) {
                    health.current -= projectile.damage;
                    info!("update_projectiles: Hit target, remaining health: {}/{}", health.current, health.max);
                    
                    if health.current <= 0.0 {
                        info!("update_projectiles: Target destroyed!");
                        commands.entity(projectile.target).despawn();
                    }
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
            commands.entity(projectile_entity).despawn();
        }
    }
}

/// system for displaying health above objects (can be added in the future)
pub fn display_health_system() {
    todo!("display_health_system");
} 