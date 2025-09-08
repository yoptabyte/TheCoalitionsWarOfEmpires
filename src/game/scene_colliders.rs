use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use crate::game::components::{Tank, Enemy, Health, HoveredOutline, Aircraft};

/// Marker component for entities that need scene colliders
#[derive(Component)]
pub struct NeedsSceneCollider {
    pub collider_shape: ColliderShape,
}

/// Component to mark a mesh child as belonging to a clickable parent
#[derive(Component)]
pub struct ChildOfClickable {
    pub parent: Entity,
}

#[derive(Clone)]
pub enum ColliderShape {
    Tank,
    Infantry,
    Aircraft,
    Building,
}

/// System to automatically add colliders to Enemy entities with SceneBundle
pub fn add_enemy_scene_colliders(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Children), (With<crate::game::Enemy>, Added<Children>)>,
    enemy_tower_query: Query<(Entity, &Children), (With<crate::game::EnemyTower>, Added<Children>)>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
    child_query: Query<&ChildOfClickable>,
) {
    // Handle regular enemies
    for (enemy_entity, children) in enemy_query.iter() {
        info!("Processing NEW Enemy entity {} with {} children", enemy_entity.index(), children.len());
        
        // Find all mesh children and add colliders to them
        for &child in children.iter() {
            // Skip if already processed
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok(_) = mesh_query.get(child) {
                info!("Adding collider to Enemy mesh child {}", child.index());
                
                // Add collider and picking to the mesh child
                commands.entity(child).insert((
                    Collider::cuboid(1.0, 1.0, 1.0), // Generic enemy collider
                    PickableBundle::default(),
                    Sensor, // Make it a sensor so it doesn't interfere with physics
                    ChildOfClickable { parent: enemy_entity },
                ));
            }
        }
    }
    
    // Handle enemy towers
    for (tower_entity, children) in enemy_tower_query.iter() {
        info!("Processing NEW EnemyTower entity {} with {} children", tower_entity.index(), children.len());
        
        // Find all mesh children and add colliders to them
        for &child in children.iter() {
            // Skip if already processed
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok(_) = mesh_query.get(child) {
                info!("Adding collider to EnemyTower mesh child {}", child.index());
                
                // Add collider and picking to the mesh child
                commands.entity(child).insert((
                    Collider::cuboid(1.2, 1.5, 1.2), // Tower collider (taller)
                    PickableBundle::default(),
                    Sensor, // Make it a sensor so it doesn't interfere with physics
                    ChildOfClickable { parent: tower_entity },
                ));
            }
        }
    }
}

/// System to recursively find and add colliders to ALL Enemy mesh descendants
pub fn add_enemy_deep_scene_colliders(
    mut commands: Commands,
    enemy_query: Query<Entity, (With<crate::game::Enemy>, Added<Children>)>,
    enemy_tower_query: Query<Entity, (With<crate::game::EnemyTower>, Added<Children>)>,
    children_query: Query<&Children>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
) {
    // Handle regular enemies
    for enemy_entity in enemy_query.iter() {
        info!("Processing Enemy entity {} for deep colliders", enemy_entity.index());
        
        // Recursively find all mesh descendants
        let mut mesh_entities = Vec::new();
        find_mesh_descendants(enemy_entity, &children_query, &mesh_query, &mut mesh_entities);
        
        info!("Found {} mesh descendants for Enemy {}", mesh_entities.len(), enemy_entity.index());
        
        for mesh_entity in mesh_entities {
            commands.entity(mesh_entity).insert((
                Collider::cuboid(0.8, 0.8, 0.8), // Enemy mesh collider
                PickableBundle::default(),
                Sensor,
                ChildOfClickable { parent: enemy_entity },
            ));
            
            info!("Added deep mesh collider to {} for Enemy parent {}", mesh_entity.index(), enemy_entity.index());
        }
    }
    
    // Handle enemy towers
    for tower_entity in enemy_tower_query.iter() {
        info!("Processing EnemyTower entity {} for deep colliders", tower_entity.index());
        
        // Recursively find all mesh descendants
        let mut mesh_entities = Vec::new();
        find_mesh_descendants(tower_entity, &children_query, &mesh_query, &mut mesh_entities);
        
        info!("Found {} mesh descendants for EnemyTower {}", mesh_entities.len(), tower_entity.index());
        
        for mesh_entity in mesh_entities {
            commands.entity(mesh_entity).insert((
                Collider::cuboid(1.2, 1.5, 1.2), // Tower mesh collider (taller)
                PickableBundle::default(),
                Sensor,
                ChildOfClickable { parent: tower_entity },
            ));
            
            info!("Added deep mesh collider to {} for EnemyTower parent {}", mesh_entity.index(), tower_entity.index());
        }
    }
}

