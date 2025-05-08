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
}