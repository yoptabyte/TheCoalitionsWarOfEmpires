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

// Функция spawn_tower_on_keystroke удалена - не нужна 