/// System to automatically add colliders to Player units (Tank, Infantry, Aircraft) with SceneBundle
pub fn add_player_unit_scene_colliders(
    mut commands: Commands,
    tank_query: Query<(Entity, &Children), (With<Tank>, Without<Enemy>, Added<Children>)>,
    infantry_query: Query<(Entity, &Children), (With<crate::game::units::infantry::Infantry>, Without<Enemy>, Added<Children>)>,
    aircraft_query: Query<(Entity, &Children), (With<Aircraft>, Without<Enemy>, Added<Children>)>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
    child_query: Query<&ChildOfClickable>,
) {
    // Handle player tanks
    for (tank_entity, children) in tank_query.iter() {
        info!("Processing NEW Player Tank entity {} with {} children", tank_entity.index(), children.len());
        
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok(_) = mesh_query.get(child) {
                info!("Adding collider to Player Tank mesh child {}", child.index());
                
                commands.entity(child).insert((
                    Collider::cuboid(2.0, 2.0, 2.5), // Tank collider - увеличен для лучшего клика
                    PickableBundle::default(),
                    Sensor,
                    ChildOfClickable { parent: tank_entity },
                    crate::game::Selectable, // Добавляем Selectable для raycast
                ));
            }
        }
    }
    
    // Handle player infantry
    for (infantry_entity, children) in infantry_query.iter() {
        info!("Processing NEW Player Infantry entity {} with {} children", infantry_entity.index(), children.len());
        
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok(_) = mesh_query.get(child) {
                info!("Adding collider to Player Infantry mesh child {}", child.index());
                
                commands.entity(child).insert((
                    Collider::ball(1.5), // Infantry collider - увеличен для лучшего клика
                    PickableBundle::default(),
                    Sensor,
                    ChildOfClickable { parent: infantry_entity },
                    crate::game::Selectable, // Добавляем Selectable для raycast
                ));
            }
        }
    }
    
    // Handle player aircraft
    for (aircraft_entity, children) in aircraft_query.iter() {
        info!("Processing NEW Player Aircraft entity {} with {} children", aircraft_entity.index(), children.len());
        
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok(_) = mesh_query.get(child) {
                info!("Adding collider to Player Aircraft mesh child {}", child.index());
                
                commands.entity(child).insert((
                    Collider::cuboid(2.5, 1.5, 3.0), // Aircraft collider - увеличен для лучшего клика
                    PickableBundle::default(),
                    Sensor,
                    ChildOfClickable { parent: aircraft_entity },
                    crate::game::Selectable, // Добавляем Selectable для raycast
                ));
            }
        }
    }
}

/// System to automatically add colliders to Player units that don't have children (primitive units)
pub fn add_player_primitive_unit_colliders(
    mut commands: Commands,
    tank_query: Query<Entity, (With<Tank>, Without<Enemy>, Without<Collider>, Added<Tank>)>,
    infantry_query: Query<Entity, (With<crate::game::units::infantry::Infantry>, Without<Enemy>, Without<Collider>, Added<crate::game::units::infantry::Infantry>)>,
    aircraft_query: Query<Entity, (With<Aircraft>, Without<Enemy>, Without<Collider>, Added<Aircraft>)>,
) {
    // Handle primitive player tanks
    for tank_entity in tank_query.iter() {
        info!("Adding collider to primitive Player Tank entity {}", tank_entity.index());
        
        commands.entity(tank_entity).insert((
            Collider::cuboid(2.0, 2.0, 2.5), // Tank collider - увеличен для лучшего клика
            PickableBundle::default(),
            Sensor,
        ));
    }
    
    // Handle primitive player infantry
    for infantry_entity in infantry_query.iter() {
        info!("Adding collider to primitive Player Infantry entity {}", infantry_entity.index());
        
        commands.entity(infantry_entity).insert((
            Collider::ball(1.5), // Infantry collider - увеличен для лучшего клика
            PickableBundle::default(),
            Sensor,
        ));
    }
    
    // Handle primitive player aircraft
    for aircraft_entity in aircraft_query.iter() {
        info!("Adding collider to primitive Player Aircraft entity {}", aircraft_entity.index());
        
        commands.entity(aircraft_entity).insert((
            Collider::cuboid(2.5, 1.5, 3.0), // Aircraft collider - увеличен для лучшего клика
            PickableBundle::default(),
            Sensor,
        ));
    }
}

