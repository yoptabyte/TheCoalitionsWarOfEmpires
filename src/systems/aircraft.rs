use bevy::prelude::*;
use crate::game::components::{Aircraft, MovementOrder, Selectable, CanShoot};

pub fn aircraft_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Aircraft, &MovementOrder)>,
) {
    for (mut transform, aircraft, movement_order) in query.iter_mut() {
        let target_position = movement_order.0;
        let current_position = transform.translation;
        
        // Calculate direction to target
        let direction = (target_position - current_position).normalize();
        
        // Move the aircraft
        let movement = direction * aircraft.speed * time.delta_seconds();
        transform.translation += movement;
        
        // Keep the aircraft at the specified height
        transform.translation.y = aircraft.height;
        
        // Rotate the aircraft to face the movement direction
        if direction.length_squared() > 0.0 {
            let rotation = Quat::from_rotation_y(direction.x.atan2(direction.z));
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
    ));
} 