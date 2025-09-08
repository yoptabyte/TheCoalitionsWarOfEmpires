use crate::game::MovementOrder;
use bevy::prelude::*;

/// Marker component for tanks that are currently playing movement sound
#[derive(Component)]
pub struct MovingTank;

/// Marker component for tank movement audio entities
#[derive(Component)]
pub struct TankMovementAudio;

/// processing movement orders
pub fn process_movement_orders(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Transform, &MovementOrder), Without<crate::game::components::Aircraft>>,
    tank_query: Query<Entity, With<crate::game::Tank>>,
    moving_tank_query: Query<Entity, With<MovingTank>>,
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
                let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, xz_direction);
                transform.rotation = transform.rotation.slerp(target_rotation, 0.2);
            }

            // Воспроизводим звук движения танка (только раз за движение)
            if tank_query.get(entity).is_ok() && moving_tank_query.get(entity).is_err() {
                // Добавляем звук и компонент движения (пока без spatial audio)
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/tank.mp3"),
                        settings: PlaybackSettings::LOOP,
                    },
                    TankMovementAudio,
                ));
                commands.entity(entity).insert(MovingTank);
                info!("🚗 Tank movement sound started");
            }

            if movement_this_frame.length_squared() >= direction.length_squared() {
                transform.translation = target;
                commands.entity(entity).remove::<MovementOrder>();
                // Убираем компонент движения танка когда достигли цели
                if tank_query.get(entity).is_ok() {
                    commands.entity(entity).remove::<MovingTank>();
                }
                info!("process_movement_orders: Object {:?} reached goal", entity);
            } else {
                transform.translation += movement_this_frame;
            }
        } else {
            transform.translation = target;
            commands.entity(entity).remove::<MovementOrder>();
            // Убираем компонент движения танка когда достигли цели (для случая близкой цели)
            if tank_query.get(entity).is_ok() {
                commands.entity(entity).remove::<MovingTank>();
            }
            info!(
                "process_movement_orders: Object {:?} reached goal (close)",
                entity
            );
        }
    }
}

/// System to stop tank movement audio when tanks are no longer moving
pub fn cleanup_tank_movement_audio(
    mut commands: Commands,
    tank_audio_query: Query<Entity, With<TankMovementAudio>>,
    moving_tank_query: Query<Entity, With<MovingTank>>,
) {
    // If no tanks are currently moving, remove all tank movement audio
    if moving_tank_query.is_empty() {
        for audio_entity in tank_audio_query.iter() {
            commands.entity(audio_entity).despawn();
            info!("🔇 Tank movement sound stopped - no moving tanks");
        }
    }
}