/// System to add colliders to loaded scene meshes
pub fn add_scene_colliders(
    mut commands: Commands,
    needs_collider_query: Query<(Entity, &NeedsSceneCollider, &Children)>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
) {
    for (parent_entity, needs_collider, children) in needs_collider_query.iter() {
        // Find all mesh children
        for &child in children.iter() {
            if let Ok(_) = mesh_query.get(child) {
                // Add collider to mesh child
                let collider = match needs_collider.collider_shape {
                    ColliderShape::Tank => Collider::cuboid(2.0, 2.0, 2.5),
                    ColliderShape::Infantry => Collider::ball(1.5),
                    ColliderShape::Aircraft => Collider::cuboid(2.5, 1.5, 3.0),
                    ColliderShape::Building => Collider::cuboid(1.0, 0.5, 1.0),
                };

                // Add a marker component to the child to identify it as clickable
                commands.entity(child).insert((
                    collider,
                    PickableBundle::default(),
                    ChildOfClickable { parent: parent_entity },
                ));
                
                info!("Added collider to mesh child {} of parent {}", child.index(), parent_entity.index());
            }
        }
        
        // Remove the marker component once processed
        commands.entity(parent_entity).remove::<NeedsSceneCollider>();
    }
}

/// System to recursively search for mesh children in deeply nested scenes
pub fn add_deep_scene_colliders(
    mut commands: Commands,
    needs_collider_query: Query<(Entity, &NeedsSceneCollider), Added<NeedsSceneCollider>>,
    children_query: Query<&Children>,
    mesh_query: Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
) {
    for (parent_entity, needs_collider) in needs_collider_query.iter() {
        // Recursively find all mesh descendants
        let mut mesh_entities = Vec::new();
        find_mesh_descendants(parent_entity, &children_query, &mesh_query, &mut mesh_entities);
        
        for mesh_entity in mesh_entities {
            let collider = match needs_collider.collider_shape {
                ColliderShape::Tank => Collider::cuboid(2.0, 2.0, 2.5),
                ColliderShape::Infantry => Collider::ball(1.5),
                ColliderShape::Aircraft => Collider::cuboid(2.5, 1.5, 3.0),
                ColliderShape::Building => Collider::cuboid(1.0, 0.5, 1.0),
            };

            commands.entity(mesh_entity).insert((
                collider,
                PickableBundle::default(),
            ));
            
            info!("Added deep collider to mesh {} for parent {}", mesh_entity.index(), parent_entity.index());
        }
    }
}

fn find_mesh_descendants(
    entity: Entity,
    children_query: &Query<&Children>,
    mesh_query: &Query<Entity, (With<Handle<Mesh>>, Without<Collider>)>,
    mesh_entities: &mut Vec<Entity>,
) {
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            // Check if this child is a mesh
            if mesh_query.get(child).is_ok() {
                mesh_entities.push(child);
            }
            
            // Recursively search this child's children
            find_mesh_descendants(child, children_query, mesh_query, mesh_entities);
        }
    }
}

/// Вычисляет размер меша на основе его вершин
fn calculate_mesh_bounds(mesh: &Mesh) -> Option<Vec3> {
    if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        if positions.is_empty() {
            return None;
        }

        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for position in positions {
            let pos = Vec3::new(position[0], position[1], position[2]);
            min = min.min(pos);
            max = max.max(pos);
        }

        let size = max - min;
        // Добавляем небольшой отступ для надежности кликов
        let padding = 0.5;
        Some(Vec3::new(
            (size.x + padding).max(1.0),
            (size.y + padding).max(1.0), 
            (size.z + padding).max(1.0)
        ))
    } else {
        None
    }
}

/// Создает коллайдер на основе реального размера меша
fn create_mesh_based_collider(mesh_handle: &Handle<Mesh>, meshes: &Assets<Mesh>) -> Collider {
    if let Some(mesh) = meshes.get(mesh_handle) {
        if let Some(bounds) = calculate_mesh_bounds(mesh) {
            info!("📏 Создаю точный коллайдер размером: {:?}", bounds);
            return Collider::cuboid(bounds.x / 2.0, bounds.y / 2.0, bounds.z / 2.0);
        }
    }
    
    // Fallback - ОГРОМНЫЙ коллайдер, чтобы точно попасть
    warn!("⚠️ Не удалось вычислить размер меша, используем ОГРОМНЫЙ fallback коллайдер");
    Collider::cuboid(5.0, 5.0, 5.0)
}

