use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::ui::money_ui::{AIMoney, AIWood, AIIron, AISteel, AIOil};
use crate::game::{ForestFarm, FarmActive, Mine, MineIronRate, SteelFactory, PetrochemicalPlant, Enemy};

/// Система генерации ресурсов для ИИ
pub fn ai_resource_generation_system(
    time: Res<Time>,
    mut ai_money: ResMut<AIMoney>,
    mut ai_wood: ResMut<AIWood>,
    mut ai_iron: ResMut<AIIron>,
    mut ai_steel: ResMut<AISteel>,
    mut ai_oil: ResMut<AIOil>,
    // Запросы для зданий ИИ
    ai_farms: Query<(&ForestFarm, &FarmActive), With<Enemy>>,
    ai_mines: Query<(&Mine, &MineIronRate, &FarmActive), With<Enemy>>,
    ai_steel_factories: Query<(&SteelFactory, &FarmActive), With<Enemy>>,
    ai_petrochemical_plants: Query<(&PetrochemicalPlant, &FarmActive), With<Enemy>>,
) {
    let delta_time = time.delta_seconds();
    
    // Ускоренный базовый доход ИИ для баланса
    ai_money.0 += 0.25 * delta_time; // Увеличено в 2.5 раза
    
    // Доход от ферм ИИ - ИСПРАВЛЕНО: деньги и дерево поменяны местами
    for (_farm, farm_active) in ai_farms.iter() {
        if farm_active.0 {
            ai_money.0 += 4.5 * delta_time; // ДЕНЬГИ с ферм (было дерево)
            ai_wood.0 += 0.8 * delta_time; // ДЕРЕВО с ферм (было деньги)
        }
    }
    
    // Доход от шахт ИИ - увеличенный
    for (_mine, iron_rate, farm_active) in ai_mines.iter() {
        if farm_active.0 {
            ai_iron.0 += iron_rate.0 * delta_time * 1.5; // Увеличено на 50%
            ai_money.0 += 0.5 * delta_time; // Увеличено с 0.3
        }
    }
    
    // Доход от сталелитейных заводов ИИ - ускоренный
    for (_steel_factory, farm_active) in ai_steel_factories.iter() {
        if farm_active.0 {
            // Конвертируем железо в сталь быстрее
            if ai_iron.0 >= 0.8 { // Меньше требуется железа
                ai_iron.0 -= 0.8 * delta_time;
                ai_steel.0 += 0.7 * delta_time; // Больше стали производится
            }
        }
    }
    
    // Доход от нефтехимических заводов ИИ - ускоренный
    for (_petrochemical_plant, farm_active) in ai_petrochemical_plants.iter() {
        if farm_active.0 {
            ai_oil.0 += 1.5 * delta_time; // Увеличено с 1.0 до 1.5
            ai_money.0 += 1.2 * delta_time; // Увеличено с 0.8 до 1.2
        }
    }
}

/// Система начальных ресурсов для ИИ и создания стартовой фермы
pub fn ai_initial_resources_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut ai_money: ResMut<AIMoney>,
    mut ai_wood: ResMut<AIWood>,
    mut ai_iron: ResMut<AIIron>,
    mut ai_steel: ResMut<AISteel>,
    mut ai_oil: ResMut<AIOil>,
) {
    // Даем ИИ улучшенные начальные ресурсы для баланса
    if ai_money.0 == 0.0 {
        ai_money.0 = 60.0; // Увеличено с 45 до 60
        ai_wood.0 = 8.0; // Увеличено с 5 до 8
        ai_iron.0 = 5.0; // Увеличено с 3 до 5
        ai_steel.0 = 1.0; // Дадим немного стали с самого начала
        ai_oil.0 = 1.0; // Дадим немного нефти с самого начала
        
        // СРАЗУ СОЗДАЕМ АКТИВНУЮ ФЕРМУ ДЛЯ ИИ!
        spawn_initial_ai_farm(&mut commands, &mut meshes, &mut materials, &asset_server);
        
        info!("AI initialized with improved starting resources and ACTIVE FARM!");
    }
}

/// Создает начальную ферму ИИ при старте игры
fn spawn_initial_ai_farm(
    commands: &mut Commands,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    // Позиция для стартовой фермы ИИ (правая сторона карты, варьируется)
    let farm_position = Vec3::new(18.0, 0.0, -8.0);
    
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
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::LockedAxes::all(),
        bevy_rapier3d::prelude::Collider::cuboid(1.0, 0.5, 1.0),
        PickableBundle::default(),
        Name::new("AI Forest - NO HP - FIXED"),
    ));
    
    info!("AI STARTING FARM CREATED AS ACTIVE at position: {:?}", farm_position);
}
