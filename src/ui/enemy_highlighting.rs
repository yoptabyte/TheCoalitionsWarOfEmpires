use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Система для визуального выделения врагов красным цветом
pub fn highlight_enemy_entities(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Запрос для всех врагов с детьми, которые еще не обработаны
    enemy_query: Query<(Entity, &Children), (With<crate::game::Enemy>, Added<Children>)>,
    enemy_tower_query: Query<(Entity, &Children), (With<crate::game::EnemyTower>, Added<Children>)>,
    // Запрос для mesh-ей с материалами
    mesh_query: Query<(Entity, &Handle<StandardMaterial>), With<Handle<Mesh>>>,
    // Маркер для уже обработанных entities
    highlighted_query: Query<&EnemyHighlighted>,
) {
    // Обрабатываем обычных врагов
    for (enemy_entity, children) in enemy_query.iter() {
        info!("Highlighting Enemy entity {} with {} children", enemy_entity.index(), children.len());
        
        // Рекурсивно обрабатываем всех потомков
        highlight_entity_children(
            enemy_entity, 
            children, 
            &mut commands, 
            &mut materials, 
            &mesh_query, 
            &highlighted_query,
            Color::rgb(0.8, 0.2, 0.2) // Красный цвет для врагов
        );
    }
    
    // Обрабатываем башни врагов
    for (tower_entity, children) in enemy_tower_query.iter() {
        info!("Highlighting EnemyTower entity {} with {} children", tower_entity.index(), children.len());
        
        // Рекурсивно обрабатываем всех потомков
        highlight_entity_children(
            tower_entity, 
            children, 
            &mut commands, 
            &mut materials, 
            &mesh_query, 
            &highlighted_query,
            Color::rgb(0.9, 0.1, 0.1) // Более яркий красный для башен
        );
    }
}

/// Рекурсивная функция для выделения всех mesh-потомков entity
fn highlight_entity_children(
    parent_entity: Entity,
    children: &Children,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mesh_query: &Query<(Entity, &Handle<StandardMaterial>), With<Handle<Mesh>>>,
    highlighted_query: &Query<&EnemyHighlighted>,
    color: Color,
) {
    for &child in children.iter() {
        // Проверяем, не был ли уже обработан этот child
        if highlighted_query.get(child).is_ok() {
            continue;
        }
        
        // Если это mesh с материалом, изменяем материал
        if let Ok((mesh_entity, material_handle)) = mesh_query.get(child) {
            info!("Changing material for mesh child {} of parent {}", mesh_entity.index(), parent_entity.index());
            
            // Создаем новый материал с красным цветом
            let new_material = StandardMaterial {
                base_color: color,
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            };
            
            let new_material_handle = materials.add(new_material);
            
            // Заменяем материал и добавляем маркер
            commands.entity(mesh_entity).insert((
                new_material_handle,
                EnemyHighlighted,
            ));
        }
    }
}

/// Система для выделения игровых юнитов зеленым цветом для лучшего контраста
pub fn highlight_player_entities(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Запрос для игровых юнитов
    tank_query: Query<(Entity, &Children), (With<crate::game::Tank>, Without<crate::game::Enemy>, Added<Children>)>,
    infantry_query: Query<(Entity, &Children), (With<crate::game::units::infantry::Infantry>, Without<crate::game::Enemy>, Added<Children>)>,
    aircraft_query: Query<(Entity, &Children), (With<crate::game::Aircraft>, Without<crate::game::Enemy>, Added<Children>)>,
    // Запрос для mesh-ей с материалами
    mesh_query: Query<(Entity, &Handle<StandardMaterial>), With<Handle<Mesh>>>,
    // Маркер для уже обработанных entities
    highlighted_query: Query<&PlayerHighlighted>,
) {
    // Обрабатываем танки игрока
    for (tank_entity, children) in tank_query.iter() {
        info!("Highlighting Player Tank entity {} with {} children", tank_entity.index(), children.len());
        
        highlight_player_entity_children(
            tank_entity, 
            children, 
            &mut commands, 
            &mut materials, 
            &mesh_query, 
            &highlighted_query,
            Color::rgb(0.2, 0.7, 0.3) // Зеленый для танков игрока
        );
    }
    
    // Обрабатываем пехоту игрока
    for (infantry_entity, children) in infantry_query.iter() {
        info!("Highlighting Player Infantry entity {} with {} children", infantry_entity.index(), children.len());
        
        highlight_player_entity_children(
            infantry_entity, 
            children, 
            &mut commands, 
            &mut materials, 
            &mesh_query, 
            &highlighted_query,
            Color::rgb(0.3, 0.6, 0.4) // Темно-зеленый для пехоты игрока
        );
    }
    
    // Обрабатываем авиацию игрока
    for (aircraft_entity, children) in aircraft_query.iter() {
        info!("Highlighting Player Aircraft entity {} with {} children", aircraft_entity.index(), children.len());
        
        highlight_player_entity_children(
            aircraft_entity, 
            children, 
            &mut commands, 
            &mut materials, 
            &mesh_query, 
            &highlighted_query,
            Color::rgb(0.1, 0.8, 0.2) // Яркий зеленый для авиации игрока
        );
    }
}

