use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::{Health, Tower};

/// System to repair tower when clicked while selected
pub fn repair_tower(
    mut click_events: EventReader<Pointer<Click>>,
    mut query: Query<&mut Health, With<Tower>>,
    _time: Res<Time>,
) {
    for event in click_events.read() {
        if event.button != PointerButton::Secondary {
            continue;
        }
        
        if let Ok(mut health) = query.get_mut(event.target) {
            // Right-click on a tower repairs it
            let repair_amount = 10.0;
            health.current = (health.current + repair_amount).min(health.max);
            info!("Tower repaired: {}/{}", health.current, health.max);
        }
    }
}

/// System to update tower health status
pub fn update_tower_health_status(
    mut commands: Commands,
    query: Query<(Entity, &Health, &Transform), With<Tower>>,
    _time: Res<Time>,
) {
    for (entity, health, transform) in query.iter() {
        // Check if the tower is destroyed
        if health.current <= 0.0 {
            info!("Tower destroyed!");
            commands.entity(entity).despawn_recursive();
        }
        // You could add more status effects here, like smoke particles at low health
        else if health.current < health.max * 0.3 {
            // Tower is heavily damaged (less than 30% health)
            // You could spawn smoke particles or change appearance
            let tower_position = transform.translation;
            info!("Tower heavily damaged at position: {:?}", tower_position);
        }
    }
}

/// System to spawn towers at predefined locations using keyboard shortcuts
pub fn spawn_tower_on_keystroke(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Spawn a tower at a fixed position when 'T' is pressed
    if keyboard.just_pressed(KeyCode::KeyT) {
        let tower_position = Vec3::new(0.0, 0.0, -5.0); // Default position
        info!("Spawning new tower at position: {:?}", tower_position);
        crate::game::setup::spawn_tower(
            &mut commands,
            &mut meshes,
            &mut materials,
            tower_position,
        );
    }
    
    // Spawn towers at different positions with different keys
    if keyboard.just_pressed(KeyCode::Digit1) {
        let tower_position = Vec3::new(-5.0, 0.0, -5.0);
        info!("Spawning tower 1 at position: {:?}", tower_position);
        crate::game::setup::spawn_tower(
            &mut commands,
            &mut meshes,
            &mut materials,
            tower_position,
        );
    }
    
    if keyboard.just_pressed(KeyCode::Digit2) {
        let tower_position = Vec3::new(5.0, 0.0, -5.0);
        info!("Spawning tower 2 at position: {:?}", tower_position);
        crate::game::setup::spawn_tower(
            &mut commands,
            &mut meshes,
            &mut materials,
            tower_position,
        );
    }
} 