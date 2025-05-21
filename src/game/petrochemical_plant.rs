use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::gizmos::gizmos::Gizmos;
use crate::game::components::{PetrochemicalPlant, FarmActive, FarmIncomeRate, OilProductionRate, Selectable, HoveredOutline, ShapeType};
use crate::game::resources::FarmIncomeTimer;
use crate::ui::money_ui::{Money, Oil, Wood, Steel};

/// System to update petrochemical plant income (money and oil)
pub fn update_petrochemical_plant_income(
    time: Res<Time>,
    mut farm_timer: ResMut<FarmIncomeTimer>,
    query: Query<(Entity, &FarmIncomeRate, &OilProductionRate, &FarmActive)>,
    mut money: ResMut<Money>,
    mut oil: ResMut<Oil>,
) {
    farm_timer.timer.tick(time.delta());
    
    if farm_timer.timer.just_finished() {
        let mut total_money_income = 0.0;
        let mut total_oil_income = 0.0;
        let mut active_plants = 0;
        
        // Detail each plant for debugging
        for (entity, income_rate, oil_rate, farm_active) in query.iter() {
            info!("Petrochemical Plant {}: active = {}, money income rate = {}/s, oil income rate = {}/s", 
                  entity.index(), farm_active.0, income_rate.0, oil_rate.0);
                  
            if farm_active.0 {
                total_money_income += income_rate.0;
                total_oil_income += oil_rate.0;
                active_plants += 1;
            }
        }
        
        // Log plant count for debugging
        info!("Petrochemical Plant income update: Found {} plants, {} are active, total money income: {}/s, total oil income: {}/s", 
               query.iter().count(), active_plants, total_money_income, total_oil_income);
        
        // Add income directly to resources
        if total_money_income > 0.0 || total_oil_income > 0.0 {
            money.0 += total_money_income;
            oil.0 += total_oil_income;
            info!("Petrochemical Plant income added: +{:.1} money, +{:.1} oil, new money total: {:.1}, new oil total: {:.1}", 
                  total_money_income, total_oil_income, money.0, oil.0);
        }
    }
}

/// System to handle clicks on petrochemical plants to activate/deactivate them
pub fn handle_petrochemical_plant_clicks(
    mut click_events: EventReader<Pointer<Click>>,
    mut plant_query: Query<(Entity, &mut FarmActive, &mut FarmIncomeRate, &mut OilProductionRate), With<PetrochemicalPlant>>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        info!("Petrochemical Plant click handler: click detected on entity {:?}", event.target);
        
        if let Ok((entity, mut farm_active, mut income_rate, mut oil_rate)) = plant_query.get_mut(event.target) {
            let old_status = farm_active.0;
            let old_money_rate = income_rate.0;
            let old_oil_rate = oil_rate.0;
            
            farm_active.0 = true;
            income_rate.0 = 0.7;
            oil_rate.0 = 0.3;

            info!("Petrochemical Plant {} status changed: {} -> {}, money rate: {} -> {}/s, oil rate: {} -> {}/s", 
                  entity.index(), old_status, farm_active.0, old_money_rate, income_rate.0, old_oil_rate, oil_rate.0);
        }
    }
}

/// System to update petrochemical plant material based on active status
pub fn update_petrochemical_plant_visuals(
    mut plant_query: Query<(&FarmActive, &mut Handle<StandardMaterial>), (With<PetrochemicalPlant>, Changed<FarmActive>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (active, material_handle) in plant_query.iter_mut() {
        let material = materials.get_mut(material_handle.id()).unwrap();
        
        if active.0 {
            material.base_color = Color::rgb(0.9, 0.1, 0.9);
            material.emissive = Color::rgb(0.3, 0.0, 0.3);
        } else {
            material.base_color = Color::rgb(0.6, 0.1, 0.6);
            material.emissive = Color::BLACK;
        }
    }
}

/// System to draw status indicators above petrochemical plants
pub fn draw_petrochemical_plant_status(
    mut gizmos: Gizmos,
    plant_query: Query<(&Transform, &FarmActive, &FarmIncomeRate, &OilProductionRate), With<PetrochemicalPlant>>,
    time: Res<Time>,
) {
    for (transform, active, income_rate, oil_rate) in &plant_query {
        let position = transform.translation + Vec3::new(0.0, 1.2, 0.0);
        
        if active.0 {
            let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.5 + 0.5;
            
            let money_intensity = (income_rate.0 / 10.0).min(1.0);
            let oil_intensity = (oil_rate.0 / 10.0).min(1.0);
            
            // Money symbol (yellow)
            let money_color = Color::rgba(0.9, 0.9 * money_intensity, 0.0, 0.5 + pulse * 0.5);
            let base_scale = 0.2;
            let money_scale = base_scale * (0.5 + money_intensity * 0.5);
            
            gizmos.line(
                position + Vec3::new(-0.5, 0.0, 0.0) + Vec3::new(0.0, money_scale * 0.5, 0.0), 
                position + Vec3::new(-0.5, 0.0, 0.0) + Vec3::new(0.0, -money_scale * 0.5, 0.0), 
                money_color
            );
            
            // Oil symbol (purple)
            let oil_color = Color::rgba(0.9 * oil_intensity, 0.1, 0.9 * oil_intensity, 0.5 + pulse * 0.5);
            let oil_scale = base_scale * (0.5 + oil_intensity * 0.5);
            
            gizmos.circle(
                position + Vec3::new(0.5, 0.0, 0.0),
                Direction3d::Z,
                oil_scale * 0.4,
                oil_color
            );
        }
    }
}

/// Create an inactive petrochemical plant that requires a click to activate
pub fn spawn_inactive_petrochemical_plant(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 1.0, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.6, 0.1, 0.6),
                ..default()
            }),
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.5, 0.0)),
            ..default()
        },
        Name::new("PetrochemicalPlant"),
        ShapeType::PetrochemicalPlant,
        Selectable,
        PetrochemicalPlant,
        FarmActive(false),
        FarmIncomeRate(0.0),
        OilProductionRate(0.0),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Create an active petrochemical plant
pub fn spawn_active_petrochemical_plant(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 1.0, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.9, 0.1, 0.9),
                emissive: Color::rgb(0.3, 0.0, 0.3),
                ..default()
            }),
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.5, 0.0)),
            ..default()
        },
        Name::new("ActivePetrochemicalPlant"),
        ShapeType::PetrochemicalPlant,
        Selectable,
        PetrochemicalPlant,
        FarmActive(true),
        FarmIncomeRate(0.7),
        OilProductionRate(0.3),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// System for creating petrochemical plants using a key press
pub fn spawn_petrochemical_plant_on_keystroke(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
    mut steel: ResMut<Steel>,
) {
    if keys.just_pressed(KeyCode::KeyP) {
        // Check if player has enough resources (10 money, 5 wood, 5 steel)
        if money.0 >= 10.0 && wood.0 >= 5.0 && steel.0 >= 5.0 {
            info!("Spawning a petrochemical plant, cost: 10 money, 5 wood, 5 steel");
            money.0 -= 10.0;
            wood.0 -= 5.0;
            steel.0 -= 5.0;
            
            // Spawn the petrochemical plant at a predetermined position
            let position = Vec3::new(10.0, 0.0, 5.0);
            
            // Immediately create an active plant, not an inactive one
            spawn_active_petrochemical_plant(
                &mut commands,
                &mut meshes,
                &mut materials,
                position,
            );
        } else {
            info!("Not enough resources to spawn a petrochemical plant! Need 10 money, 5 wood and 5 steel");
        }
    }
} 