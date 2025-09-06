use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::game::{Enemy, Health, CanShoot, ShapeType, MovementOrder, Tank};
use crate::ui::money_ui::{Money, Wood, Iron, Steel, Oil, PurchasableItem};
use crate::systems::turn_system::{TurnState, PlayerTurn};

#[derive(Resource, Debug)]
pub struct AIBehavior {
    pub difficulty: AIDifficulty,
    pub strategy: AIStrategy,
    pub weights: AIWeights,
    pub last_decision_time: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum AIDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Copy)]
pub enum AIStrategy {
    Rusher,     // Быстрые атаки пехотой и танками
    Defender,   // Строительство укреплений и башен
    Economic,   // Фокус на заводы и ресурсы
    Balanced,   // Смешанная стратегия
}

#[derive(Debug, Clone)]
pub struct AIWeights {
    pub aggression: f32,    // 0.0-1.0 - насколько агрессивно атакует
    pub economy: f32,       // 0.0-1.0 - насколько фокусируется на экономике
    pub defense: f32,       // 0.0-1.0 - насколько строит оборону
}

impl Default for AIBehavior {
    fn default() -> Self {
        Self {
            difficulty: AIDifficulty::Medium,
            strategy: AIStrategy::Balanced,
            weights: AIWeights {
                aggression: 0.5,
                economy: 0.4,
                defense: 0.3,
            },
            last_decision_time: 0.0,
        }
    }
}

impl AIStrategy {
    pub fn get_weights(&self) -> AIWeights {
        match self {
            AIStrategy::Rusher => AIWeights {
                aggression: 0.8,
                economy: 0.2,
                defense: 0.1,
            },
            AIStrategy::Defender => AIWeights {
                aggression: 0.2,
                economy: 0.3,
                defense: 0.7,
            },
            AIStrategy::Economic => AIWeights {
                aggression: 0.1,
                economy: 0.8,
                defense: 0.3,
            },
            AIStrategy::Balanced => AIWeights {
                aggression: 0.5,
                economy: 0.4,
                defense: 0.4,
            },
        }
    }
}

/// Система покупок ИИ - работает только в ход ИИ
pub fn ai_purchase_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    turn_state: Res<TurnState>,
    mut ai_behavior: ResMut<AIBehavior>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    mut steel: ResMut<Steel>,
    mut oil: ResMut<Oil>,
    time: Res<Time>,
    // Запросы для анализа текущего состояния
    player_units: Query<&Transform, (With<crate::game::Tank>, Without<Enemy>)>,
    ai_units: Query<&Transform, (With<crate::game::Tank>, With<Enemy>)>,
    _ai_buildings: Query<&ShapeType, With<Enemy>>,
) {
    // ИИ покупает только в свой ход
    if turn_state.current_player != PlayerTurn::AI {
        return;
    }

    // ИИ принимает решения не каждый кадр, а раз в секунду
    if time.elapsed_seconds() - ai_behavior.last_decision_time < 1.0 {
        return;
    }
    
    ai_behavior.last_decision_time = time.elapsed_seconds();

    let weights = ai_behavior.strategy.get_weights();
    
    // Анализ ситуации
    let player_unit_count = player_units.iter().count();
    let ai_unit_count = ai_units.iter().count();
    let unit_ratio = if ai_unit_count > 0 { 
        player_unit_count as f32 / ai_unit_count as f32 
    } else { 
        10.0 // Если у ИИ нет юнитов, срочно нужно покупать
    };

    // Определяем приоритеты покупок
    let mut purchase_priorities = vec![
        (PurchasableItem::Infantry, weights.aggression * unit_ratio),
        (PurchasableItem::Tank, weights.aggression * 0.8),
        (PurchasableItem::Airplane, weights.aggression * 0.6),
        (PurchasableItem::Mine, weights.economy * 2.0),
        (PurchasableItem::SteelFactory, weights.economy * 1.5),
        (PurchasableItem::PetrochemicalPlant, weights.economy * 1.2),
        (PurchasableItem::Trench, weights.defense * 1.0),
    ];

    // Сортируем по приоритету
    purchase_priorities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Пытаемся купить самый приоритетный доступный предмет
    for (item, priority) in purchase_priorities.iter() {
        if priority < &0.3 {
            break; // Слишком низкий приоритет
        }

        if can_afford(*item, &money, &wood, &iron, &steel, &oil) {
            make_purchase(
                *item,
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut money,
                &mut wood,
                &mut iron,
                &mut steel,
                &mut oil,
                &time,
            );
            
            info!("AI purchased {:?} with priority {:.2}", item, priority);
            break; // Покупаем только один предмет за раз
        }
    }
}

