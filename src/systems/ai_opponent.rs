use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::menu::main_menu::Faction;
use crate::game::{Enemy, Health, CanShoot, ShapeType, MovementOrder, Tank, Selectable};

use crate::ui::money_ui::{AIMoney, AIWood, AIIron, AISteel, AIOil, PurchasableItem, can_afford_item_ai, deduct_resources_ai};
use crate::systems::turn_system::{TurnState, PlayerTurn};
use std::collections::HashSet;

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

/// Упрощенная система покупок ИИ - только базовый функционал
pub fn ai_purchase_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    turn_state: Res<TurnState>,
    mut ai_money: ResMut<AIMoney>,
    mut ai_wood: ResMut<AIWood>,
    mut ai_iron: ResMut<AIIron>,
    mut ai_steel: ResMut<AISteel>,
    mut ai_oil: ResMut<AIOil>,
    time: Res<Time>,
    ai_faction: Res<crate::game::units::AIFaction>,
    // Объединенный запрос для всех AI юнитов
    ai_units: Query<(
        Option<&crate::game::Tank>,
        Option<&crate::game::ShapeType>,
        Option<&crate::game::Aircraft>,
        Option<&crate::game::ForestFarm>,
        Option<&crate::game::Mine>,
        Option<&crate::game::SteelFactory>,
        Option<&crate::game::PetrochemicalPlant>,
    ), With<Enemy>>,
) {
    // ИИ покупает только в ход ИИ
    if turn_state.current_player != PlayerTurn::AI {
        return;
    }

    // Простой таймер - раз в 3 секунды
    static mut LAST_PURCHASE_TIME: f32 = 0.0;
    let current_time = time.elapsed_seconds();
    
    unsafe {
        if current_time - LAST_PURCHASE_TIME < 3.0 {
            return;
        }
        LAST_PURCHASE_TIME = current_time;
    }
    
    // Подсчет юнитов с лимитами используя объединенный Query
    let mut ai_tank_count = 0;
    let mut ai_infantry_count = 0;
    let mut ai_aircraft_count = 0;
    let mut ai_farm_count = 0;
    let mut ai_mine_count = 0;
    let mut ai_steel_factory_count = 0;
    let mut ai_petrochemical_plant_count = 0;
    
    for (tank, infantry, aircraft, farm, mine, steel_factory, petrochemical_plant) in ai_units.iter() {
        if tank.is_some() { ai_tank_count += 1; }
        if infantry.is_some() { ai_infantry_count += 1; }
        if aircraft.is_some() { ai_aircraft_count += 1; }
        if farm.is_some() { ai_farm_count += 1; }
        if mine.is_some() { ai_mine_count += 1; }
        if steel_factory.is_some() { ai_steel_factory_count += 1; }
        if petrochemical_plant.is_some() { ai_petrochemical_plant_count += 1; }
    }
    
    // Проверяем лимиты для каждого типа юнитов
    let tank_limit_reached = ai_tank_count >= 3;
    let infantry_limit_reached = ai_infantry_count >= 3;
    let aircraft_limit_reached = ai_aircraft_count >= 3;
    
    let farm_limit_reached = ai_farm_count >= 2; // Разрешаем ИИ строить до 2 ферм
    let mine_limit_reached = ai_mine_count >= 1;
    let steel_factory_limit_reached = ai_steel_factory_count >= 1;
    let petrochemical_plant_limit_reached = ai_petrochemical_plant_count >= 1;

    // Определяем приоритеты покупок с учетом лимитов
    let mut purchase_priorities = vec![];
    
    // ЗДАНИЯ ИМЕЮТ ВЫСШИЙ ПРИОРИТЕТ (только если не достигнут лимит и можем позволить)
    if !farm_limit_reached && can_afford_item_ai(PurchasableItem::Farm, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::Farm, 11.0)); // Фермы имеют высший приоритет для экономики
    }
    if !mine_limit_reached && can_afford_item_ai(PurchasableItem::Mine, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::Mine, 10.0));
    }
    if !steel_factory_limit_reached && can_afford_item_ai(PurchasableItem::SteelFactory, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::SteelFactory, 9.0));
    }
    if !petrochemical_plant_limit_reached && can_afford_item_ai(PurchasableItem::PetrochemicalPlant, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::PetrochemicalPlant, 8.0));
    }
    
    // Добавляем юниты только если не достигнут лимит и можем позволить
    if !infantry_limit_reached && can_afford_item_ai(PurchasableItem::Infantry, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::Infantry, 3.0));
    }
    if !tank_limit_reached && can_afford_item_ai(PurchasableItem::Tank, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::Tank, 2.0));
    }
    if !aircraft_limit_reached && can_afford_item_ai(PurchasableItem::Airplane, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
        purchase_priorities.push((PurchasableItem::Airplane, 1.0));
    }

    // Сортируем по приоритету
    purchase_priorities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Пытаемся купить самый приоритетный доступный предмет
    for (item, priority) in purchase_priorities.iter() {
        if priority < &0.3 {
            break; // Слишком низкий приоритет
        }

        if can_afford_item_ai(*item, &ai_money, &ai_wood, &ai_iron, &ai_steel, &ai_oil) {
            // Списываем все ресурсы
            deduct_resources_ai(*item, &mut ai_money, &mut ai_wood, &mut ai_iron, &mut ai_steel, &mut ai_oil);
            
            // Создаем юнит
            simple_spawn_ai_unit(*item, &mut commands, &asset_server, &time, &ai_faction);
            
            info!("AI purchased {:?} with priority {:.2}. Counts: Infantry {}/3, Tanks {}/3, Aircraft {}/3, Farms {}/2, Mines {}/1, Factories {}/1, Plants {}/1", 
                  item, priority, 
                  ai_infantry_count + if *item == PurchasableItem::Infantry { 1 } else { 0 },
                  ai_tank_count + if *item == PurchasableItem::Tank { 1 } else { 0 },
                  ai_aircraft_count + if *item == PurchasableItem::Airplane { 1 } else { 0 },
                  ai_farm_count + if *item == PurchasableItem::Farm { 1 } else { 0 },
                  ai_mine_count + if *item == PurchasableItem::Mine { 1 } else { 0 },
                  ai_steel_factory_count + if *item == PurchasableItem::SteelFactory { 1 } else { 0 },
                  ai_petrochemical_plant_count + if *item == PurchasableItem::PetrochemicalPlant { 1 } else { 0 });
            break; // Покупаем только один предмет за раз
        }
    }

}

