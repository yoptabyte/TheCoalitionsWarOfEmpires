use bevy::prelude::*;

/// marker for the controllable tank
#[derive(Component)]
pub struct Tank;

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
    Farm,
    Mine,
    SteelFactory,
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

/// general marker for farm structures
#[derive(Component)]
pub struct Farm;

/// marker for forest farm type
#[derive(Component)]
pub struct ForestFarm;

/// marker for mine structure
#[derive(Component)]
pub struct Mine;

/// marker for steel factory structure
#[derive(Component)]
pub struct SteelFactory;

/// component indicating if the farm is active
#[derive(Component)]
pub struct FarmActive(pub bool);

/// component for farm income rate per second
#[derive(Component)]
pub struct FarmIncomeRate(pub f32);

/// component for mine iron production rate per second
#[derive(Component)]
pub struct MineIronRate(pub f32);

/// component for steel factory steel production rate per second
#[derive(Component)]
pub struct SteelProductionRate(pub f32);