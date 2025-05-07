use bevy::{prelude::*};
use bevy::gizmos::gizmos::Gizmos;
use bevy_hanabi::prelude::*;

use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::picking_core::PickSet;
use bevy_mod_picking::backends::raycast::RaycastPickable;

/// Marker for the controllable cube.
#[derive(Component)]
struct ControllableCube;

/// Marker for the selectable entities.
#[derive(Component)]
struct Selectable;

/// Marker for the highlighted entities.
#[derive(Component)]
struct Highlighted;

/// Маркер для земли, чтобы отличать её от других объектов
#[derive(Component)]
struct Ground;

/// Marker for the selected entity to visually highlight it
#[derive(Component)]
struct Selected;

/// Resource to store the selected entity.
#[derive(Resource, Default)]
struct SelectedEntity(Option<Entity>);

/// Resource to store click circle information for gizmos
#[derive(Resource, Default)]
struct ClickCircle {
    position: Option<Vec3>,
    spawn_time: Option<f32>,
}

/// Component to store individual movement order for an entity.
#[derive(Component)]
struct MovementOrder(Vec3);

// Resource to store the handle for our particle effect asset.
#[derive(Resource)]
struct ClickEffectHandle(Handle<EffectAsset>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(DefaultPickingPlugins)
        .init_resource::<ClickCircle>()
        .init_resource::<SelectedEntity>()
        .add_systems(Startup, (setup, setup_particle_effect))
        .add_systems(Update, (
            apply_highlight,
            update_selected_entities,
            process_movement_orders,
            draw_click_circle,
            draw_movement_lines,
            select_entity_system.after(PickSet::Last),
            handle_ground_clicks.after(select_entity_system),
        ))
        .run();
}

/// Set up the initial scene.
fn setup(
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
        RaycastPickable::default(),
        On::<Pointer<Over>>::run(handle_pointer_over),
        On::<Pointer<Out>>::run(handle_pointer_out),
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
        RaycastPickable::default(),
        On::<Pointer<Over>>::run(handle_pointer_over),
        On::<Pointer<Out>>::run(handle_pointer_out),
    ));

    let plane_mesh = meshes.add(Plane3d::default().mesh().size(25.0, 25.0));
    commands.spawn((
        PbrBundle {
            mesh: plane_mesh.clone(),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickable::default(),
        Ground,
    ));

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
    mut click_events: EventReader<Pointer<Click>>,
    mut selected_entity: ResMut<SelectedEntity>,
    query_selectable: Query<(), With<Selectable>>,
) {
    for event in click_events.read() {
        if query_selectable.get(event.target).is_ok() {
            info!("select_entity_system: clicked on selectable {:?}, previously selected: {:?}", event.target, selected_entity.0);
            
            if selected_entity.0 != Some(event.target) {
                selected_entity.0 = Some(event.target);
            }
            
            return;
        }
    }
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

fn update_selected_entities(
    selected_entity: Res<SelectedEntity>,
    mut commands: Commands,
    selectable_entities: Query<Entity, With<Selectable>>,
) {
    if selected_entity.is_changed() {
        for entity in selectable_entities.iter() {
            commands.entity(entity).remove::<Selected>();
        }
        
        if let Some(entity) = selected_entity.0 {
            commands.entity(entity).insert(Selected);
        }
    }
}

fn apply_highlight(
    highlighted_query: Query<(Entity, &Handle<StandardMaterial>), (With<Highlighted>, With<Selectable>)>,
    selected_query: Query<(Entity, &Handle<StandardMaterial>), (With<Selected>, With<Selectable>)>,
    mut other_query: Query<(Entity, &Handle<StandardMaterial>), (Without<Highlighted>, Without<Selected>, With<Selectable>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (_entity, material_handle) in highlighted_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.emissive = Color::YELLOW * 2.0;
        }
    }
    
    for (_entity, material_handle) in selected_query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.emissive = Color::GREEN * 3.0;
        }
    }

    for (_entity, material_handle) in other_query.iter_mut() {
         if let Some(material) = materials.get_mut(material_handle) {
             if material.emissive != Color::BLACK {
                 material.emissive = Color::BLACK;
             }
         }
    }
}

fn handle_ground_clicks(
    mut commands: Commands,
    mut click_events: EventReader<Pointer<Click>>,
    query_selectable: Query<(), With<Selectable>>,
    query_ground: Query<(), With<Ground>>,
    mut click_circle: ResMut<ClickCircle>,
    time: Res<Time>,
    click_effect_handle: Res<ClickEffectHandle>,
    selected_entity_res: Res<SelectedEntity>,
) {
    let mut clicked_on_selectable = false;
    let mut clicked_on_ground = false;
    let mut ground_click_position: Option<Vec3> = None;
    
    for event in click_events.read() {
        if query_selectable.get(event.target).is_ok() {
            clicked_on_selectable = true;
            info!("handle_ground_clicks: clicked on selectable {:?}", event.target);
        }
        
        if query_ground.get(event.target).is_ok() {
            clicked_on_ground = true;
            if let Some(position) = event.hit.position {
                ground_click_position = Some(position);
                info!("handle_ground_clicks: clicked on ground {:?}", position);
            }
        }
    }
    
    if !clicked_on_selectable && clicked_on_ground && selected_entity_res.0.is_some() && ground_click_position.is_some() {
        let target_point = ground_click_position.unwrap();
        
        if let Some(entity_to_move) = selected_entity_res.0 {
            info!("handle_ground_clicks: giving movement order to {:?} to point {:?}", entity_to_move, target_point);
            commands.entity(entity_to_move).insert(MovementOrder(target_point));
        }
        
        click_circle.position = Some(target_point);
        click_circle.spawn_time = Some(time.elapsed_seconds());
            Name::new("click_particles"),
            ParticleEffectBundle {
                effect: ParticleEffect::new(click_effect_handle.0.clone()),
                transform: Transform::from_translation(target_point),
                ..default()
            },
        ));
    } else if clicked_on_selectable {
        click_circle.position = None;
    }
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
            let xz_direction = Vec3::new(direction.x, 0.0, direction.z).normalize();
            
            if xz_direction.length_squared() > 0.001 {
                let target_rotation = Quat::from_rotation_arc(Vec3::Z, xz_direction);
                transform.rotation = transform.rotation.slerp(target_rotation, 0.2);
            }

            if movement_this_frame.length_squared() >= direction.length_squared() {
                transform.translation = target; 
                commands.entity(entity).remove::<MovementOrder>(); 
                info!("process_movement_orders: object {:?} reached destination", entity);
            } else {
                transform.translation += movement_this_frame;
            }
        } else {
            transform.translation = target; 
            commands.entity(entity).remove::<MovementOrder>();
            info!("process_movement_orders: object {:?} reached destination (close)", entity);
        }
    }
}

fn draw_movement_lines(
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