/// Создает специально увеличенный коллайдер для танков и самолетов
fn create_oversized_collider_for_unit(unit_type: &str) -> Collider {
    match unit_type {
        "tank" => {
            info!("🚗 Создаю СУПЕР-ОГРОМНЫЙ коллайдер для танка (для mark1/tsar_tank)");
            Collider::cuboid(8.0, 8.0, 10.0) // СУПЕР-ОГРОМНЫЙ коллайдер для танков с множеством дочерних элементов
        },
        "aircraft" => {
            info!("✈️ Создаю ОГРОМНЫЙ коллайдер для самолета");
            Collider::cuboid(6.0, 3.0, 8.0) // Очень большой коллайдер для самолета
        },
        "infantry" => {
            info!("🏃 Создаю увеличенный коллайдер для пехоты");
            Collider::ball(2.5) // Увеличенная сфера для пехоты
        },
        _ => {
            info!("❓ Создаю стандартный огромный коллайдер");
            Collider::cuboid(3.0, 3.0, 3.0)
        }
    }
}

/// Система для добавления коллайдеров прямо на родительские сущности юнитов
pub fn add_parent_unit_colliders(
    mut commands: Commands,
    tank_query: Query<Entity, (With<Tank>, Without<Enemy>, Without<Collider>, Added<Tank>)>,
    infantry_query: Query<Entity, (With<crate::game::units::infantry::Infantry>, Without<Enemy>, Without<Collider>, Added<crate::game::units::infantry::Infantry>)>,
    aircraft_query: Query<Entity, (With<Aircraft>, Without<Enemy>, Without<Collider>, Added<Aircraft>)>,
) {
    // Добавляем коллайдеры прямо на родительские танки
    for tank_entity in tank_query.iter() {
        info!("🚗💥 ДОБАВЛЯЮ ОГРОМНЫЙ КОЛЛАЙДЕР ПРЯМО НА РОДИТЕЛЬСКИЙ ТАНК {}", tank_entity.index());
        
        commands.entity(tank_entity).insert((
            Collider::cuboid(5.0, 5.0, 6.0), // ОГРОМНЫЙ коллайдер
            PickableBundle::default(),
            Sensor,
        ));
    }
    
    // Добавляем коллайдеры прямо на родительскую пехоту
    for infantry_entity in infantry_query.iter() {
        info!("🏃💥 ДОБАВЛЯЮ ОГРОМНЫЙ КОЛЛАЙДЕР ПРЯМО НА РОДИТЕЛЬСКУЮ ПЕХОТУ {}", infantry_entity.index());
        
        commands.entity(infantry_entity).insert((
            Collider::ball(3.0), // ОГРОМНАЯ сфера
            PickableBundle::default(),
            Sensor,
        ));
    }
    
    // Добавляем коллайдеры прямо на родительские самолеты
    for aircraft_entity in aircraft_query.iter() {
        info!("✈️💥 ДОБАВЛЯЮ ОГРОМНЫЙ КОЛЛАЙДЕР ПРЯМО НА РОДИТЕЛЬСКИЙ САМОЛЕТ {}", aircraft_entity.index());
        
        commands.entity(aircraft_entity).insert((
            Collider::cuboid(8.0, 4.0, 10.0), // МАКСИМАЛЬНО ОГРОМНЫЙ коллайдер
            PickableBundle::default(),
            Sensor,
        ));
    }
}

