use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy::gizmos::gizmos::Gizmos;
use crate::game::components::{Mine, FarmActive, FarmIncomeRate, MineIronRate, Selectable, HoveredOutline, ShapeType};
use crate::game::resources::FarmIncomeTimer;
use crate::ui::money_ui::{Money, Iron, Wood};

/// System to update mine income (money and iron)
pub fn update_mine_income(
    time: Res<Time>,
    mut farm_timer: ResMut<FarmIncomeTimer>,
    query: Query<(Entity, &FarmIncomeRate, &MineIronRate, &FarmActive)>,
    mut money: ResMut<Money>,
    mut iron: ResMut<Iron>,
) {
    farm_timer.timer.tick(time.delta());
    
    if farm_timer.timer.just_finished() {
        let mut total_money_income = 0.0;
        let mut total_iron_income = 0.0;
        let mut active_mines = 0;
        
        // Detail each mine for debugging
        for (entity, income_rate, iron_rate, farm_active) in query.iter() {
            info!("Mine {}: active = {}, money income rate = {}/s, iron income rate = {}/s", 
                  entity.index(), farm_active.0, income_rate.0, iron_rate.0);
                  
            if farm_active.0 {
                total_money_income += income_rate.0;
                total_iron_income += iron_rate.0;
                active_mines += 1;
            }
        }
        
        // Log mine count for debugging
        info!("Mine income update: Found {} mines, {} are active, total money income: {}/s, total iron income: {}/s", 
               query.iter().count(), active_mines, total_money_income, total_iron_income);
        
        // Add income directly to resources
        if total_money_income > 0.0 || total_iron_income > 0.0 {
            money.0 += total_money_income;
            iron.0 += total_iron_income;
            info!("Mine income added: +{:.1} money, +{:.1} iron, new money total: {:.1}, new iron total: {:.1}", 
                  total_money_income, total_iron_income, money.0, iron.0);
        }
    }
}

/// System to handle clicks on mines to activate/deactivate them
pub fn handle_mine_clicks(
    mut click_events: EventReader<Pointer<Click>>,
    mut mine_query: Query<(Entity, &mut FarmActive, &mut FarmIncomeRate, &mut MineIronRate), With<Mine>>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Primary {
            continue;
        }
        
        info!("Mine click handler: click detected on entity {:?}", event.target);
        
        if let Ok((entity, mut farm_active, mut income_rate, mut iron_rate)) = mine_query.get_mut(event.target) {
            let old_status = farm_active.0;
            let old_money_rate = income_rate.0;
            let old_iron_rate = iron_rate.0;
            
            farm_active.0 = true;
            income_rate.0 = 0.6;
            iron_rate.0 = 0.2;

            info!("Mine {} status changed: {} -> {}, money rate: {} -> {}/s, iron rate: {} -> {}/s", 
                  entity.index(), old_status, farm_active.0, old_money_rate, income_rate.0, old_iron_rate, iron_rate.0);
        }
    }
}

/// System to update mine material based on active status
pub fn update_mine_visuals(
    mut mine_query: Query<(&FarmActive, &mut Handle<StandardMaterial>), (With<Mine>, Changed<FarmActive>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (active, material_handle) in mine_query.iter_mut() {
        let material = materials.get_mut(material_handle.id()).unwrap();
        
        if active.0 {
            material.base_color = Color::rgb(0.0, 0.0, 0.9);
            material.emissive = Color::rgb(0.0, 0.0, 0.3);
        } else {
            material.base_color = Color::rgb(0.0, 0.0, 0.6);
            material.emissive = Color::BLACK;
        }
    }
}

/// System to draw status indicators above mines
pub fn draw_mine_status(
    mut gizmos: Gizmos,
    mine_query: Query<(&Transform, &FarmActive, &FarmIncomeRate, &MineIronRate), With<Mine>>,
    time: Res<Time>,
) {
    for (transform, active, income_rate, iron_rate) in &mine_query {
        let position = transform.translation + Vec3::new(0.0, 1.2, 0.0);
        
        if active.0 {
            let pulse = (time.elapsed_seconds() * 2.0).sin() * 0.5 + 0.5;
            
            let money_intensity = (income_rate.0 / 10.0).min(1.0);
            let iron_intensity = (iron_rate.0 / 10.0).min(1.0);
            
            // Money symbol (yellow)
            let money_color = Color::rgba(0.9, 0.9 * money_intensity, 0.0, 0.5 + pulse * 0.5);
            let base_scale = 0.2;
            let money_scale = base_scale * (0.5 + money_intensity * 0.5);
            
            gizmos.line(
                position + Vec3::new(-0.5, 0.0, 0.0) + Vec3::new(0.0, money_scale * 0.5, 0.0), 
                position + Vec3::new(-0.5, 0.0, 0.0) + Vec3::new(0.0, -money_scale * 0.5, 0.0), 
                money_color
            );
            
            // Iron symbol (blue)
            let iron_color = Color::rgba(0.0, 0.0, 0.9 * iron_intensity, 0.5 + pulse * 0.5);
            let iron_scale = base_scale * (0.5 + iron_intensity * 0.5);
            
            gizmos.circle(
                position + Vec3::new(0.5, 0.0, 0.0),
                Direction3d::Z,
                iron_scale * 0.4,
                iron_color
            );
        }
    }
}

/// Spawn an inactive mine at the given position (requires click to activate)
pub fn spawn_inactive_mine(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 1.0, 2.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 0.6),
                ..default()
            }),
            transform: Transform::from_translation(position + Vec3::new(0.0, 0.5, 0.0)),
            ..default()
        },
        Name::new("InactiveMine"),
        ShapeType::Mine,
        Selectable,
        Mine,
        FarmActive(false),
        FarmIncomeRate(0.0),
        MineIronRate(0.0),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Spawn a mine on keystroke (M key)
pub fn spawn_mine_on_keystroke(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    mut wood: ResMut<Wood>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        // Check if player has enough resources (100 money, 35 wood)
        if money.0 >= 100.0 && wood.0 >= 35.0 {
            info!("Spawning a mine, cost: 100 money, 35 wood");
            money.0 -= 100.0;
            wood.0 -= 35.0;
            spawn_inactive_mine(
                &mut commands,
                &mut meshes,
                &mut materials,
                Vec3::new(0.0, 0.0, 0.0),
            );
        } else {
            info!("Not enough resources to spawn a mine! Need 100 money and 35 wood");
        }
    }
} 