use bevy::{prelude::*, window::PrimaryWindow};
use bevy_hanabi::prelude::*;

/// Marker for the controllable cube.
#[derive(Component)]
struct ControllableCube;

/// Resource to store the movement target destination.
#[derive(Resource, Default)]
struct TargetDestination(Option<Vec3>);

// Resource to store the handle for our particle effect asset.
#[derive(Resource)]
struct ClickEffectHandle(Handle<EffectAsset>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin) 
        .init_resource::<TargetDestination>()
        // .init_resource::<ClickEffectHandle>() 
        .add_systems(Startup, (setup, setup_particle_effect)) 
        .add_systems(Update, (handle_mouse_clicks, move_cube_towards_target))
        .run();
}

/// Set up the initial scene.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ControllableCube,
    ));

    // Plane (ground)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Create and store the particle effect asset.
fn setup_particle_effect(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut color_gradient = Gradient::new();
    color_gradient.add_key(0.0, Vec4::new(1.0, 1.0, 0.0, 1.0)); // Yellow start
    color_gradient.add_key(0.5, Vec4::new(1.0, 0.5, 0.0, 1.0)); // Orange middle
    color_gradient.add_key(1.0, Vec4::new(1.0, 0.0, 0.0, 0.0)); // Red and transparent end

    let mut size_gradient = Gradient::new();
    size_gradient.add_key(0.0, Vec2::splat(0.1));
    size_gradient.add_key(0.8, Vec2::splat(0.05));
    size_gradient.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.1).expr(), // Small spawn radius
        dimension: ShapeDimension::Volume,
    };

    // Make particles shoot generally upwards
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::NEG_Y * 0.5).expr(), // Center *below* spawn point
        speed: writer.lit(1.0).uniform(writer.lit(2.0)).expr(),
    };

    let lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(0.6).uniform(writer.lit(1.0)).expr(), // Random lifetime
    );

    let effect = effects.add(
        EffectAsset::new(
            32, // Effect capacity
            Spawner::once(30.0.into(), true), // Spawn 30 particles once
            writer.finish(),
        )
        .with_name("click_effect")
        .init(init_pos)
        .init(init_vel)
        .init(lifetime)
        .render(ColorOverLifetimeModifier { gradient: color_gradient })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient,
            screen_space_size: false, // Size is in world units
        }),
    );

    commands.insert_resource(ClickEffectHandle(effect));
}

/// Handle mouse clicks to set the target destination and spawn particles.
fn handle_mouse_clicks(
    mut commands: Commands, // Need commands to spawn the effect
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut target_dest: ResMut<TargetDestination>,
    click_effect_handle: Res<ClickEffectHandle>, // Get the effect handle resource
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let Ok(primary_window) = window_query.get_single() else { return };
        let Some(cursor_position) = primary_window.cursor_position() else { return };
        let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

        // Create a ray from the camera through the cursor position
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };

        // Check ray intersection with the plane (simple version without physics)
        let plane_normal = Vec3::Y;
        let plane_origin = Vec3::ZERO;

        let denominator = ray.direction.dot(plane_normal);
        if denominator.abs() > 1e-6 { // Avoid division by zero (ray is parallel to plane)
            let t = (plane_origin - ray.origin).dot(plane_normal) / denominator;
            if t >= 0.0 { // intersection in front of camera
                 let target_point = ray.origin + ray.direction * t;
                 target_dest.0 = Some(target_point);
                 // println!("New target: {:?}", target_dest.0); 

                 commands.spawn((
                     Name::new("click_particles"),
                     ParticleEffectBundle {
                         effect: ParticleEffect::new(click_effect_handle.0.clone()),
                         transform: Transform::from_translation(target_point),
                         ..default()
                     },
                 ));

            } else {
                 // println!("Click ray does not intersect the plane positively.");
                 target_dest.0 = None; 
            }
        } else {
            // println!("Click ray is parallel to the plane.");
            target_dest.0 = None; 
        }
    }
}

/// Move the cube towards the target destination.
fn move_cube_towards_target(
    mut cube_query: Query<&mut Transform, With<ControllableCube>>,
    target_dest: Res<TargetDestination>,
    time: Res<Time>,
) {
    if let Some(target) = target_dest.0 {
        let Ok(mut cube_transform) = cube_query.get_single_mut() else { return };
        let speed = 2.0;
        let direction = target - cube_transform.translation;

        if direction.length_squared() > 0.01 { // Small threshold for stopping
            let movement = direction.normalize() * speed * time.delta_seconds();
            
            if movement.length_squared() >= direction.length_squared() {
                 cube_transform.translation = target;
            } else {
                 cube_transform.translation += movement;
            }
        }
    }
}