fn can_afford(
    item: PurchasableItem,
    money: &Money,
    wood: &Wood,
    iron: &Iron,
    steel: &Steel,
    oil: &Oil,
) -> bool {
    money.0 >= item.cost() 
        && wood.0 >= item.wood_cost()
        && iron.0 >= item.iron_cost()
        && steel.0 >= item.steel_cost()
        && oil.0 >= item.oil_cost()
}

fn make_purchase(
    item: PurchasableItem,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    money: &mut ResMut<Money>,
    wood: &mut ResMut<Wood>,
    iron: &mut ResMut<Iron>,
    steel: &mut ResMut<Steel>,
    oil: &mut ResMut<Oil>,
    time: &Res<Time>,
) {
    // Списываем ресурсы
    money.0 -= item.cost();
    wood.0 -= item.wood_cost();
    iron.0 -= item.iron_cost();
    steel.0 -= item.steel_cost();
    oil.0 -= item.oil_cost();

    // Создаем объект
    spawn_ai_unit(item, commands, meshes, materials, time);
}

fn spawn_ai_unit(
    item: PurchasableItem,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    time: &Res<Time>,
) {
    // Определяем позицию для спавна (правая сторона карты для ИИ)
    let seed = time.elapsed_seconds_f64().fract() as f32;
    let x = 15.0 + (seed * 50.0).sin() * 5.0;
    let z = (seed * 75.0).cos() * 8.0;
    let spawn_pos = Vec3::new(x, 0.0, z);

    match item {
        PurchasableItem::Tank => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.8, 0.2, 0.2), // Красный для врагов
                        ..default()
                    }),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 0.5, 0.0)),
                    ..default()
                },
                ShapeType::Cube,
                Enemy,
                Tank,
                Health { current: 100.0, max: 100.0 },
                CanShoot {
                    cooldown: 1.2,
                    last_shot: time.elapsed_seconds(),
                    range: 10.0,
                    damage: 12.0,
                },
                RigidBody::Dynamic,
                Collider::cuboid(0.5, 0.5, 0.5), // Коллайдер танка ИИ
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y, // Заблокируем вращение и движение по Y
                Restitution::coefficient(0.0), // Без отскока
                Friction::coefficient(0.8), // Трение
                Name::new("AI Tank"),
            ));
        }
        PurchasableItem::Infantry => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Sphere::new(0.5))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.7, 0.1, 0.1),
                        ..default()
                    }),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 0.5, 0.0)),
                    ..default()
                },
                ShapeType::Infantry,
                Enemy,
                Health { current: 60.0, max: 60.0 },
                CanShoot {
                    cooldown: 0.9,
                    last_shot: time.elapsed_seconds(),
                    range: 12.0,
                    damage: 8.0,
                },
                RigidBody::Dynamic,
                Collider::ball(0.5), // Коллайдер пехоты ИИ
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y, // Заблокируем вращение и движение по Y
                Restitution::coefficient(0.0), // Без отскока
                Friction::coefficient(0.8), // Трение
                Name::new("AI Infantry"),
            ));
        }
        PurchasableItem::Airplane => {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 4.0))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.6, 0.1, 0.1),
                        ..default()
                    }),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 10.0, 0.0)),
                    ..default()
                },
                ShapeType::Airplane,
                Enemy,
                crate::game::Aircraft { height: 10.0, speed: 5.0 },
                Health { current: 75.0, max: 75.0 },
                CanShoot {
                    cooldown: 0.6,
                    last_shot: time.elapsed_seconds(),
                    range: 20.0,
                    damage: 15.0,
                },
                RigidBody::Dynamic,
                Collider::cuboid(1.0, 0.25, 2.0), // Коллайдер самолета ИИ
                LockedAxes::ROTATION_LOCKED, // Заблокируем только вращение, самолеты могут двигаться по Y
                Restitution::coefficient(0.0), // Без отскока
                Friction::coefficient(0.0), // Без трения в воздухе
                Name::new("AI Aircraft"),
            ));
        }
        // Для зданий пока просто логируем
        _ => {
            info!("AI would build {:?} at {:?}", item, spawn_pos);
        }
    }
}

