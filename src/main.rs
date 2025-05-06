use bevy::{prelude::*, window::PrimaryWindow};
use bevy::gizmos::gizmos::Gizmos;
use bevy_hanabi::prelude::*;

use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::picking_core::PickSet;

/// Marker for the controllable cube.
#[derive(Component)]
struct ControllableCube;

/// Marker for the selectable entities.
#[derive(Component)]
struct Selectable;

/// Marker for the highlighted entities.
#[derive(Component)]
struct Highlighted;

/// Resource to store the selected entity.
#[derive(Resource, Default)]
struct SelectedEntity(Option<Entity>);

/// Resource to store the object clicked flag.
#[derive(Resource, Default)]
struct WasObjectClicked(bool);

/// Component to store individual movement order for an entity.
#[derive(Component)]
struct MovementOrder(Vec3);

// Resource to store the handle for our particle effect asset.
#[derive(Resource)]
struct ClickEffectHandle(Handle<EffectAsset>);

// Resource to store click circle information for gizmos
#[derive(Resource, Default)]
struct ClickCircle {
    position: Option<Vec3>,
    spawn_time: Option<f32>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .init_resource::<ClickCircle>()
        .init_resource::<SelectedEntity>()
        .init_resource::<WasObjectClicked>()
        .add_systems(Startup, (setup, setup_particle_effect))
        .add_systems(Update, (
            apply_highlight,
            handle_ground_clicks.after(PickSet::Last),
            process_movement_orders,
            draw_click_circle,
        ))
        .add_systems(Last, reset_click_flag)
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
        Selectable,
        PickableBundle::default(),
        On::<Pointer<Over>>::run(handle_pointer_over),
        On::<Pointer<Out>>::run(handle_pointer_out),
        On::<Pointer<Click>>::run(select_entity_system),
    ));

    // Sphere
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::new(0.5))),
            material: materials.add(Color::rgb(0.2, 0.4, 0.8)),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..default()
        },
        Selectable,
        PickableBundle::default(),
        On::<Pointer<Over>>::run(handle_pointer_over),
        On::<Pointer<Out>>::run(handle_pointer_out),
        On::<Pointer<Click>>::run(select_entity_system),
    ));

    // Plane (ground)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(25.0, 25.0)),
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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));
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

/// System to draw a fading circle gizmo at the last click position.
const CIRCLE_LIFETIME: f32 = 0.5; // How long the circle lasts in seconds
const CIRCLE_COLOR: Color = Color::YELLOW; // Matches particle start color
const CIRCLE_FINAL_RADIUS: f32 = 0.2; // The final radius the circle expands to

fn draw_click_circle(
    mut gizmos: Gizmos,
    click_circle: Res<ClickCircle>,
    time: Res<Time>,
) {
    if let (Some(pos), Some(spawn_time)) = (click_circle.position, click_circle.spawn_time) {
        let elapsed = time.elapsed_seconds() - spawn_time;
        if elapsed < CIRCLE_LIFETIME {
            let progress = elapsed / CIRCLE_LIFETIME; // Progress from 0.0 to 1.0
            let alpha = 1.0 - progress; // Fade out alpha
            let current_radius = CIRCLE_FINAL_RADIUS * progress; // Expand radius

            let color = CIRCLE_COLOR.with_a(alpha);
            // Draw circle on the XZ plane (Y is up)
            gizmos.circle(
                pos + Vec3::Y * 0.01, // Slightly above the ground to avoid z-fighting
                Direction3d::Y,      // Normal to the circle is up (aligned with Y-axis)
                current_radius,      // Use the expanding radius
                color,
            );
        }
    }
}

fn select_entity_system(
    event: Listener<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut object_clicked_flag: ResMut<WasObjectClicked>,
    mut click_circle: ResMut<ClickCircle>,
) {
    selected_entity.0 = Some(event.target);
    object_clicked_flag.0 = true;
    click_circle.position = None;
}

fn handle_pointer_over(
    listener: Listener<Pointer<Over>>,
    mut commands: Commands,
) {
    commands.entity(listener.target).insert(Highlighted);
}

fn handle_pointer_out(
    listener: Listener<Pointer<Out>>,
    mut commands: Commands,
) {
     commands.entity(listener.target).remove::<Highlighted>();
}

fn apply_highlight(
    highlighted_query: Query<(Entity, &Handle<StandardMaterial>), (With<Highlighted>, With<Selectable>)>,
    mut unhighlighted_query: Query<(Entity, &Handle<StandardMaterial>), (Without<Highlighted>, With<Selectable>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (_entity, material_handle) in highlighted_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.emissive = Color::YELLOW * 2.0;
        }
    }

    for (_entity, material_handle) in unhighlighted_query.iter_mut() {
         if let Some(material) = materials.get_mut(material_handle) {
             if material.emissive != Color::BLACK {
                 material.emissive = Color::BLACK;
             }
         }
    }
}

fn handle_ground_clicks(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut click_circle: ResMut<ClickCircle>,
    time: Res<Time>,
    click_effect_handle: Res<ClickEffectHandle>,
    object_clicked_flag: Res<WasObjectClicked>,
    selected_entity_res: Res<SelectedEntity>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) && !object_clicked_flag.0 {
        if selected_entity_res.0.is_none() { 
            click_circle.position = None; 
            return;
        }

        let Ok(primary_window) = window_query.get_single() else { return };
        let Some(cursor_position) = primary_window.cursor_position() else { return };
        let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };

        let plane_normal = Vec3::Y;
        let plane_origin = Vec3::ZERO;

        let denominator = ray.direction.dot(plane_normal);
        if denominator.abs() > 1e-6 {
            let t = (plane_origin - ray.origin).dot(plane_normal) / denominator;
            if t >= 0.0 {
                 let target_point = ray.origin + ray.direction * t;

                 if let Some(entity_to_move) = selected_entity_res.0 {
                     commands.entity(entity_to_move).insert(MovementOrder(target_point));
                 }

                 click_circle.position = Some(target_point);
                 click_circle.spawn_time = Some(time.elapsed_seconds());

                 commands.spawn((
                     Name::new("click_particles"),
                     ParticleEffectBundle {
                         effect: ParticleEffect::new(click_effect_handle.0.clone()),
                         transform: Transform::from_translation(target_point),
                         ..default()
                     },
                 ));
            } else {
                 click_circle.position = None;
            }
        } else {
            click_circle.position = None;
        }
    }
}


fn reset_click_flag(mut object_clicked_flag: ResMut<WasObjectClicked>) {
    object_clicked_flag.0 = false;
}


fn process_movement_orders(
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

            if movement_this_frame.length_squared() >= direction.length_squared() {
                transform.translation = target; 
                commands.entity(entity).remove::<MovementOrder>(); 
            } else {
                transform.translation += movement_this_frame;
            }
        } else {
            transform.translation = target; 
            commands.entity(entity).remove::<MovementOrder>();
        }
    }
}