/// Простая функция создания юнитов ИИ
fn simple_spawn_ai_unit(
    item: PurchasableItem,
    commands: &mut Commands,
    asset_server: &AssetServer,
    time: &Res<Time>,
    ai_faction: &Res<crate::game::units::AIFaction>,
) {
    // Определяем позицию для спавна (правая сторона карты для ИИ)
    let seed = time.elapsed_seconds_f64().fract() as f32;
    let x = 15.0 + (seed * 50.0).sin() * 5.0;
    let z = (seed * 75.0).cos() * 8.0;
    let spawn_pos = Vec3::new(x, 0.0, z);

    match item {
        PurchasableItem::Tank => {
            let faction = ai_faction.get_opposite();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let tank_type_index = rng.gen_range(0..3);
            
            let (model_path, scale) = match faction {
                Faction::Entente => {
                    match tank_type_index {
                        0 => ("models/entente/tanks/tsar_tank.glb#Scene0", 0.1), // tsar_tank уменьшен в 4 раза
                        1 => ("models/entente/tanks/mark1.glb#Scene0", 0.08), // mark1 уменьшен в 5 раз
                        _ => ("models/entente/tanks/renault_ft17.glb#Scene0", 0.4), // renault остается нормальным
                    }
                },
                Faction::CentralPowers => {
                    match tank_type_index {
                        0 => ("models/central_powers/tanks/panzerwagen.glb#Scene0", 0.08), // panzerwagen уменьшен в 5 раз
                        1 => ("models/central_powers/tanks/a7v.glb#Scene0", 0.08), // a7v уменьшен в 5 раз
                        _ => ("models/central_powers/tanks/steam_wheel_tank.glb#Scene0", 0.08), // steam_wheel остается как был
                    }
                },
            };
            let tank_entity = commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(scale)),
                    ..default()
                },
                ShapeType::Cube,
                Enemy,
                Tank,
                Selectable,
                Health { current: 100.0, max: 100.0 },
                CanShoot {
                    cooldown: 1.2,
                    last_shot: time.elapsed_seconds(),
                    range: 10.0,
                    damage: 12.0,
                },
                RigidBody::Dynamic,
                Collider::cuboid(5.0, 5.0, 5.0), // Очень большой коллайдер для AI танков
                Sensor, // Невидимый коллайдер для кликов
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
                Restitution::coefficient(0.0),
                Friction::coefficient(0.8),
                PickableBundle::default(),
                Name::new("AI Tank"),
            )).id();

            // Добавляем видимый клик-коллайдер для отладки - пока отложим
        }
        PurchasableItem::Infantry => {
            let faction = ai_faction.get_opposite();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let infantry_type_index = rng.gen_range(0..3);
            
            let model_path = match faction {
                Faction::Entente => {
                    match infantry_type_index {
                        0 => "models/infantry/russian_soldier.glb#Scene0",
                        1 => "models/infantry/british_soldier.glb#Scene0",
                        _ => "models/infantry/french_soldier.glb#Scene0",
                    }
                },
                Faction::CentralPowers => {
                    match infantry_type_index {
                        0 => "models/infantry/german_soldier.glb#Scene0",
                        1 => "models/infantry/turkish_soldier.glb#Scene0",
                        _ => "models/infantry/austrian_soldier.glb#Scene0",
                    }
                },
            };
            
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(1.0)),
                    ..default()
                },
                ShapeType::Infantry,
                Enemy,
                Selectable,
                Health { current: 60.0, max: 60.0 },
                CanShoot {
                    cooldown: 0.9,
                    last_shot: time.elapsed_seconds(),
                    range: 12.0,
                    damage: 8.0,
                },
                RigidBody::Dynamic,
                Collider::ball(3.0), // Очень большой коллайдер для AI пехоты
                Sensor, // Невидимый коллайдер для кликов
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y,
                Restitution::coefficient(0.0),
                Friction::coefficient(0.8),
                bevy_mod_picking::prelude::PickableBundle::default(),
                Name::new("AI Infantry"),
            ));
        }
        PurchasableItem::Airplane => {
            let faction = ai_faction.get_opposite();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let aircraft_type_index = rng.gen_range(0..3);
            
            let model_path = match faction {
                Faction::Entente => {
                    match aircraft_type_index {
                        0 => "models/entente/airplanes/sopwith_camel.glb#Scene0",
                        1 => "models/entente/airplanes/breguet_14.glb#Scene0",
                        _ => "models/entente/airplanes/ilya_muromets.glb#Scene0",
                    }
                },
                Faction::CentralPowers => {
                    match aircraft_type_index {
                        0 => "models/central_powers/airplanes/fokker.glb#Scene0",
                        1 => "models/central_powers/airplanes/albatros.glb#Scene0",
                        _ => "models/central_powers/airplanes/red_baron.glb#Scene0",
                    }
                },
            };
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 10.0, 0.0))
                        .with_scale(Vec3::splat(0.6)),
                    ..default()
                },
                ShapeType::Airplane,
                Enemy,
                crate::game::Aircraft { height: 10.0, speed: 5.0 },
                MovementOrder(Vec3::ZERO),
                Health { current: 75.0, max: 75.0 },
                CanShoot {
                    cooldown: 0.6,
                    last_shot: time.elapsed_seconds(),
                    range: 20.0,
                    damage: 15.0,
                },
                RigidBody::Fixed,
                Collider::cuboid(7.0, 4.0, 8.0), // Очень большой коллайдер для AI самолетов
                Sensor, // Невидимый коллайдер для кликов
                LockedAxes::all(),
                PickableBundle::default(),
                Name::new("AI Aircraft"),
            ));
        }
        PurchasableItem::Farm => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/forest.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.3)),
                    ..default()
                },
                crate::game::ForestFarm,
                crate::game::FarmIncomeRate(2.0), // Доход от фермы
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 80.0, max: 80.0 },
                RigidBody::Fixed,
                LockedAxes::all(),
                Collider::cuboid(1.5, 1.0, 1.5),
                PickableBundle::default(),
                Name::new("AI Farm"),
            ));
        }
        PurchasableItem::Mine => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/mine.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.3)),
                    ..default()
                },
                crate::game::Mine,
                crate::game::MineIronRate(2.0),
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 100.0, max: 100.0 },
                RigidBody::Fixed,
        LockedAxes::all(),
                Collider::cuboid(1.0, 0.5, 1.0),
                PickableBundle::default(),
                Name::new("AI Mine - FIXED"),
            ));
        }
        PurchasableItem::SteelFactory => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/factory.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.25)),
                    ..default()
                },
                crate::game::SteelFactory,
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 120.0, max: 120.0 },
                RigidBody::Fixed,
        LockedAxes::all(),
                Collider::cuboid(1.0, 0.5, 1.0),
                PickableBundle::default(),
                Name::new("AI Steel Factory - FIXED"),
            ));
        }
        PurchasableItem::PetrochemicalPlant => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/oil_pump.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.3)),
                    ..default()
                },
                crate::game::PetrochemicalPlant,
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 110.0, max: 110.0 },
                RigidBody::Fixed,
        LockedAxes::all(),
                Collider::cuboid(1.0, 0.5, 1.0),
                PickableBundle::default(),
                Name::new("AI Petrochemical Plant - FIXED"),
            ));
        }
    }
}

