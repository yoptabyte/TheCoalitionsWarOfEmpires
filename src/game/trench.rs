use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::{
    Health, Selectable, ShapeType, TrenchConstruction,
    Trench, HoveredOutline, TrenchCost
};
use crate::ui::money_ui::{Money, Wood};

// Функция создания окопа под строительство
pub fn spawn_constructing_trench(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let construction_time = 6.0; // 6 секунд на строительство
    
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 1.5))),
            material: materials.add(Color::rgb(0.6, 0.4, 0.2)), // цвет земли/грязи для окопа
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.25, 0.0)),
            ..default()
        },
        Trench,
        Selectable,
        PickableBundle::default(),
        ShapeType::Trench,
        Health { current: 50.0, max: 50.0 },
        TrenchConstruction {
            time_remaining: construction_time,
            total_construction_time: construction_time,
        },
        Name::new("Trench"),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

// Функция для создания уже построенного окопа
pub fn spawn_completed_trench(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 1.5))),
            material: materials.add(Color::rgb(0.5, 0.35, 0.15)), // цвет готового окопа
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.25, 0.0)),
            ..default()
        },
        Trench,
        Selectable,
        PickableBundle::default(),
        ShapeType::Trench,
        Health { current: 100.0, max: 100.0 }, 
        Name::new("Trench"),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

// Система для обработки строительства окопов
pub fn update_trench_construction(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut TrenchConstruction, &Transform), With<Trench>>,
    time: Res<Time>,
) {
    for (entity, mut construction, transform) in query.iter_mut() {
        construction.time_remaining -= time.delta_seconds();
        
        if construction.time_remaining <= 0.0 {
            info!("Trench construction completed!");
            
            // Удаляем строящийся окоп
            commands.entity(entity).despawn_recursive();
            
            // Создаем готовый окоп на том же месте
            spawn_completed_trench(
                &mut commands, 
                &mut meshes, 
                &mut materials, 
                transform.translation - Vec3::new(0.0, 0.25, 0.0) // Отнимаем смещение, которое было добавлено при создании
            );
        }
    }
}

// Система для отображения прогресса строительства окопа
pub fn draw_trench_construction_progress(
    mut gizmos: Gizmos,
    query: Query<(&TrenchConstruction, &Transform), With<Trench>>,
) {
    for (construction, transform) in query.iter() {
        let progress = 1.0 - (construction.time_remaining / construction.total_construction_time);
        let width = 2.0 * progress; // Визуальная длина прогресса строительства
        
        let start = transform.translation + Vec3::new(-1.0, 1.0, 0.0);
        let end = start + Vec3::new(width, 0.0, 0.0);
        
        // Рисуем линию прогресса над окопом
        gizmos.line(start, end, Color::GREEN);
    }
}

// Система для постройки окопа по нажатию клавиши
pub fn spawn_trench_on_keystroke(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    trench_cost: Option<Res<TrenchCost>>,
    time: Res<Time>,
) {
    let cost = match trench_cost {
        Some(ref cost) => cost,
        None => {
            // Если ресурс не инициализирован, используем значения по умолчанию
            let default_cost = TrenchCost::default();
            commands.insert_resource(default_cost);
            return; // Возвращаемся, чтобы в следующем кадре использовать созданный ресурс
        }
    };
    
    // Построить окоп при нажатии на клавишу 'B' (от слова Build)
    if keyboard.just_pressed(KeyCode::KeyB) {
        // Проверка наличия ресурсов
        if money.0 >= cost.money as f32 && wood.0 >= cost.wood as f32 {
            // Списываем ресурсы
            money.0 -= cost.money as f32;
            wood.0 -= cost.wood as f32;
            
            // Определяем положение для окопа, используя время вместо rand
            let seed = time.elapsed_seconds_f64().fract() as f32;
            let x = (seed * 100.0).sin() * 10.0 - 5.0;
            let z = (seed * 100.0).cos() * 10.0 - 5.0;
            let trench_position = Vec3::new(x, 0.0, z);
            
            info!("Spawning new trench at position: {:?}, cost: {} wood, {} money", trench_position, cost.wood, cost.money);
            
            spawn_constructing_trench(
                &mut commands,
                &mut meshes,
                &mut materials,
                trench_position,
            );
        } else {
            info!("Not enough resources to build a trench! Need {} wood and {} money", cost.wood, cost.money);
        }
    }
} 