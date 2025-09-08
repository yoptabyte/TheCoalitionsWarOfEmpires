use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::gizmos::gizmos::Gizmos;
use crate::game::components::{Farm, ForestFarm, FarmActive, FarmIncomeRate, Selectable, HoveredOutline, ShapeType, Health};
use crate::game::resources::FarmIncomeTimer;
use crate::ui::money_ui::Money;

/// System to update farm income
pub fn update_farm_income(
    time: Res<Time>,
    mut farm_timer: ResMut<FarmIncomeTimer>,
    query: Query<(Entity, &FarmIncomeRate, &FarmActive)>,
    mut money: ResMut<Money>,
) {
    farm_timer.timer.tick(time.delta());
    
    if farm_timer.timer.just_finished() {
        let mut total_income = 0.0;
        let mut active_farms = 0;
        
        // Detail each farm for debugging
        for (entity, income_rate, farm_active) in query.iter() {
            info!("Farm {}: active = {}, income rate = {}/s", 
                  entity.index(), farm_active.0, income_rate.0);
                  
            if farm_active.0 {
                total_income += income_rate.0;
                active_farms += 1;
            }
        }
        
        // Log farm count for debugging
        info!("Farm income update: Found {} farms, {} are active, total income: {}/s", 
               query.iter().count(), active_farms, total_income);
        
        // Add income directly to money resource
        if total_income > 0.0 {
            money.0 += total_income;
            info!("Farm income added: +{:.1} money, new total: {:.1}", total_income, money.0);
        }
    }
}

/// System to handle clicks on farms to activate/deactivate them
pub fn handle_farm_clicks(
    mut click_events: EventReader<Pointer<Click>>,
    mut farm_query: Query<(Entity, &mut FarmActive, &mut FarmIncomeRate), With<Farm>>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        info!("Farm click handler: click detected on entity {:?}", event.target);
        
        if let Ok((entity, mut farm_active, mut income_rate)) = farm_query.get_mut(event.target) {
            let old_status = farm_active.0;
            let old_rate = income_rate.0;
            
            farm_active.0 = true;
            income_rate.0 = 0.2;

            info!("Farm {} status changed: {} -> {}, income rate: {} -> {}/s", 
                  entity.index(), old_status, farm_active.0, old_rate, income_rate.0);
        }
    }
}