fn make_purchase_ai(
    item: PurchasableItem,
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    ai_money: &mut ResMut<AIMoney>,
    ai_wood: &mut ResMut<AIWood>,
    ai_iron: &mut ResMut<AIIron>,
    ai_steel: &mut ResMut<AISteel>,
    ai_oil: &mut ResMut<AIOil>,
    time: &Res<Time>,
    asset_server: &Res<AssetServer>,
    ai_faction: &Res<crate::game::units::AIFaction>,
) {
    // Списываем ресурсы ИИ
    deduct_resources_ai(item, ai_money, ai_wood, ai_iron, ai_steel, ai_oil);

    // Создаем объект
    spawn_ai_unit(item, commands, _meshes, _materials, time, asset_server, ai_faction);
}

fn spawn_ai_unit(
    item: PurchasableItem,
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    time: &Res<Time>,
    asset_server: &Res<AssetServer>,
    ai_faction: &Res<crate::game::units::AIFaction>,
) {
    // Определяем позицию для спавна (правая сторона карты для ИИ)
    let seed = time.elapsed_seconds_f64().fract() as f32;
    let x = 15.0 + (seed * 50.0).sin() * 5.0;
    let z = (seed * 75.0).cos() * 8.0;
    let spawn_pos = Vec3::new(x, 0.0, z);

    match item {
        PurchasableItem::Tank => {
            let faction = ai_faction.get_opposite();
            let model_path = match faction {
                Faction::Entente => "models/entente/tanks/mark1.glb#Scene0",
                Faction::CentralPowers => "models/central_powers/tanks/a7v.glb#Scene0",
            };
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 0.0, 0.0))
                        .with_scale(Vec3::splat(0.4)),
                    ..default()
                },
                ShapeType::Cube,
                Enemy,
                Tank,
                Selectable,
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
                PickableBundle::default(),
                Name::new("AI Tank"),
            ));
        }
        PurchasableItem::Infantry => {
            let faction = ai_faction.get_opposite();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let infantry_type_index = rng.gen_range(0..3);
            
            let model_path = match faction {
                Faction::Entente => {
                    match infantry_type_index {
                        0 => "models/infantry/russian_soldier.glb#Scene0",
                        1 => "models/infantry/british_soldier.glb#Scene0",
                        _ => "models/infantry/french_soldier.glb#Scene0",
                    }
                },
                Faction::CentralPowers => {
                    match infantry_type_index {
                        0 => "models/infantry/german_soldier.glb#Scene0",
                        1 => "models/infantry/turkish_soldier.glb#Scene0",
                        _ => "models/infantry/austrian_soldier.glb#Scene0",
                    }
                },
            };
            
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 0.0, 0.0))
                        .with_scale(Vec3::splat(0.8)),
                    ..default()
                },
                ShapeType::Infantry,
                Enemy,
                Selectable,
                Health { current: 60.0, max: 60.0 },
                CanShoot {
                    cooldown: 0.9,
                    last_shot: time.elapsed_seconds(),
                    range: 12.0,
                    damage: 8.0,
                },
                RigidBody::Dynamic,
                Collider::ball(3.0), // Очень большой коллайдер для AI пехоты
                Sensor, // Невидимый коллайдер для кликов
                LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y, // Заблокируем вращение и движение по Y
                Restitution::coefficient(0.0), // Без отскока
                Friction::coefficient(0.8), // Трение
                bevy_mod_picking::prelude::PickableBundle::default(),
                Name::new("AI Infantry"),
            ));
        }
        PurchasableItem::Airplane => {
            let faction = ai_faction.get_opposite();
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let aircraft_type_index = rng.gen_range(0..3);
            
            let model_path = match faction {
                Faction::Entente => {
                    match aircraft_type_index {
                        0 => "models/entente/airplanes/sopwith_camel.glb#Scene0",
                        1 => "models/entente/airplanes/breguet_14.glb#Scene0",
                        _ => "models/entente/airplanes/ilya_muromets.glb#Scene0",
                    }
                },
                Faction::CentralPowers => {
                    match aircraft_type_index {
                        0 => "models/central_powers/airplanes/fokker.glb#Scene0",
                        1 => "models/central_powers/airplanes/albatros.glb#Scene0",
                        _ => "models/central_powers/airplanes/red_baron.glb#Scene0",
                    }
                },
            };
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load(model_path),
                    transform: Transform::from_translation(spawn_pos + Vec3::new(0.0, 10.0, 0.0))
                        .with_scale(Vec3::splat(0.6)),
                    ..default()
                },
                ShapeType::Airplane,
                Enemy,
                crate::game::Aircraft { height: 10.0, speed: 5.0 },
                MovementOrder(Vec3::ZERO),
                Health { current: 75.0, max: 75.0 },
                CanShoot {
                    cooldown: 0.6,
                    last_shot: time.elapsed_seconds(),
                    range: 20.0,
                    damage: 15.0,
                },
                RigidBody::Fixed, // Самолеты теперь фиксированы в воздухе
                Collider::cuboid(5.0, 2.5, 6.0), // Большой коллайдер для AI самолетов
                LockedAxes::all(), // Блокируем все движения
                PickableBundle::default(),
                Name::new("AI Aircraft"),
            ));
        }
        PurchasableItem::Farm => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/forest.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.3)),
                    ..default()
                },
                crate::game::ForestFarm,
                crate::game::FarmIncomeRate(2.0), // Доход от фермы
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 80.0, max: 80.0 },
                RigidBody::Fixed,
                LockedAxes::all(),
                Collider::cuboid(1.5, 1.0, 1.5),
                PickableBundle::default(),
                Name::new("AI Farm"),
            ));
        }
        PurchasableItem::Mine => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/mine.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.3)),
                    ..default()
                },
                crate::game::Mine,
                crate::game::MineIronRate(2.0),
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 100.0, max: 100.0 },
                RigidBody::Fixed,
        LockedAxes::all(),
                Collider::cuboid(1.0, 0.5, 1.0),
                PickableBundle::default(),
                Name::new("AI Mine - FIXED"),
            ));
        }
        PurchasableItem::SteelFactory => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/factory.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.25)),
                    ..default()
                },
                crate::game::SteelFactory,
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 120.0, max: 120.0 },
                RigidBody::Fixed,
        LockedAxes::all(),
                Collider::cuboid(1.0, 0.5, 1.0),
                PickableBundle::default(),
                Name::new("AI Steel Factory - FIXED"),
            ));
        }
        PurchasableItem::PetrochemicalPlant => {
            commands.spawn((
                SceneBundle {
                    scene: asset_server.load("models/farm/oil_pump.glb#Scene0"),
                    transform: Transform::from_translation(spawn_pos)
                        .with_scale(Vec3::splat(0.3)),
                    ..default()
                },
                crate::game::PetrochemicalPlant,
                crate::game::FarmActive(true),
                crate::game::Enemy,
                Health { current: 110.0, max: 110.0 },
                RigidBody::Fixed,
        LockedAxes::all(),
                Collider::cuboid(1.0, 0.5, 1.0),
                PickableBundle::default(),
                Name::new("AI Petrochemical Plant - FIXED"),
            ));
        }
    }
}

