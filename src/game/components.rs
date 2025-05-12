use bevy::prelude::*;

/// marker for the controllable cube
#[derive(Component)]
pub struct ControllableCube;

/// marker for selectable entities
#[derive(Component)]
pub struct Selectable;

/// marker for entities that should have an outline when hovered
#[derive(Component)]
pub struct HoveredOutline;

/// marker for the ground to distinguish it from other objects
#[derive(Component)]
pub struct Ground;

/// marker for the main camera
#[derive(Component)]
pub struct MainCamera;

/// component for storing an individual movement order for an entity
#[derive(Component)]
pub struct MovementOrder(pub Vec3);

/// component for storing the shape type of an object
#[derive(Component)]
pub enum ShapeType {
    Cube,
    Sphere,
    Airplane,
    Tower,
}

/// marker for enemies
#[derive(Component)]
pub struct Enemy;

/// marker for towers
#[derive(Component)]
pub struct Tower {
    pub height: f32,
}

/// marker for enemy towers that can be attacked
#[derive(Component)]
pub struct EnemyTower;

/// health component for objects
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

/// component for managing shooting
#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub speed: f32,
    pub damage: f32,
}

/// component for indicating the ability to shoot
#[derive(Component)]
pub struct CanShoot {
    pub cooldown: f32,
    pub last_shot: f32,
    pub range: f32,
    pub damage: f32,
}

/// marker for aircraft
#[derive(Component)]
pub struct Aircraft {
    pub height: f32,
    pub speed: f32,
}