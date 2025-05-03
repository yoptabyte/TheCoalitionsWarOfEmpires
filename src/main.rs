use bevy::{prelude::*, window::PrimaryWindow};

/// Маркер для нашего управляемого куба.
#[derive(Component)]
struct ControllableCube;

/// Ресурс для хранения целевой точки перемещения.
#[derive(Resource, Default)]
struct TargetDestination(Option<Vec3>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<TargetDestination>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_mouse_clicks, move_cube_towards_target))
        .run();
}

/// Система для настройки сцены при запуске.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Куб
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        ControllableCube,
    ));

    // Плоскость (земля)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..default()
    });

    // Свет
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Камера
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

/// Система для обработки кликов мыши и обновления целевой точки.
fn handle_mouse_clicks(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut target_dest: ResMut<TargetDestination>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let Ok(primary_window) = window_query.get_single() else { return };
        let Some(cursor_position) = primary_window.cursor_position() else { return };
        let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

        // Создаем луч из камеры через позицию курсора
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };

        // Проверяем пересечение луча с плоскостью (простой вариант без физики)
        let plane_normal = Vec3::Y;
        let plane_origin = Vec3::ZERO;

        let denominator = ray.direction.dot(plane_normal);
        if denominator.abs() > 1e-6 { // Избегаем деления на ноль (луч параллелен плоскости)
            let t = (plane_origin - ray.origin).dot(plane_normal) / denominator;
            if t >= 0.0 { // Пересечение перед камерой
                 target_dest.0 = Some(ray.origin + ray.direction * t);
                 println!("New target: {:?}", target_dest.0); // Для отладки
            } else {
                 // Луч направлен от плоскости или параллелен ей, но начинается за ней
                 println!("Click ray does not intersect the plane positively.");
                 target_dest.0 = None; // Сбрасываем цель
            }
        } else {
            // Луч параллелен плоскости
            println!("Click ray is parallel to the plane.");
            target_dest.0 = None; // Сбрасываем цель
        }
    }
}

/// Система для перемещения куба к целевой точке.
fn move_cube_towards_target(
    mut cube_query: Query<&mut Transform, With<ControllableCube>>,
    target_dest: Res<TargetDestination>,
    time: Res<Time>,
) {
    if let Some(target) = target_dest.0 {
        let Ok(mut cube_transform) = cube_query.get_single_mut() else { return };
        let speed = 2.0; // Скорость движения куба
        let direction = target - cube_transform.translation;

        // Перемещаем куб, если он еще не достиг цели
        if direction.length_squared() > 0.01 { // Небольшой порог для остановки
            let movement = direction.normalize() * speed * time.delta_seconds();
            // Ограничиваем движение, чтобы не перелететь цель за один кадр
            if movement.length_squared() >= direction.length_squared() {
                 cube_transform.translation = target;
            } else {
                 cube_transform.translation += movement;
            }
        }
    }
}