/// Система активных действий ИИ - движение и атака во время хода ИИ
pub fn ai_movement_system(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    time: Res<Time>,
    // ИИ юниты
    mut ai_units: Query<(Entity, &mut Transform, Option<&MovementOrder>), (With<Enemy>, Without<crate::game::Tank>, Without<crate::game::ForestFarm>, Without<crate::game::Mine>, Without<crate::game::SteelFactory>, Without<crate::game::PetrochemicalPlant>)>,
    mut ai_tanks: Query<(Entity, &mut Transform, Option<&MovementOrder>), (With<Enemy>, With<Tank>)>,
    // Цели для атаки (все юниты игрока кроме зданий)
    player_units: Query<&Transform, (With<Health>, Without<Enemy>)>, // Ищем ВСЕ цели игрока включая здания
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

/// Система атак ИИ - с разносом по времени
pub fn ai_combat_system(
    turn_state: Res<TurnState>,
    time: Res<Time>,
    mut ai_units: Query<(Entity, &Transform, &mut CanShoot), (With<Enemy>, Without<crate::game::ForestFarm>, Without<crate::game::Mine>, Without<crate::game::SteelFactory>, Without<crate::game::PetrochemicalPlant>)>,
    mut player_units: Query<(Entity, &Transform, &mut Health), Without<Enemy>>, // Атакуем ВСЕ цели игрока включая здания
    mut commands: Commands,
) {
    // ИИ атакует только в свой ход
    if turn_state.current_player != PlayerTurn::AI {
        return;
    }
    
    let current_time = time.elapsed_seconds();
    
    // Отслеживаем, какие цели уже атакуются в этом кадре
    let mut targets_being_attacked: HashSet<Entity> = HashSet::new();
    
    // Собираем всех AI юнитов, готовых к атаке, и сортируем по расстоянию до ближайшей цели
    let mut ready_ai_units: Vec<(Entity, Vec3, f32, f32)> = Vec::new(); // Entity, position, range, closest_distance
    
    for (ai_entity, ai_transform, mut can_shoot) in ai_units.iter_mut() {
        // Создаем уникальную задержку для каждого юнита на основе их ID
        let unit_specific_delay = (ai_entity.index() as f32 * 0.3) % 1.5;
        let adjusted_cooldown = can_shoot.cooldown + unit_specific_delay;
        
        // Проверяем кулдаун с индивидуальной задержкой
        if current_time - can_shoot.last_shot < adjusted_cooldown {
            continue;
        }
        
        // Найдем ближайшую цель для этого юнита
        let mut closest_distance = f32::INFINITY;
        for (_, target_transform, _) in player_units.iter() {
            let distance = ai_transform.translation.distance(target_transform.translation);
            if distance <= can_shoot.range && distance < closest_distance {
                closest_distance = distance;
            }
        }
        
        if closest_distance < f32::INFINITY {
            ready_ai_units.push((ai_entity, ai_transform.translation, can_shoot.range, closest_distance));
        }
    }
    
    // Сортируем AI юнитов по расстоянию до ближайшей цели (ближайшие атакуют первыми)
    ready_ai_units.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
    
    // Теперь обрабатываем атаки, избегая множественных атак на одну цель
    for (ready_ai_entity, ai_pos, ai_range, _) in ready_ai_units {
        // Получаем мутабельную ссылку на CanShoot для этого юнита
        if let Ok((_, _, mut can_shoot)) = ai_units.get_mut(ready_ai_entity) {
        let mut target_found = false;
        
        // Ищем цели в радиусе, которые еще не атакуются
        for (target_entity, target_transform, mut target_health) in player_units.iter_mut() {
            // Пропускаем цели, которые уже атакуются
            if targets_being_attacked.contains(&target_entity) {
                continue;
            }
            
            let distance = ai_pos.distance(target_transform.translation);
            
            if distance <= ai_range {
                // Отмечаем эту цель как атакуемую
                targets_being_attacked.insert(target_entity);
                
                // Атакуем
                target_health.current -= can_shoot.damage;
                can_shoot.last_shot = current_time;
                
                info!("AI unit attacked player unit for {} damage!", can_shoot.damage);
                
                // Если цель уничтожена
                if target_health.current <= 0.0 {
                    if let Some(entity_commands) = commands.get_entity(target_entity) {
                        entity_commands.despawn_recursive();
                    }
                    info!("Player unit destroyed by AI!");
                }
                
                target_found = true;
                break; // Атакуем только одну цель за раз
            }
        }
        
        // Если юнит не нашел цель, он не атакует в этом кадре
        if !target_found {
            // Можно добавить логику поиска цели или движения
        }
        } // Закрываем if let Ok
    }
}

fn find_nearest_target(pos: &Vec3, targets: &Query<&Transform, (With<Health>, Without<Enemy>)>) -> Option<Vec3> {
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

fn spawn_ai_farm(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    existing_farms: &Query<&Transform, (With<crate::game::ForestFarm>, With<Enemy>)>,
    time: &Res<Time>,
) {
    // Генерируем позицию с достаточным расстоянием от существующих ферм
    let mut attempts = 0;
    let farm_position = loop {
        let seed = time.elapsed_seconds_f64().fract() as f32 + attempts as f32 * 0.1;
        let x = 15.0 + (seed * 30.0).sin() * 8.0; // От 7 до 23
        let z = (seed * 45.0).cos() * 10.0; // От -10 до 10
        let candidate_pos = Vec3::new(x, 0.0, z);
        
        // Проверяем минимальное расстояние до существующих ферм (6 единиц)
        let mut too_close = false;
        for existing_transform in existing_farms.iter() {
            if existing_transform.translation.distance(candidate_pos) < 6.0 {
                too_close = true;
                break;
            }
        }
        
        if !too_close || attempts > 10 {
            break candidate_pos;
        }
        attempts += 1;
    };
    
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/farm/forest.glb#Scene0"),
            transform: Transform::from_translation(farm_position)
                .with_scale(Vec3::splat(0.2)),
            ..default()
        },
        crate::game::ForestFarm,
        crate::game::FarmActive(true), 
        crate::game::Enemy,
        RigidBody::Fixed,
        LockedAxes::all(),
        Collider::cuboid(1.0, 0.5, 1.0),
        PickableBundle::default(),
        Name::new("AI Farm - WITH HP"),
    ));
    
    info!("AI farm spawned as ACTIVE at position: {:?}", farm_position);
}