/// Система для создания точных коллайдеров на основе размеров мешей игроков
pub fn add_precise_player_unit_colliders(
    mut commands: Commands,
    tank_query: Query<(Entity, &Children), (With<Tank>, Without<Enemy>, Added<Children>)>,
    infantry_query: Query<(Entity, &Children), (With<crate::game::units::infantry::Infantry>, Without<Enemy>, Added<Children>)>,
    aircraft_query: Query<(Entity, &Children), (With<Aircraft>, Without<Enemy>, Added<Children>)>,
    mesh_query: Query<(Entity, &Handle<Mesh>), (With<Handle<Mesh>>, Without<Collider>)>,
    child_query: Query<&ChildOfClickable>,
    meshes: Res<Assets<Mesh>>,
) {
    // Обработка танков игрока
    for (tank_entity, children) in tank_query.iter() {
        info!("🚗 Обрабатываю танк игрока {} с точными коллайдерами - всего детей: {}", 
              tank_entity.index(), children.len());
        
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                continue;
            }
            
            // Добавляем коллайдер ВСЕМ дочерним элементам, не только mesh-элементам
            info!("📐 Создаю СУПЕР-ОГРОМНЫЙ коллайдер для дочернего элемента танка {}", child.index());
            
            let collider = create_oversized_collider_for_unit("tank");
            
            commands.entity(child).insert((
                collider,
                PickableBundle::default(),
                Sensor,
                ChildOfClickable { parent: tank_entity },
                crate::game::Selectable, // Добавляем Selectable для raycast
            ));
        }
    }
    
    // Обработка пехоты игрока  
    for (infantry_entity, children) in infantry_query.iter() {
        info!("🏃 Обрабатываю пехоту игрока {} с точными коллайдерами", infantry_entity.index());
        
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok((_, mesh_handle)) = mesh_query.get(child) {
                info!("📐 Создаю ОГРОМНЫЙ коллайдер для меша пехоты {}", child.index());
                
                let collider = create_oversized_collider_for_unit("infantry");
                
                commands.entity(child).insert((
                    collider,
                    PickableBundle::default(),
                    Sensor,
                    ChildOfClickable { parent: infantry_entity },
                    crate::game::Selectable, // Добавляем Selectable для raycast
                ));
            }
        }
    }
    
    // Обработка самолетов игрока
    for (aircraft_entity, children) in aircraft_query.iter() {
        info!("✈️ Обрабатываю самолет игрока {} с точными коллайдерами", aircraft_entity.index());
        
        for &child in children.iter() {
            if child_query.get(child).is_ok() {
                continue;
            }
            
            if let Ok((_, mesh_handle)) = mesh_query.get(child) {
                info!("📐 Создаю ОГРОМНЫЙ коллайдер для меша самолета {}", child.index());
                
                let collider = create_oversized_collider_for_unit("aircraft");
                
                commands.entity(child).insert((
                    collider,
                    PickableBundle::default(),
                    Sensor,
                    ChildOfClickable { parent: aircraft_entity },
                    crate::game::Selectable, // Добавляем Selectable для raycast
                ));
            }
        }
    }
}

/// Система для обновления коллайдеров после загрузки мешей (fallback)
pub fn update_player_colliders_on_mesh_load(
    mut commands: Commands,
    mesh_query: Query<(Entity, &Handle<Mesh>, &Parent), (With<Handle<Mesh>>, With<Collider>, Changed<Handle<Mesh>>)>,
    parent_query: Query<Entity, Or<(With<Tank>, With<crate::game::units::infantry::Infantry>, With<Aircraft>)>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (mesh_entity, mesh_handle, parent) in mesh_query.iter() {
        // Проверяем, является ли родитель юнитом игрока
        if parent_query.get(parent.get()).is_ok() {
            info!("🔄 Обновляю коллайдер после загрузки меша для {}", mesh_entity.index());
            
            // Удаляем старый коллайдер и создаем новый точный
            commands.entity(mesh_entity).remove::<Collider>();
            
            let new_collider = create_mesh_based_collider(mesh_handle, &meshes);
            commands.entity(mesh_entity).insert(new_collider);
        }
    }
}

/// System to handle clicks on mesh children - just log for now
pub fn handle_child_clicks(
    mut click_events_reader: EventReader<Pointer<Click>>,
    child_query: Query<&ChildOfClickable>,
) {
    for event in click_events_reader.read() {
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            info!("Click detected on mesh child {} (parent: {})", 
                  event.target.index(), child_of_clickable.parent.index());
        }
    }
}

/// System to handle hover events on mesh children and forward them to parent entities
pub fn handle_child_hover(
    mut commands: Commands,
    mut over_events: EventReader<Pointer<Over>>,
    mut out_events: EventReader<Pointer<Out>>,
    child_query: Query<&ChildOfClickable>,
    parent_query: Query<Entity>, // Query to check if parent still exists
) {
    // Handle mouse over events
    for event in over_events.read() {
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            // Check if parent still exists before trying to modify it
            if parent_query.get(child_of_clickable.parent).is_ok() {
                commands.entity(child_of_clickable.parent).insert(HoveredOutline);
            }
        }
    }
    
    // Handle mouse out events
    for event in out_events.read() {
        if let Ok(child_of_clickable) = child_query.get(event.target) {
            // Check if parent still exists before trying to modify it
            if parent_query.get(child_of_clickable.parent).is_ok() {
                commands.entity(child_of_clickable.parent).remove::<HoveredOutline>();
            }
        }
    }
}