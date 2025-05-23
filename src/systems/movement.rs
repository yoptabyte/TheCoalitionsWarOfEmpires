use crate::game::MovementOrder;
use bevy::prelude::*;

/// processing movement orders
pub fn process_movement_orders(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MovementOrder)>,
    time: Res<Time>,
) {
    for (entity, mut transform, movement_order) in query.iter_mut() {
        let target = movement_order.0;
        let speed = 2.0;
        let direction = target - transform.translation;

        if direction.length_squared() > 0.01 {
            let movement_this_frame = direction.normalize() * speed * time.delta_seconds();
            let xz_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();

            if xz_direction.length_squared() > 0.001 {
                let target_rotation = Quat::from_rotation_arc(Vec3::Z, xz_direction);
                transform.rotation = transform.rotation.slerp(target_rotation, 0.2);
            }

            if movement_this_frame.length_squared() >= direction.length_squared() {
                transform.translation = target;
                commands.entity(entity).remove::<MovementOrder>();
                info!("process_movement_orders: Object {:?} reached goal", entity);
            } else {
                transform.translation += movement_this_frame;
            }
        } else {
            transform.translation = target;
            commands.entity(entity).remove::<MovementOrder>();
            info!(
                "process_movement_orders: Object {:?} reached goal (close)",
                entity
            );
        }
    }
}
