use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::{SelectedEntity, Enemy, Health, CanShoot, EnemyTower, ShapeType};
use crate::systems::turn_system::{TurnState, PlayerTurn};

/// system for processing clicks on attackable objects (enemies or towers) with instant hit
pub fn handle_attacks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    selected_entity: Res<SelectedEntity>,
    query_enemy: Query<Entity, With<Enemy>>,
    query_enemy_tower: Query<Entity, With<EnemyTower>>,
    transform_query: Query<&Transform>,
    can_shoot_query: Query<&CanShoot>,
    mut health_query: Query<&mut Health>,
    time: Res<Time>,
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
        
        let is_enemy = query_enemy.get(event.target).is_ok();
        let is_enemy_tower = query_enemy_tower.get(event.target).is_ok();
        
        let is_valid_target = is_enemy || is_enemy_tower;
        
        if is_valid_target {
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
                                // Instant hit - apply damage immediately
                                if let Ok(mut health) = health_query.get_mut(event.target) {
                                    health.current -= can_shoot.damage;
                                    
                                    if health.current <= 0.0 {
                                        commands.entity(event.target).despawn_recursive();
                                    }
                                }
                                
                                // Update last shot time
                                commands.entity(shooter_entity).insert(CanShoot {
                                    cooldown: can_shoot.cooldown,
                                    last_shot: current_time,
                                    range: can_shoot.range,
                                    damage: can_shoot.damage,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}



/// System to handle damage from trenches to enemy infantry
pub fn handle_trench_damage(
    trench_query: Query<&Transform, With<crate::game::Trench>>,
    mut enemy_query: Query<(&Transform, &mut Health), (With<Enemy>, With<ShapeType>)>,
    _time: Res<Time>,
) {
    // Basic implementation for future addition of full functionality
    // In the future there will be:
    // 1. Checking distance from enemies to trenches
    // 2. If enemy attacks trench (is nearby), trench deals damage
    // 3. Implementation of trench health and destruction

    // Note: full functionality will be added later as specified in requirements
    
    for trench_transform in trench_query.iter() {
        let trench_pos = trench_transform.translation;
        
        // Check all Infantry type enemies within 3 units radius of trench
        for (enemy_transform, mut _health) in enemy_query.iter_mut() {
            let enemy_pos = enemy_transform.translation;
            let distance = trench_pos.distance(enemy_pos);
            
            if distance < 3.0 {
                // In the future here will be:
                // health.current -= 1.0 * time.delta_seconds();
            }
        }
    }
} 