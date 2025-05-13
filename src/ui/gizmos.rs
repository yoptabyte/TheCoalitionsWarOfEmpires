use bevy::prelude::*;
use bevy::gizmos::gizmos::Gizmos;
use crate::game::{ClickCircle, MovementOrder, Selectable, HoveredOutline, ShapeType, Health};

/// constants for the click circle
pub const CIRCLE_LIFETIME: f32 = 0.5; // how long the circle exists in seconds
pub const CIRCLE_COLOR: Color = Color::YELLOW; // corresponds to the initial color of the particles
pub const CIRCLE_FINAL_RADIUS: f32 = 1.0; // final radius of the expanding circle

/// system for drawing a fading circle gizmo in the last click position.
pub fn draw_click_circle(
    mut gizmos: Gizmos,
    click_circle: Res<ClickCircle>,
    time: Res<Time>,
) {
    if let (Some(pos), Some(spawn_time)) = (click_circle.position, click_circle.spawn_time) {
        let elapsed = time.elapsed_seconds() - spawn_time;
        if elapsed < CIRCLE_LIFETIME {
            let progress = elapsed / CIRCLE_LIFETIME; // progress from 0.0 to 1.0
            let alpha = 1.0 - progress; // fading opacity
            let current_radius = CIRCLE_FINAL_RADIUS * progress; // expanding radius

            let color = CIRCLE_COLOR.with_a(alpha);
            gizmos.circle(
                pos + Vec3::Y * 0.01, 
                Direction3d::Y,      
                current_radius,
                color,
            );
        }
    }
}

/// system for drawing movement lines
pub fn draw_movement_lines(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &MovementOrder), With<Selectable>>,
) {
    for (transform, movement_order) in query.iter() {
        let start = transform.translation;
        let end = movement_order.0;
        
        gizmos.line(
            start, 
            end, 
            Color::BLUE
        );
        
        let direction = (end - start).normalize();
        let arrow_length = 0.3;
        let arrow_angle = 0.6; 
        
        let perpendicular = Vec3::new(-direction.z, 0.0, direction.x).normalize();
        
        let arrow_left = end - direction * arrow_length + perpendicular * arrow_length * arrow_angle;
        let arrow_right = end - direction * arrow_length - perpendicular * arrow_length * arrow_angle;
        
        gizmos.line(end, arrow_left, Color::BLUE);
        gizmos.line(end, arrow_right, Color::BLUE);
    }
}

/// system for drawing a yellow square on the ground under objects under the cursor.
pub fn draw_hover_outline(
    mut gizmos: Gizmos,
    hovered_entities_query: Query<(&Transform, &ShapeType), (With<HoveredOutline>, With<Selectable>)>,
    selected_entity: Res<crate::game::SelectedEntity>,
    transform_query: Query<&Transform>,
    shape_type_query: Query<&ShapeType>,
) {
    // Draw outline for hovered entities
    for (transform, shape_type) in hovered_entities_query.iter() {
        let world_position = transform.translation;
        let size = match shape_type {
            ShapeType::Cube => 1.0, // cube size
            ShapeType::Sphere => 1.0, // sphere diameter
            ShapeType::Airplane => 4.0, // airplane length
            ShapeType::Tower => 2.0, // tower base size
            ShapeType::Farm => 2.0, // farm size
        };
        
        // Draw a square on the ground
        let half_size = size / 2.0;
        let corners = [
            Vec3::new(world_position.x - half_size, 0.01, world_position.z - half_size),
            Vec3::new(world_position.x + half_size, 0.01, world_position.z - half_size),
            Vec3::new(world_position.x + half_size, 0.01, world_position.z + half_size),
            Vec3::new(world_position.x - half_size, 0.01, world_position.z + half_size),
        ];
        
        // Draw the square
        gizmos.line(corners[0], corners[1], Color::YELLOW);
        gizmos.line(corners[1], corners[2], Color::YELLOW);
        gizmos.line(corners[2], corners[3], Color::YELLOW);
        gizmos.line(corners[3], corners[0], Color::YELLOW);
    }
    
    // Draw outline for selected entity
    if let Some(entity) = selected_entity.0 {
        if let (Ok(transform), Ok(shape_type)) = (transform_query.get(entity), shape_type_query.get(entity)) {
            let world_position = transform.translation;
            let size = match shape_type {
                ShapeType::Cube => 1.0,
                ShapeType::Sphere => 1.0,
                ShapeType::Airplane => 4.0,
                ShapeType::Tower => 2.0,
                ShapeType::Farm => 2.0,
            };
            
            // Draw a square on the ground with a different color
            let half_size = size / 2.0;
            let corners = [
                Vec3::new(world_position.x - half_size, 0.01, world_position.z - half_size),
                Vec3::new(world_position.x + half_size, 0.01, world_position.z - half_size),
                Vec3::new(world_position.x + half_size, 0.01, world_position.z + half_size),
                Vec3::new(world_position.x - half_size, 0.01, world_position.z + half_size),
            ];
            
            // Draw the square with a green color for selected entity
            gizmos.line(corners[0], corners[1], Color::GREEN);
            gizmos.line(corners[1], corners[2], Color::GREEN);
            gizmos.line(corners[2], corners[3], Color::GREEN);
            gizmos.line(corners[3], corners[0], Color::GREEN);
        }
    }
}

/// system for displaying health bars above objects
pub fn draw_health_bars(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Health, Option<&crate::game::EnemyTower>)>,
    _camera_query: Query<(&Transform, &Camera), With<Camera>>,
) {
    for (transform, health, is_tower) in query.iter() {
        if health.max <= 0.0 {
            continue;
        }
        
        // Определяем позицию для отрисовки
        let mut position = transform.translation + Vec3::new(0.0, 1.5, 0.0);
        
        // Для башни рисуем полосу здоровья выше
        if is_tower.is_some() {
            position = transform.translation + Vec3::new(0.0, 7.0, 0.0);
        }
        
        let health_ratio = health.current / health.max;
        let bar_width = 1.0;
        let filled_width = bar_width * health_ratio;
        
        // background of the health bar
        gizmos.line(
            position + Vec3::new(-bar_width/2.0, 0.0, 0.0),
            position + Vec3::new(bar_width/2.0, 0.0, 0.0),
            Color::DARK_GRAY,
        );
        
        let health_color = if health_ratio > 0.7 {
            Color::GREEN
        } else if health_ratio > 0.3 {
            Color::YELLOW
        } else {
            Color::RED
        };
        
        // fill the health bar
        if health_ratio > 0.0 {
            gizmos.line(
                position + Vec3::new(-bar_width/2.0, 0.0, 0.0),
                position + Vec3::new(-bar_width/2.0 + filled_width, 0.0, 0.0),
                health_color,
            );
        }
    }
}