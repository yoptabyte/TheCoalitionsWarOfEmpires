use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use crate::game::{Enemy, Health};

/// Компонент для связи клик-коллайдера с настоящим врагом
#[derive(Component)]
pub struct LinkedToEnemy(pub Entity);

/// Система для добавления невидимых клик-коллайдеров к AI юнитам
pub fn add_debug_click_colliders(
    mut commands: Commands,
    // Враги с Health компонентом (настоящие юниты), у которых еще нет коллайдера
    enemy_units_query: Query<(Entity, &Transform, &Health), (With<Enemy>, Without<HasClickCollider>)>,
    // Враги без Health (это уже коллайдеры) - игнорируем
    existing_colliders: Query<Entity, (With<Enemy>, Without<Health>)>,
) {
    for (enemy_entity, enemy_transform, _health) in enemy_units_query.iter() {
        // Создаем невидимый клик-коллайдер над врагом
        let collider_entity = commands.spawn((
            TransformBundle::from_transform(
                Transform::from_translation(enemy_transform.translation + Vec3::new(0.0, 1.5, 0.0))
            ),
            PickableBundle::default(),
            LinkedToEnemy(enemy_entity), // Связываем с настоящим врагом
            Name::new("Invisible Click Collider"),
        )).id();

        // Помечаем врага как имеющего коллайдер
        commands.entity(enemy_entity).insert(HasClickCollider(collider_entity));

        info!("Added invisible click collider {:?} for enemy {:?}", collider_entity, enemy_entity);
    }
}

/// Маркер что у врага уже есть клик-коллайдер
#[derive(Component)]
pub struct HasClickCollider(pub Entity);