use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::{Ground, MainCamera, Enemy, Selectable, ShapeType, Health, HoveredOutline, CanShoot, Tower, EnemyTower};
use crate::game::farm::{spawn_inactive_forest_farm};

/// setup initial scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let sphere_mesh = meshes.add(Mesh::from(Sphere::new(0.5)));

    // enemy objects
    // enemy 1 - red cube
    commands.spawn((
        PbrBundle {
            mesh: cube_mesh.clone(),
            material: materials.add(Color::rgb(0.9, 0.2, 0.2)),
            transform: Transform::from_xyz(-4.0, 0.5, 3.0),
            ..default()
        },
        Enemy,
        Selectable,
        PickableBundle::default(),
        ShapeType::Cube,
        Health { current: 80.0, max: 80.0 },
        CanShoot {
            cooldown: 1.2,
            last_shot: 0.0,
            range: 8.0,
            damage: 12.0,
        },
        Name::new("EnemyCube"),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));

    // enemy 2 - red sphere (infantry)
    commands.spawn((
        PbrBundle {
            mesh: sphere_mesh.clone(),
            material: materials.add(Color::rgb(0.9, 0.2, 0.2)),
            transform: Transform::from_xyz(4.0, 0.5, 3.0),
            ..default()
        },
        Enemy,
        Selectable,
        PickableBundle::default(),
        ShapeType::Infantry,
        Health { current: 60.0, max: 60.0 },
        CanShoot {
            cooldown: 0.8,
            last_shot: 0.0,
            range: 12.0,
            damage: 8.0,
        },
        Name::new("EnemyInfantry"),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));

    // Tower - spawn a tower with high HP
    spawn_tower(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, -5.0),
    );
    
    // Forest Farm
    info!("Creating initial inactive forest farm at position (-5.0, 0.0, -1.0)");
    spawn_inactive_forest_farm(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-5.0, 0.0, -1.0),
    );

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

/// Create a tower at the specified position
pub fn spawn_tower(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.5, 6.0, 1.5))),
            material: materials.add(Color::rgb(0.9, 0.2, 0.2)),
            transform: Transform::from_translation(position + Vec3::new(0.0, 3.0, 0.0)),
            ..default()
        },
        Tower { height: 6.0 },
        Selectable,
        PickableBundle::default(),
        ShapeType::Tower,
        EnemyTower,
        Health { current: 500.0, max: 500.0 },
        Name::new("Tower"),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}