use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::components::{Aircraft, MovementOrder, Selectable, CanShoot};

pub fn aircraft_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Aircraft, &MovementOrder)>,
) {
    for (entity, mut transform, aircraft, movement_order) in query.iter_mut() {
        let target_position = movement_order.0;
        let current_position = transform.translation;
        
        // Skip movement if target is zero (no movement order)
        if target_position == Vec3::ZERO {
            continue;
        }
        
        // Calculate direction to target (only in XZ plane for aircraft)
        let direction_3d = target_position - current_position;
        let direction_xz = Vec3::new(direction_3d.x, 0.0, direction_3d.z);
        
        // Check if we've reached the target (within 1.0 unit distance)
        if direction_xz.length_squared() <= 1.0 {
            // Reached target, remove movement order
            commands.entity(entity).remove::<MovementOrder>();
            info!("Aircraft {:?} reached target at {:?}", entity, target_position);
            continue;
        }
        
        // Only proceed with movement and rotation if we have a valid direction
        if direction_xz.length_squared() > 0.01 {
            // Normalize direction for movement
            let normalized_direction = direction_xz.normalize();
            
            // Move the aircraft
            let movement = normalized_direction * aircraft.speed * time.delta_seconds();
            transform.translation += movement;
            
            // Keep the aircraft at the specified height
            transform.translation.y = aircraft.height;
            
            // Rotate the aircraft to face the movement direction
            let rotation = Quat::from_rotation_y(normalized_direction.x.atan2(normalized_direction.z));
            transform.rotation = rotation;
        }
    }
}

pub fn spawn_initial_aircraft(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(2.0, 0.5, 4.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.8, 0.8),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        },
        Aircraft {
            height: 10.0,
            speed: 5.0,
        },
        Selectable,
        MovementOrder(Vec3::ZERO),
        CanShoot {
            cooldown: 0.5,
            last_shot: 0.0,
            range: 20.0,
            damage: 15.0,
        },
        // Добавляем коллайдер и picking для кликабельности
        Collider::cuboid(7.0, 4.0, 8.0),
        Sensor,
        PickableBundle::default(),
        Name::new("Initial Aircraft"),
    ));
} 