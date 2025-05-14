use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::gizmos::gizmos::Gizmos;
use crate::game::components::{SteelFactory, FarmActive, FarmIncomeRate, SteelProductionRate, Selectable, HoveredOutline, ShapeType};
use crate::game::resources::FarmIncomeTimer;
use crate::ui::money_ui::{Money, Steel, Wood, Iron};

/// System to update steel factory income (money and steel)
pub fn update_steel_factory_income(
    time: Res<Time>,
    mut farm_timer: ResMut<FarmIncomeTimer>,
    query: Query<(Entity, &FarmIncomeRate, &SteelProductionRate, &FarmActive)>,
    mut money: ResMut<Money>,
    mut steel: ResMut<Steel>,
) {
    farm_timer.timer.tick(time.delta());
    
    if farm_timer.timer.just_finished() {
        let mut total_money_income = 0.0;
        let mut total_steel_income = 0.0;
        let mut active_factories = 0;
        
        // Detail each steel factory for debugging
        for (entity, income_rate, steel_rate, farm_active) in query.iter() {
            info!("Steel Factory {}: active = {}, money income rate = {}/s, steel production rate = {}/s", 
                  entity.index(), farm_active.0, income_rate.0, steel_rate.0);
                  
            if farm_active.0 {
                total_money_income += income_rate.0;
                total_steel_income += steel_rate.0;
                active_factories += 1;
            }
        }
        
        // Log steel factory count for debugging
        info!("Steel Factory income update: Found {} factories, {} are active, total money income: {}/s, total steel production: {}/s", 
               query.iter().count(), active_factories, total_money_income, total_steel_income);
        
        // Add income directly to resources
        if total_money_income > 0.0 || total_steel_income > 0.0 {
            money.0 += total_money_income;
            steel.0 += total_steel_income;
            info!("Steel Factory income added: +{:.1} money, +{:.1} steel, new money total: {:.1}, new steel total: {:.1}", 
                  total_money_income, total_steel_income, money.0, steel.0);
        }
    }
}

/// System to handle clicks on steel factories to activate/deactivate them
pub fn handle_steel_factory_clicks(
    mut click_events: EventReader<Pointer<Click>>,
    mut factory_query: Query<(Entity, &mut FarmActive, &mut FarmIncomeRate, &mut SteelProductionRate), With<SteelFactory>>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        info!("Steel Factory click handler: click detected on entity {:?}", event.target);
        
        if let Ok((entity, mut farm_active, mut income_rate, mut steel_rate)) = factory_query.get_mut(event.target) {
            let old_status = farm_active.0;
            let old_money_rate = income_rate.0;
            let old_steel_rate = steel_rate.0;
            
            farm_active.0 = true;
            income_rate.0 = 0.9;
            steel_rate.0 = 1.0;

            info!("Steel Factory {} status changed: {} -> {}, money rate: {} -> {}/s, steel rate: {} -> {}/s", 
                  entity.index(), old_status, farm_active.0, old_money_rate, income_rate.0, old_steel_rate, steel_rate.0);
        }
    }
}

