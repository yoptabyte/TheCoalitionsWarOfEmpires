use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::{Ground, MainCamera, ShapeType, Health, HoveredOutline, Tower, EnemyTower};
use crate::game::farm::{spawn_active_forest_farm};
use crate::menu::main_menu::Faction;
use crate::game::units::{PlayerFaction, AIFaction};

/// setup initial scene.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    player_faction: Res<PlayerFaction>,
    ai_faction: Res<AIFaction>,
    // Add query to clean up any existing cameras
    existing_cameras: Query<Entity, With<Camera>>,
) {
    // Clean up any existing cameras to prevent conflicts
    for camera_entity in existing_cameras.iter() {
        if let Some(entity_commands) = commands.get_entity(camera_entity) {
            entity_commands.despawn_recursive();
            println!("🧹 Cleaned up existing camera: {:?}", camera_entity);
        }
    }

    // Forest Farm
    spawn_active_forest_farm(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-5.0, 0.0, -1.0),
        &asset_server,
    );


    let plane_mesh = meshes.add(Plane3d::default().mesh().size(120.0, 120.0));
    commands.spawn((
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        PickableBundle::default(),
        Ground,
    ));

    // Main point light - much brighter
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 15.0, 4.0),
        ..default()
    });

    // Additional ambient lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1500.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera - positioned to look at the battlefield center, not at Vec3::ZERO
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 15.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            camera: Camera {
                // 3D cameras render in background
                order: -1,
                clear_color: ClearColorConfig::Custom(Color::rgb(0.1, 0.2, 0.3)), // Dark blue sky
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));

    // Spawn player faction towers (closer to camera, at positive Z)
    spawn_faction_towers(
        &mut commands,
        &asset_server,
        player_faction.0,
        Vec3::new(-35.0, 0.0, 40.0), // Left tower - moved further
        Vec3::new(0.0, 0.0, 40.0),   // Center tower - moved further
        Vec3::new(35.0, 0.0, 40.0),  // Right tower - moved further
        false, // Not enemy towers
    );

    // Spawn AI faction towers (far from camera, at negative Z)  
    spawn_faction_towers(
        &mut commands,
        &asset_server,
        ai_faction.0,
        Vec3::new(-35.0, 0.0, -40.0), // Left tower - moved further
        Vec3::new(0.0, 0.0, -40.0),   // Center tower - moved further
        Vec3::new(35.0, 0.0, -40.0),  // Right tower - moved further
        true, // Enemy towers
    );
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
        PickableBundle::default(),
        ShapeType::Tower,
        EnemyTower,
        Health { current: 500.0, max: 500.0 },
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cuboid(1.5, 3.5, 1.5),
        bevy_rapier3d::prelude::Sensor,
        bevy_rapier3d::prelude::LockedAxes::all(),
        Name::new("Tower"),
        On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
            commands.entity(event.target).insert(HoveredOutline);
        }),
        On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
            commands.entity(event.target).remove::<HoveredOutline>();
        }),
    ));
}

/// Spawn faction-specific towers at given positions
pub fn spawn_faction_towers(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    faction: Faction,
    pos1: Vec3,
    pos2: Vec3, 
    pos3: Vec3,
    is_enemy: bool,
) {
    let (tower_models, tower_scales) = match faction {
        Faction::Entente => (
            vec![
                "models/entente/towers/big_ben.glb#Scene0",
                "models/entente/towers/eiffel.glb#Scene0", 
                "models/entente/towers/st_petersburg.glb#Scene0"
            ],
            vec![2.0, 0.4, 6.0] // big_ben , eiffel , st_petersburg 
        ),
        Faction::CentralPowers => (
            vec![
                "models/central_powers/towers/reichstag.glb#Scene0",
                "models/central_powers/towers/vienna.glb#Scene0",
                "models/central_powers/towers/istanbul.glb#Scene0"
            ],
            vec![0.2, 4.5, 4.5] // reichstag , vienna , istanbul 
        ),
    };

    let positions = vec![pos1, pos2, pos3];
    
    for (i, position) in positions.iter().enumerate() {
        let tower_scene = asset_server.load(tower_models[i]);
        let tower_scale = tower_scales[i];
        
        let tower_bundle = (
            SceneBundle {
                scene: tower_scene,
                transform: Transform::from_translation(*position + Vec3::new(0.0, 5.0, 0.0))
                    .with_scale(Vec3::splat(tower_scale)),
                ..default()
            },
            Tower { height: 10.0 },
            PickableBundle::default(),
            ShapeType::Tower,
            Health { current: 600.0, max: 600.0 },
            bevy_rapier3d::prelude::RigidBody::Fixed,
            bevy_rapier3d::prelude::Collider::cuboid(6.0, 8.0, 6.0), // Большой коллайдер для легкого клика по башням
            bevy_rapier3d::prelude::Sensor,
            bevy_rapier3d::prelude::LockedAxes::all(),
            Name::new(format!("FactionTower_{}", i)),
            On::<Pointer<Over>>::run(|mut commands: Commands, event: Listener<Pointer<Over>>| {
                commands.entity(event.target).insert(HoveredOutline);
            }),
            On::<Pointer<Out>>::run(|mut commands: Commands, event: Listener<Pointer<Out>>| {
                commands.entity(event.target).remove::<HoveredOutline>();
            }),
        );

        if is_enemy {
            commands.spawn((tower_bundle, EnemyTower));
        } else {
            commands.spawn(tower_bundle);
        }
    }
}