use bevy::prelude::*;
use crate::game::components::{
    Health, Tower, EnemyTower, Tank, Aircraft,
    Farm, Mine, SteelFactory, PetrochemicalPlant
};
use crate::game::units::infantry::Infantry;

/// System to draw health bars above towers and units
pub fn draw_health_bars(
    mut gizmos: Gizmos,
    // Towers
    tower_query: Query<(&Transform, &Health), With<Tower>>,
    // Units
    tank_query: Query<(&Transform, &Health), (With<Tank>, Without<Tower>)>,
    aircraft_query: Query<(&Transform, &Health), (With<Aircraft>, Without<Tower>, Without<Tank>)>,
    infantry_query: Query<(&Transform, &Health), (With<Infantry>, Without<Tower>, Without<Tank>, Without<Aircraft>)>,
    // Buildings
    farm_query: Query<(&Transform, &Health), (With<Farm>, Without<Tower>, Without<Tank>, Without<Aircraft>, Without<Infantry>)>,
    mine_query: Query<(&Transform, &Health), (With<Mine>, Without<Tower>, Without<Tank>, Without<Aircraft>, Without<Infantry>, Without<Farm>)>,
    steel_factory_query: Query<(&Transform, &Health), (With<SteelFactory>, Without<Tower>, Without<Tank>, Without<Aircraft>, Without<Infantry>, Without<Farm>, Without<Mine>)>,
    oil_pump_query: Query<(&Transform, &Health), (With<PetrochemicalPlant>, Without<Tower>, Without<Tank>, Without<Aircraft>, Without<Infantry>, Without<Farm>, Without<Mine>, Without<SteelFactory>)>,
) {
    // Draw health bars for towers
    for (transform, health) in tower_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 25.0, 3.0, 0.5);
    }
    
    // Draw health bars for tanks
    for (transform, health) in tank_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 4.0, 2.0, 0.3);
    }
    
    // Draw health bars for aircraft
    for (transform, health) in aircraft_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 2.0, 2.0, 0.3);
    }
    
    // Draw health bars for infantry
    for (transform, health) in infantry_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 3.0, 1.5, 0.25);
    }
    
    // Draw health bars for farms
    for (transform, health) in farm_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 8.0, 2.5, 0.4);
    }
    
    // Draw health bars for mines
    for (transform, health) in mine_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 6.0, 2.0, 0.35);
    }
    
    // Draw health bars for steel factories
    for (transform, health) in steel_factory_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 10.0, 3.0, 0.45);
    }
    
    // Draw health bars for oil pumps
    for (transform, health) in oil_pump_query.iter() {
        draw_health_bar(&mut gizmos, transform, health, 7.0, 2.2, 0.4);
    }
}

/// Helper function to draw a health bar at the specified position
fn draw_health_bar(
    gizmos: &mut Gizmos,
    transform: &Transform,
    health: &Health,
    height_offset: f32,
    bar_width: f32,
    bar_height: f32,
) {
    let pos = transform.translation;
    
    // Skip if position is at origin or invalid
    if pos == Vec3::ZERO || pos.length() < 0.1 {
        return;
    }
    
    let health_percentage = health.current / health.max;
    
    // Health bar position (above the entity)
    let bar_pos = pos + Vec3::new(0.0, height_offset, 0.0);
    
    // Background bar (red)
    gizmos.rect(
        bar_pos,
        Quat::IDENTITY,
        Vec2::new(bar_width, bar_height),
        Color::rgba(0.8, 0.2, 0.2, 0.8), // Semi-transparent red
    );
    
    // Current health (green) - fill
    let current_bar_width = bar_width * health_percentage;
    if current_bar_width > 0.0 {
        let current_bar_pos = bar_pos + Vec3::new(-(bar_width - current_bar_width) / 2.0, 0.0, 0.0);
        gizmos.rect(
            current_bar_pos,
            Quat::IDENTITY,
            Vec2::new(current_bar_width, bar_height),
            Color::rgba(0.2, 0.8, 0.2, 0.9), // Bright green
        );
    }
    
    // Health bar outline (black)
    gizmos.rect(
        bar_pos,
        Quat::IDENTITY,
        Vec2::new(bar_width + 0.1, bar_height + 0.1),
        Color::BLACK,
    );
}