/// System to update steel factory material based on active status
pub fn update_steel_factory_visuals(
    mut factory_query: Query<(&FarmActive, &mut Handle<StandardMaterial>), (With<SteelFactory>, Changed<FarmActive>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (active, material_handle) in factory_query.iter_mut() {
        let material = materials.get_mut(material_handle.id()).unwrap();
        
        if active.0 {
            material.base_color = Color::rgb(0.7, 0.3, 0.1);
            material.emissive = Color::rgb(0.3, 0.1, 0.0);
        } else {
            material.base_color = Color::rgb(0.5, 0.2, 0.1);
            material.emissive = Color::BLACK;
        }
    }
}

/// System to draw status indicators above steel factories
pub fn draw_steel_factory_status(
    mut gizmos: Gizmos,
    factory_query: Query<(&Transform, &FarmActive, &FarmIncomeRate, &SteelProductionRate), With<SteelFactory>>,
    time: Res<Time>,
) {
    for (transform, active, income_rate, steel_rate) in &factory_query {
        let position = transform.translation + Vec3::new(0.0, 1.2, 0.0);
        
        if active.0 {
            let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.5 + 0.5;
            
            let money_intensity = (income_rate.0 / 10.0).min(1.0);
            let steel_intensity = (steel_rate.0 / 10.0).min(1.0);
            
            // Money symbol (yellow)
            let money_color = Color::rgba(0.9, 0.9 * money_intensity, 0.0, 0.5 + pulse * 0.5);
            let base_scale = 0.2;
            let money_scale = base_scale * (0.5 + money_intensity * 0.5);
            
            gizmos.line(
                position + Vec3::new(-0.5, 0.0, 0.0) + Vec3::new(0.0, money_scale * 0.5, 0.0), 
                position + Vec3::new(-0.5, 0.0, 0.0) + Vec3::new(0.0, -money_scale * 0.5, 0.0), 
                money_color
            );
            
            // Steel symbol (orange-brown)
            let steel_color = Color::rgba(0.7, 0.3, 0.1, 0.5 + pulse * 0.5);
            let steel_scale = base_scale * (0.5 + steel_intensity * 0.5);
            
            gizmos.circle(
                position + Vec3::new(0.5, 0.0, 0.0),
                Direction3d::Z,
                steel_scale * 0.4,
                steel_color
            );
        }
    }
}

/// Spawn an inactive steel factory at the given position (requires click to activate)
pub fn spawn_inactive_steel_factory(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 1.5, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.5, 0.2, 0.1),
                ..default()
            }),
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.75, 0.0)),
            ..default()
        },
        Name::new("InactiveSteelFactory"),
        ShapeType::SteelFactory,
        Selectable,
        SteelFactory,
        FarmActive(false),
        FarmIncomeRate(0.0),
        SteelProductionRate(0.0),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Spawn a steel factory on keystroke (S key)
pub fn spawn_steel_factory_on_keystroke(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut iron: ResMut<Iron>,
    query: Query<&Transform, With<SteelFactory>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        // Check if player has enough resources (10 money, 2 wood, 2 iron)
        if money.0 >= 10.0 && wood.0 >= 2.0 && iron.0 >= 2.0 {
            info!("Spawning a steel factory, cost: 10 money, 2 wood, 2 iron");
            money.0 -= 10.0;
            wood.0 -= 2.0;
            iron.0 -= 2.0;
            
            // Determine the position for the new steel mill on the right side of the map
            let position = find_free_position_in_area(&query, 5.0, Vec3::new(15.0, 0.0, 0.0), 8.0);
            
            info!("Found position for steel factory in right area: {:?}", position);
            
            spawn_active_steel_factory(
                &mut commands,
                &mut meshes,
                &mut materials,
                position,
            );
        } else {
            info!("Not enough resources to spawn a steel factory! Need 10 money, 2 wood and 2 iron");
        }
    }
}

/// Spawn an active steel factory at the given position
pub fn spawn_active_steel_factory(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 1.5, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.7, 0.3, 0.1),
                emissive: Color::rgb(0.3, 0.1, 0.0),
                ..default()
            }),
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.75, 0.0)),
            ..default()
        },
        Name::new("ActiveSteelFactory"),
        ShapeType::SteelFactory,
        Selectable,
        SteelFactory,
        FarmActive(true),
        FarmIncomeRate(0.9),
        SteelProductionRate(1.0),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Finds a free position to place a building in the specified area
fn find_free_position_in_area(query: &Query<&Transform, With<SteelFactory>>, min_distance: f32, center: Vec3, radius: f32) -> Vec3 {
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
        
        // Check if the position is far enough from other objects
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
    
    // If no free space is found, return a random point in the area
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