/// Рекурсивная функция для выделения всех mesh-потомков игровых юнитов
fn highlight_player_entity_children(
    parent_entity: Entity,
    children: &Children,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mesh_query: &Query<(Entity, &Handle<StandardMaterial>), With<Handle<Mesh>>>,
    highlighted_query: &Query<&PlayerHighlighted>,
    color: Color,
) {
    for &child in children.iter() {
        // Проверяем, не был ли уже обработан этот child
        if highlighted_query.get(child).is_ok() {
            continue;
        }
        
        // Если это mesh с материалом, изменяем материал
        if let Ok((mesh_entity, _material_handle)) = mesh_query.get(child) {
            info!("Changing material for Player mesh child {} of parent {}", mesh_entity.index(), parent_entity.index());
            
            // Создаем новый материал с зеленым цветом
            let new_material = StandardMaterial {
                base_color: color,
                metallic: 0.2,
                perceptual_roughness: 0.6,
                ..default()
            };
            
            let new_material_handle = materials.add(new_material);
            
            // Заменяем материал и добавляем маркер
            commands.entity(mesh_entity).insert((
                new_material_handle,
                PlayerHighlighted,
            ));
        }
    }
}

/// Система для визуального выделения примитивных (non-3D модели) игровых юнитов
pub fn highlight_primitive_player_entities(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Запрос для примитивных игровых юнитов
    tank_query: Query<(Entity, &Handle<StandardMaterial>), (With<crate::game::Tank>, Without<crate::game::Enemy>, Added<crate::game::Tank>)>,
    infantry_query: Query<(Entity, &Handle<StandardMaterial>), (With<crate::game::units::infantry::Infantry>, Without<crate::game::Enemy>, Added<crate::game::units::infantry::Infantry>)>,
    aircraft_query: Query<(Entity, &Handle<StandardMaterial>), (With<crate::game::Aircraft>, Without<crate::game::Enemy>, Added<crate::game::Aircraft>)>,
    // Маркер для уже обработанных entities
    highlighted_query: Query<&PlayerHighlighted>,
) {
    // Обрабатываем примитивные танки игрока
    for (tank_entity, material_handle) in tank_query.iter() {
        if highlighted_query.get(tank_entity).is_ok() {
            continue;
        }
        
        info!("Highlighting primitive Player Tank entity {}", tank_entity.index());
        
        let new_material = StandardMaterial {
            base_color: Color::rgb(0.2, 0.7, 0.3), // Зеленый для танков игрока
            metallic: 0.2,
            perceptual_roughness: 0.6,
            ..default()
        };
        
        let new_material_handle = materials.add(new_material);
        
        commands.entity(tank_entity).insert((
            new_material_handle,
            PlayerHighlighted,
        ));
    }
    
    // Обрабатываем примитивную пехоту игрока
    for (infantry_entity, material_handle) in infantry_query.iter() {
        if highlighted_query.get(infantry_entity).is_ok() {
            continue;
        }
        
        info!("Highlighting primitive Player Infantry entity {}", infantry_entity.index());
        
        let new_material = StandardMaterial {
            base_color: Color::rgb(0.3, 0.6, 0.4), // Темно-зеленый для пехоты игрока
            metallic: 0.2,
            perceptual_roughness: 0.6,
            ..default()
        };
        
        let new_material_handle = materials.add(new_material);
        
        commands.entity(infantry_entity).insert((
            new_material_handle,
            PlayerHighlighted,
        ));
    }
    
    // Обрабатываем примитивную авиацию игрока
    for (aircraft_entity, material_handle) in aircraft_query.iter() {
        if highlighted_query.get(aircraft_entity).is_ok() {
            continue;
        }
        
        info!("Highlighting primitive Player Aircraft entity {}", aircraft_entity.index());
        
        let new_material = StandardMaterial {
            base_color: Color::rgb(0.1, 0.8, 0.2), // Яркий зеленый для авиации игрока
            metallic: 0.2,
            perceptual_roughness: 0.6,
            ..default()
        };
        
        let new_material_handle = materials.add(new_material);
        
        commands.entity(aircraft_entity).insert((
            new_material_handle,
            PlayerHighlighted,
        ));
    }
}

/// Компонент-маркер для mesh-ей врагов, которые уже были выделены
#[derive(Component)]
pub struct EnemyHighlighted;

/// Компонент-маркер для mesh-ей игрока, которые уже были выделены
#[derive(Component)]
pub struct PlayerHighlighted;