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

/// Marker for entities that should have a hover outline.
#[derive(Component)]
struct HoveredOutline;

/// Маркер для земли, чтобы отличать её от других объектов
#[derive(Component)]
struct Ground;

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

/// Компонент для хранения типа формы объекта
#[derive(Component)]
enum ShapeType {
    Cube,
    Sphere,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(
            DefaultPickingPlugins
                // Отключаем плагины, которые могут менять цвета при наведении и выборе
                .build()
                .disable::<DefaultHighlightingPlugin>()
                .disable::<DebugPickingPlugin>()
        )
        .init_resource::<ClickCircle>()
        .init_resource::<SelectedEntity>()
        .add_systems(Startup, (setup, setup_particle_effect))
        .add_systems(Update, (
            process_movement_orders,
            draw_click_circle,
            draw_movement_lines,
            select_entity_system.after(PickSet::Last),
            handle_ground_clicks.after(select_entity_system),
            draw_hover_outline,
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
        RaycastPickable::default(),
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
            info!("select_entity_system: Кликнули на выбираемый объект {:?}, ранее выбранный: {:?}", event.target, selected_entity.0);
            
            // Обновляем выбранный объект только если он изменился
            if selected_entity.0 != Some(event.target) {
                selected_entity.0 = Some(event.target);
            }
            
            return;
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
            info!("handle_ground_clicks: Отдаем приказ на движение для {:?} к точке {:?}", entity_to_move, target_point);
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
                info!("process_movement_orders: Объект {:?} достиг цели", entity);
            } else {
                transform.translation += movement_this_frame;
            }
        } else {
            transform.translation = target; 
            commands.entity(entity).remove::<MovementOrder>();
            info!("process_movement_orders: Объект {:?} достиг цели (close)", entity);
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

/// System to draw a yellow outline around hovered entities.
fn draw_hover_outline(
    mut gizmos: Gizmos,
    hovered_entities_query: Query<(&Transform, &ShapeType, Option<&Handle<Mesh>>), (With<HoveredOutline>, With<Selectable>)>,
    meshes: Res<Assets<Mesh>>,
) {
    for (transform, shape_type, mesh_handle) in hovered_entities_query.iter() {
        match shape_type {
            ShapeType::Cube => {
                // Отрисовка обводки для куба с использованием AABB
                if let Some(mesh_handle) = mesh_handle {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(aabb) = mesh.compute_aabb() {
                            // Get the corners of the AABB in local space
                            let min = aabb.min();
                            let max = aabb.max();
                            let corners = [
                                Vec3::new(min.x, min.y, min.z),
                                Vec3::new(max.x, min.y, min.z),
                                Vec3::new(max.x, max.y, min.z),
                                Vec3::new(min.x, max.y, min.z),
                                Vec3::new(min.x, min.y, max.z),
                                Vec3::new(max.x, min.y, max.z),
                                Vec3::new(max.x, max.y, max.z),
                                Vec3::new(min.x, max.y, max.z),
                            ];

                            // Transform corners to world space
                            let world_corners: Vec<Vec3> = corners
                                .iter()
                                .map(|&corner| transform.transform_point(corner))
                                .collect();

                            // Define edges of the cuboid based on corners
                            let edges = [
                                (world_corners[0], world_corners[1]), (world_corners[1], world_corners[2]),
                                (world_corners[2], world_corners[3]), (world_corners[3], world_corners[0]),
                                (world_corners[4], world_corners[5]), (world_corners[5], world_corners[6]),
                                (world_corners[6], world_corners[7]), (world_corners[7], world_corners[4]),
                                (world_corners[0], world_corners[4]), (world_corners[1], world_corners[5]),
                                (world_corners[2], world_corners[6]), (world_corners[3], world_corners[7]),
                            ];

                            for (start, end) in edges.iter() {
                                gizmos.line(*start, *end, Color::YELLOW);
                            }
                        }
                    }
                }
            },
            ShapeType::Sphere => {
                // Рисуем сферическую обводку для сферы
                let radius = 0.5; // Радиус сферы
                let world_position = transform.translation;
                
                // Рисуем три окружности в ортогональных плоскостях для создания эффекта сферы
                // Окружность в плоскости XZ (нормаль Y)
                gizmos.circle(world_position, Direction3d::Y, radius, Color::YELLOW);
                // Окружность в плоскости XY (нормаль Z)
                gizmos.circle(world_position, Direction3d::Z, radius, Color::YELLOW);
                // Окружность в плоскости YZ (нормаль X)
                gizmos.circle(world_position, Direction3d::X, radius, Color::YELLOW);
            }
        }
    }
}