use bevy::prelude::*;
use crate::game::components::{Enemy, EnemyTower, Selectable, Tower};

/// Plugin for managing visual markers
pub struct EnemyVisualMarkersPlugin;

impl Plugin for EnemyVisualMarkersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            draw_unit_markers.run_if(in_state(crate::menu::common::GameState::Game))
        );
    }
}

/// System to draw simple circular markers above units and towers
fn draw_unit_markers(
    mut gizmos: Gizmos,
    enemy_query: Query<&Transform, (Or<(With<Enemy>, With<EnemyTower>)>, Without<Tower>)>,
    enemy_tower_query: Query<&Transform, With<EnemyTower>>,
    player_query: Query<&Transform, (With<Selectable>, Without<Enemy>, Without<EnemyTower>)>,
    player_tower_query: Query<&Transform, (With<Tower>, Without<EnemyTower>)>,
) {
    // Draw red circles above enemy units
    for transform in enemy_query.iter() {
        // Skip if position is at origin or invalid
        if transform.translation == Vec3::ZERO || transform.translation.length() < 0.1 {
            continue;
        }
        let marker_pos = transform.translation + Vec3::new(0.0, 4.0, 0.0);
        gizmos.circle(
            marker_pos,
            Direction3d::Y,
            0.8,
            Color::rgba(1.0, 0.0, 0.0, 0.9), // Bright red with slight transparency
        );
    }
    
    // Draw red circles above enemy towers (higher up)
    for transform in enemy_tower_query.iter() {
        // Skip if position is at origin or invalid
        if transform.translation == Vec3::ZERO || transform.translation.length() < 0.1 {
            continue;
        }
        let marker_pos = transform.translation + Vec3::new(0.0, 30.0, 0.0);
        gizmos.circle(
            marker_pos,
            Direction3d::Y,
            1.5,
            Color::rgba(1.0, 0.0, 0.0, 0.9), // Bright red with slight transparency
        );
    }
    
    // Draw green circles above player units
    for transform in player_query.iter() {
        // Skip if position is at origin or invalid
        if transform.translation == Vec3::ZERO || transform.translation.length() < 0.1 {
            continue;
        }
        let marker_pos = transform.translation + Vec3::new(0.0, 4.0, 0.0);
        gizmos.circle(
            marker_pos,
            Direction3d::Y,
            0.8,
            Color::rgba(0.0, 1.0, 0.0, 0.9), // Bright green with slight transparency
        );
    }
    
    // Draw green circles above player towers (higher up)
    for transform in player_tower_query.iter() {
        // Skip if position is at origin or invalid
        if transform.translation == Vec3::ZERO || transform.translation.length() < 0.1 {
            continue;
        }
        let marker_pos = transform.translation + Vec3::new(0.0, 30.0, 0.0);
        gizmos.circle(
            marker_pos,
            Direction3d::Y,
            1.5,
            Color::rgba(0.0, 1.0, 0.0, 0.9), // Bright green with slight transparency
        );
    }
}
