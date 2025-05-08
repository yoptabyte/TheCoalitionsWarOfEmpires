use bevy::prelude::*;
use bevy::gizmos::gizmos::Gizmos;
use crate::game::{ClickCircle, MovementOrder, Selectable, HoveredOutline, ShapeType};

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

/// system for drawing a yellow outline around objects under the cursor.
pub fn draw_hover_outline(
    mut gizmos: Gizmos,
    hovered_entities_query: Query<(&Transform, &ShapeType, Option<&Handle<Mesh>>), (With<HoveredOutline>, With<Selectable>)>,
    meshes: Res<Assets<Mesh>>,
) {
    for (transform, shape_type, mesh_handle) in hovered_entities_query.iter() {
        match shape_type {
            ShapeType::Cube => {
                // drawing the outline for the cube using the AABB
                if let Some(mesh_handle) = mesh_handle {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(aabb) = mesh.compute_aabb() {
                            // getting the corners of the AABB in the local space
                            let min = aabb.min();
                            let max = aabb.max();
                            let corners = [
                                Vec3::new(min.x, min.y, min.z),
                                Vec3::new(max.x, min.y, min.z),
                                Vec3::new(max.x, max.y, min.z),
                                Vec3::new(min.x, max.y, min.z),
                                Vec3::new(min.x, min.y, max.z),
                                Vec3::new(max.x, min.y, max.z),
                                Vec3::new(max.x, max.y, max.z),
                                Vec3::new(min.x, max.y, max.z),
                            ];

                            // transforming the corners to the world space
                            let world_corners: Vec<Vec3> = corners
                                .iter()
                                .map(|&corner| transform.transform_point(corner))
                                .collect();

                            // defining the edges of the cube based on the corners
                            let edges = [
                                (world_corners[0], world_corners[1]), (world_corners[1], world_corners[2]),
                                (world_corners[2], world_corners[3]), (world_corners[3], world_corners[0]),
                                (world_corners[4], world_corners[5]), (world_corners[5], world_corners[6]),
                                (world_corners[6], world_corners[7]), (world_corners[7], world_corners[4]),
                                (world_corners[0], world_corners[4]), (world_corners[1], world_corners[5]),
                                (world_corners[2], world_corners[6]), (world_corners[3], world_corners[7]),
                            ];

                            for (start, end) in edges.iter() {
                                gizmos.line(*start, *end, Color::YELLOW);
                            }
                        }
                    }
                }
            },
            ShapeType::Sphere => {
                // drawing the spherical outline for the sphere
                let radius = 0.5; // sphere radius
                let world_position = transform.translation;
                
                // drawing three circles in orthogonal planes to create the sphere effect
                // circle in the XY plane (normal Z)
                gizmos.circle(world_position, Direction3d::Z, radius, Color::YELLOW);
                // circle in the YZ plane (normal X)
                gizmos.circle(world_position, Direction3d::X, radius, Color::YELLOW);
                // circle in the XZ plane (normal Y)
                gizmos.circle(world_position, Direction3d::Y, radius, Color::YELLOW);
            }
        }
    }
}