/// System to update farm material based on active status
pub fn update_farm_visuals(
    mut farm_query: Query<(&FarmActive, &mut Handle<StandardMaterial>), (With<Farm>, Changed<FarmActive>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (active, material_handle) in farm_query.iter_mut() {
        let material = materials.get_mut(material_handle.id()).unwrap();
        
        if active.0 {
            material.base_color = Color::rgb(0.0, 0.9, 0.0);
            material.emissive = Color::rgb(0.0, 0.3, 0.0);
        } else {
            material.base_color = Color::rgb(0.0, 0.6, 0.0);
            material.emissive = Color::BLACK;
        }
    }
}

/// System to draw status indicators above farms
pub fn draw_farm_status(
    mut gizmos: Gizmos,
    farm_query: Query<(&Transform, &FarmActive, &FarmIncomeRate), With<Farm>>,
    time: Res<Time>,
) {
    for (transform, active, income_rate) in &farm_query {
        let position = transform.translation + Vec3::new(0.0, 1.2, 0.0);
        
        if active.0 {
            let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.5 + 0.5;
            
            let intensity = (income_rate.0 / 10.0).min(1.0); // Normalize to the range 0-1
            let color = Color::rgba(0.9, 0.9 * intensity, 0.0, 0.5 + pulse * 0.5);
            
            let base_scale = 0.2;
            let scale = base_scale * (0.5 + intensity * 0.5);
            
            gizmos.line(
                position + Vec3::new(0.0, scale * 0.5, 0.0), 
                position + Vec3::new(0.0, -scale * 0.5, 0.0), 
                color
            );
            
            gizmos.line(
                position + Vec3::new(0.0, scale * 0.5, 0.0), 
                position + Vec3::new(-scale * 0.3, scale * 0.3, 0.0), 
                color
            );
            
            gizmos.line(
                position + Vec3::new(-scale * 0.3, scale * 0.3, 0.0), 
                position + Vec3::new(scale * 0.3, -scale * 0.3, 0.0), 
                color
            );
            
            gizmos.line(
                position + Vec3::new(scale * 0.3, -scale * 0.3, 0.0), 
                position + Vec3::new(0.0, -scale * 0.5, 0.0), 
                color
            );
            
            if income_rate.0 >= 5.0 {
                let pos2 = position + Vec3::new(scale * 0.8, 0.0, 0.0);
                
                gizmos.line(
                    pos2 + Vec3::new(0.0, scale * 0.4, 0.0), 
                    pos2 + Vec3::new(0.0, -scale * 0.4, 0.0), 
                    color
                );
                
                gizmos.line(
                    pos2 + Vec3::new(0.0, scale * 0.4, 0.0), 
                    pos2 + Vec3::new(-scale * 0.2, scale * 0.2, 0.0), 
                    color
                );
                
                gizmos.line(
                    pos2 + Vec3::new(-scale * 0.2, scale * 0.2, 0.0), 
                    pos2 + Vec3::new(scale * 0.2, -scale * 0.2, 0.0), 
                    color
                );
                
                gizmos.line(
                    pos2 + Vec3::new(scale * 0.2, -scale * 0.2, 0.0), 
                    pos2 + Vec3::new(0.0, -scale * 0.4, 0.0), 
                    color
                );
            }
        }
    }
}

/// Spawn a forest farm at the given position
pub fn spawn_forest_farm(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
    position: Vec3,
    asset_server: &AssetServer,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/farm/forest.glb#Scene0"),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(0.2)),
            ..default()
        },
        Name::new("ForestFarm"),
        ShapeType::Farm,
        Selectable,
        Farm,
        ForestFarm,
        FarmActive(true),
        FarmIncomeRate(10.0),
        Health { current: 120.0, max: 120.0 },
        PickableBundle::default(),
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cuboid(1.0, 0.5, 1.0),
        bevy_rapier3d::prelude::LockedAxes::all(),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Spawn an active forest farm at the given position
pub fn spawn_active_forest_farm(
    commands: &mut Commands,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
    position: Vec3,
    asset_server: &AssetServer,
) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/farm/forest.glb#Scene0"),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::splat(0.2)),
            ..default()
        },
        Name::new("ActiveForestFarm"),
        ShapeType::Farm,
        Selectable,
        Farm,
        ForestFarm,
        FarmActive(true),
        FarmIncomeRate(10.0),
        Health { current: 120.0, max: 120.0 },
        PickableBundle::default(),
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cuboid(1.0, 0.5, 1.0),
        bevy_rapier3d::prelude::LockedAxes::all(),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Spawn a test forest farm with keyboard shortcut
pub fn spawn_forest_farm_on_keystroke(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Farm>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        info!("Spawning a test forest farm");
        
        // Determine the position for the new farm at the top of the map
        let position = find_free_position_in_area(&query, 5.0, Vec3::new(0.0, 0.0, -15.0), 8.0);
        
        info!("Found position for farm in upper area: {:?}", position);
        
        spawn_forest_farm(
            &mut commands,
            &mut meshes,
            &mut materials,
            position,
            &asset_server,
        );
    }
}

/// Finds a free position to place a building in the specified area
fn find_free_position_in_area(query: &Query<&Transform, With<Farm>>, min_distance: f32, center: Vec3, radius: f32) -> Vec3 {
    const MAX_ATTEMPTS: usize = 30;
    
    for _ in 0..MAX_ATTEMPTS {
        // Generate a random position within the maximum radius around the center of the area
        let uuid = bevy::utils::Uuid::new_v4();
        let bytes = uuid.as_bytes();
        let random_angle = (bytes[0] as f32 / 255.0) * std::f32::consts::TAU;
        let random_distance = (bytes[1] as f32 / 255.0) * radius;
        
        let test_position = center + Vec3::new(
            random_distance * random_angle.cos(),
            0.0,
            random_distance * random_angle.sin()
        );
        
        // Check if it's far enough from other objects
        let mut is_valid = true;
        for transform in query.iter() {
            let distance = (transform.translation - test_position).length();
            if distance < min_distance {
                is_valid = false;
                break;
            }
        }
        
        if is_valid {
            return test_position;
        }
    }
    
    // If no free space was found, return a random point in the area
    let uuid = bevy::utils::Uuid::new_v4();
    let bytes = uuid.as_bytes();
    let random_angle = (bytes[0] as f32 / 255.0) * std::f32::consts::TAU;
    let random_distance = (bytes[1] as f32 / 255.0) * radius;
    
    center + Vec3::new(
        random_distance * random_angle.cos(),
        0.0,
        random_distance * random_angle.sin()
    )
} 