/// Система активных действий ИИ - движение и атака во время хода ИИ
pub fn ai_movement_system(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    time: Res<Time>,
    // ИИ юниты
    mut ai_units: Query<(Entity, &mut Transform, Option<&MovementOrder>), (With<Enemy>, Without<crate::game::Tank>)>,
    mut ai_tanks: Query<(Entity, &mut Transform, Option<&MovementOrder>), (With<Enemy>, With<Tank>)>,
    // Цели для атаки (все юниты игрока кроме зданий)
    player_units: Query<&Transform, (With<Health>, Without<Enemy>, Without<crate::game::components::Farm>, Without<crate::game::components::Mine>, Without<crate::game::components::SteelFactory>, Without<crate::game::components::PetrochemicalPlant>)>,
) {
    // ИИ действует только в свой ход
    if turn_state.current_player != PlayerTurn::AI {
        return;
    }
    
    let delta_time = time.delta_seconds();
    
    // Обработка движения ИИ танков
    for (entity, mut transform, movement_order) in ai_tanks.iter_mut() {
        if let Some(order) = movement_order {
            let distance_to_target = Vec3::new(transform.translation.x, 0.0, transform.translation.z)
                .distance(Vec3::new(order.0.x, 0.0, order.0.z));
            let attack_range = 10.0; // Дистанция атаки танка
            
            // Если еще далеко от цели - двигаемся
            if distance_to_target > attack_range {
                let mut direction = order.0 - transform.translation;
                direction.y = 0.0; // Игнорируем Y координату - движемся только по земле
                let direction = direction.normalize();
                let move_speed = 3.0; // Скорость танка
                let movement = direction * move_speed * delta_time;
                transform.translation.x += movement.x;
                transform.translation.z += movement.z;
                // Y координата остается неизменной (на земле)
            }
            // Если цель в радиусе атаки - останавливаемся и убираем приказ на движение
            else {
                commands.entity(entity).remove::<MovementOrder>();
            }
        } else {
            // Если нет приказа, найти ближайшую цель для атаки
            if let Some(target_pos) = find_nearest_target(&transform.translation, &player_units) {
                commands.entity(entity).insert(MovementOrder(target_pos));
            }
        }
    }
    
    // Обработка движения ИИ пехоты
    for (entity, mut transform, movement_order) in ai_units.iter_mut() {
        if let Some(order) = movement_order {
            let distance_to_target = Vec3::new(transform.translation.x, 0.0, transform.translation.z)
                .distance(Vec3::new(order.0.x, 0.0, order.0.z));
            let attack_range = 12.0; // Дистанция атаки пехоты
            
            // Если еще далеко от цели - двигаемся
            if distance_to_target > attack_range {
                let mut direction = order.0 - transform.translation;
                direction.y = 0.0; // Игнорируем Y координату - движемся только по земле
                let direction = direction.normalize();
                let move_speed = 2.0; // Скорость пехоты
                let movement = direction * move_speed * delta_time;
                transform.translation.x += movement.x;
                transform.translation.z += movement.z;
                // Y координата остается неизменной (на земле)
            }
            // Если цель в радиусе атаки - останавливаемся и убираем приказ на движение
            else {
                commands.entity(entity).remove::<MovementOrder>();
            }
        } else {
            // Если нет приказа, найти ближайшую цель для атаки
            if let Some(target_pos) = find_nearest_target(&transform.translation, &player_units) {
                commands.entity(entity).insert(MovementOrder(target_pos));
            }
        }
    }
}

/// Система атак ИИ
pub fn ai_combat_system(
    turn_state: Res<TurnState>,
    time: Res<Time>,
    mut ai_units: Query<(&Transform, &mut CanShoot), With<Enemy>>,
    mut player_units: Query<(Entity, &Transform, &mut Health), (Without<Enemy>, Without<crate::game::components::Farm>, Without<crate::game::components::Mine>, Without<crate::game::components::SteelFactory>, Without<crate::game::components::PetrochemicalPlant>)>,
    mut commands: Commands,
) {
    // ИИ атакует только в свой ход
    if turn_state.current_player != PlayerTurn::AI {
        return;
    }
    
    let current_time = time.elapsed_seconds();
    
    for (ai_transform, mut can_shoot) in ai_units.iter_mut() {
        // Проверяем кулдаун
        if current_time - can_shoot.last_shot < can_shoot.cooldown {
            continue;
        }
        
        // Ищем цели в радиусе
        for (target_entity, target_transform, mut target_health) in player_units.iter_mut() {
            let distance = ai_transform.translation.distance(target_transform.translation);
            
            if distance <= can_shoot.range {
                // Атакуем
                target_health.current -= can_shoot.damage;
                can_shoot.last_shot = current_time;
                
                info!("AI unit attacked player unit for {} damage!", can_shoot.damage);
                
                // Если цель уничтожена
                if target_health.current <= 0.0 {
                    commands.entity(target_entity).despawn();
                    info!("Player unit destroyed by AI!");
                }
                
                break; // Атакуем только одну цель за раз
            }
        }
    }
}

fn find_nearest_target(pos: &Vec3, targets: &Query<&Transform, (With<Health>, Without<Enemy>, Without<crate::game::components::Farm>, Without<crate::game::components::Mine>, Without<crate::game::components::SteelFactory>, Without<crate::game::components::PetrochemicalPlant>)>) -> Option<Vec3> {
    let mut nearest_pos = None;
    let mut nearest_distance = f32::INFINITY;
    
    for target_transform in targets.iter() {
        let distance = pos.distance(target_transform.translation);
        if distance < nearest_distance {
            nearest_distance = distance;
            nearest_pos = Some(target_transform.translation);
        }
    }
    
    nearest_pos
}