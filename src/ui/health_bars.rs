use bevy::prelude::*;
use crate::game::{Health, Tower, EnemyTower};

/// System to draw health bars above towers and units
pub fn draw_health_bars(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Health), With<Tower>>,
) {
    for (transform, health) in query.iter() {
        let pos = transform.translation;
        let health_percentage = health.current / health.max;
        
        // Health bar position (above the tower)
        let bar_pos = pos + Vec3::new(0.0, 8.0, 0.0);
        let bar_width = 3.0;
        let bar_height = 0.5;
        
        // Background bar (red)
        let health_bar_pos = bar_pos;
        
        // Рисуем полоску здоровья с заливкой
        gizmos.rect(
            health_bar_pos,
            Quat::IDENTITY,
            Vec2::new(bar_width, bar_height),
            Color::rgba(0.8, 0.2, 0.2, 0.8), // Полупрозрачный красный
        );
        
        // Текущее здоровье (зеленый) - заливка
        let current_bar_width = bar_width * health_percentage;
        if current_bar_width > 0.0 {
            let current_bar_pos = health_bar_pos + Vec3::new(-(bar_width - current_bar_width) / 2.0, 0.0, 0.0);
            gizmos.rect(
                current_bar_pos,
                Quat::IDENTITY,
                Vec2::new(current_bar_width, bar_height),
                Color::rgba(0.2, 0.8, 0.2, 0.9), // Яркий зеленый
            );
        }
        
        // Контур полоски здоровья (черный)
        gizmos.rect(
            health_bar_pos,
            Quat::IDENTITY,
            Vec2::new(bar_width + 0.1, bar_height + 0.1),
            Color::BLACK,
        );
        
        // Health text
        if health.current < health.max {
            // Only show health numbers if damaged
            println!("Tower health: {:.0}/{:.0}", health.current, health.max);
        }
    }
}
