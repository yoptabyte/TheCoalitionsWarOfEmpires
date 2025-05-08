use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::{ControllableCube, Selectable, Ground, MainCamera, HoveredOutline, ShapeType};

/// setup initial scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ControllableCube,
        Selectable,
        PickableBundle::default(),
        ShapeType::Cube,
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));

    let sphere_mesh = meshes.add(Mesh::from(Sphere::new(0.5)));
    commands.spawn((
        PbrBundle {
            mesh: sphere_mesh.clone(),
            material: materials.add(Color::rgb(0.2, 0.4, 0.8)),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..default()
        },
        Selectable,
        PickableBundle::default(),
        ShapeType::Sphere,
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));

    let plane_mesh = meshes.add(Plane3d::default().mesh().size(25.0, 25.0));
    commands.spawn((
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        PickableBundle::default(),
        Ground,
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera,
    ));
}