use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::game::{SelectedEntity, Enemy, Health, CanShoot, EnemyTower, ShapeType, Mine, SteelFactory, PetrochemicalPlant, LinkedToEnemy, Tank, Aircraft};
use crate::game::units::infantry::Infantry;
use crate::systems::turn_system::{TurnState, PlayerTurn};


/// system for processing clicks on attackable objects (enemies or towers) with instant hit
pub fn handle_attacks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut click_events: EventReader<Pointer<Click>>,
    selected_entity: Res<SelectedEntity>,
    // Combined query for all enemy entities
    query_enemies: Query<Entity, Or<(
        With<Enemy>,
        With<EnemyTower>,
        (With<Mine>, With<Enemy>),
        (With<SteelFactory>, With<Enemy>),
        (With<PetrochemicalPlant>, With<Enemy>)
    )>>,
    child_query: Query<&crate::game::scene_colliders::ChildOfClickable>,
    collider_query: Query<&LinkedToEnemy>,
    transform_query: Query<&Transform>,
    can_shoot_query: Query<&CanShoot>,
    mut health_query: Query<&mut Health>,
    // Combined query for unit types
    unit_type_query: Query<(
        Option<&Tank>, 
        Option<&Aircraft>, 
        Option<&Infantry>
    )>,
    time: Res<Time>,
    turn_state: Res<TurnState>,
) {
    // Блокируем все клики во время хода ИИ
    if turn_state.current_player != PlayerTurn::Human {
        info!("handle_attacks: Blocked - AI turn active");
        return;
    }
    
    let click_count = click_events.len();
    if click_count > 0 {
        info!("handle_attacks: Processing {} click events", click_count);
    }
    
    for event in click_events.read() {
        info!("handle_attacks: Click event received on entity {:?}, button: {:?}", event.target, event.button);
        if event.button != PointerButton::Primary {
            continue;
        }
        
        // Check direct targets first
        let is_valid_enemy = query_enemies.get(event.target).is_ok();
        
        info!("Target check: is_valid_enemy={}", is_valid_enemy);
        
        // Обрабатываем клики по дочерним элементам 3D моделей
        let mut target_entity = event.target;
        let mut is_child_of_enemy = false;
        let mut is_click_collider = false;
        
        // Сначала проверяем, есть ли ChildOfClickable компонент
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            info!("✅ Click on mesh child detected, redirecting to parent entity {}", child_of_clickable.parent.index());
            target_entity = child_of_clickable.parent;
            // Перепроверяем компоненты для родительского entity
            is_child_of_enemy = query_enemies.get(target_entity).is_ok();
            info!("Parent entity check: is_child_of_enemy={}", is_child_of_enemy);
        }
        
        // Проверяем LinkedToEnemy компонент
        if let Ok(linked) = collider_query.get(event.target) {
            info!("✅ Click on linked collider detected, redirecting to linked entity {}", linked.0.index());
            target_entity = linked.0;
            is_click_collider = true;
        }
        
        let is_valid_target = is_valid_enemy || is_child_of_enemy || is_click_collider;
        
        info!("handle_attacks: Click on entity {:?}, is_valid_target: {}", event.target, is_valid_target);
        
        if is_valid_target {
            info!("handle_attacks: Valid target clicked, selected_entity: {:?}", selected_entity.0);
            if let Some(shooter_entity) = selected_entity.0 {
                if let Ok(can_shoot) = can_shoot_query.get(shooter_entity) {
                    info!("handle_attacks: Shooter has CanShoot component, damage: {}, range: {}", can_shoot.damage, can_shoot.range);
                    let current_time = time.elapsed_seconds();
                    
                    if current_time - can_shoot.last_shot >= can_shoot.cooldown {
                        if let (Ok(shooter_transform), Ok(target_transform)) = (
                            transform_query.get(shooter_entity),
                            transform_query.get(target_entity)
                        ) {
                            let shooter_pos = shooter_transform.translation;
                            let target_pos = target_transform.translation;
                            let distance = (target_pos - shooter_pos).length();
                            
                            info!("handle_attacks: Distance {} <= range {}: {}", distance, can_shoot.range, distance <= can_shoot.range);
                            if distance <= can_shoot.range {
                                // Instant hit - apply damage immediately
                                if let Ok(mut health) = health_query.get_mut(target_entity) {
                                    let old_health = health.current;
                                    health.current -= can_shoot.damage;
                                    info!("handle_attacks: Damage applied! Health: {} -> {}", old_health, health.current);
                                    
                                    // Воспроизводим звук стрельбы
                                    let (tank_opt, aircraft_opt, infantry_opt) = unit_type_query.get(selected_entity.0.unwrap()).unwrap_or((None, None, None));
                                    let audio_source = if tank_opt.is_some() {
                                        asset_server.load("audio/tank_shot.mp3")
                                    } else if aircraft_opt.is_some() {
                                        asset_server.load("audio/aircraft_gun.mp3")  
                                    } else if infantry_opt.is_some() {
                                        asset_server.load("audio/infantry_shot.ogg")
                                    } else {
                                        asset_server.load("audio/gun.mp3")
                                    };

                                    info!("🔫 Playing shooting sound from unit at {:?}", shooter_transform.translation);
                                    commands.spawn(AudioBundle {
                                        source: audio_source,
                                        settings: PlaybackSettings::ONCE,
                                    });
                                    
                                    if health.current <= 0.0 {
                                        info!("handle_attacks: Enemy destroyed!");
                                        // Use try_despawn_recursive to avoid panics if entity is already despawned
                                        if let Some(entity_commands) = commands.get_entity(target_entity) {
                                            entity_commands.despawn_recursive();
                                        }
                                    }
                                } else {
                                    info!("handle_attacks: Could not get Health component from target");
                                }
                                
                                // Update last shot time
                                commands.entity(shooter_entity).insert(CanShoot {
                                    cooldown: can_shoot.cooldown,
                                    last_shot: current_time,
                                    range: can_shoot.range,
                                    damage: can_shoot.damage,
                                });
                            } else {
                                info!("handle_attacks: Target too far! Distance: {}, Range: {}", distance, can_shoot.range);
                            }
                        } else {
                            info!("handle_attacks: Could not get transforms for shooter or target");
                        }
                    } else {
                        info!("handle_attacks: Weapon on cooldown");
                    }
                } else {
                    info!("handle_attacks: Selected entity has no CanShoot component");
                }
            } else {
                info!("handle_attacks: No entity selected - cannot attack");
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