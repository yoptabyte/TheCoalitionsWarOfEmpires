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
#[derive(Component, Debug, Clone, Copy)]
pub enum ShapeType {
    Cube,
    Infantry,
    Airplane,
    Tower,
    Farm,
    Mine,
    SteelFactory,
    PetrochemicalPlant,
    Trench,
}

/// marker for enemies
#[derive(Component)]
pub struct Enemy;

/// marker for towers
#[derive(Component)]
pub struct Tower {
    #[allow(dead_code)]
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

/// marker for petrochemical plant structure
#[derive(Component)]
pub struct PetrochemicalPlant;

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

/// component for petrochemical plant oil production rate per second
#[derive(Component)]
pub struct OilProductionRate(pub f32);

/// marker for the trench
#[derive(Component)]
pub struct Trench;

/// component to track trench construction time
#[derive(Component)]
pub struct TrenchConstruction {
    pub time_remaining: f32,
    pub total_construction_time: f32,
}

/// resource for trench cost
#[derive(Resource, Clone)]
pub struct TrenchCost {
    pub wood: u32,
    pub money: u32,
}

impl Default for TrenchCost {
    fn default() -> Self {
        Self {
            wood: 3,
            money: 3,
        }
    }
}