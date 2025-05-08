use bevy::prelude::*;
use bevy_hanabi::prelude::*;

/// resource for storing the selected entity
#[derive(Resource, Default)]
pub struct SelectedEntity(pub Option<Entity>);

/// resource for storing information about the click circle for gizmos
#[derive(Resource, Default)]
pub struct ClickCircle {
    pub position: Option<Vec3>,
    pub spawn_time: Option<f32>,
}

/// resource for storing the handle for the particle effect asset
#[derive(Resource)]
pub struct ClickEffectHandle(pub Handle<EffectAsset>);

/// resource for storing the camera settings
#[derive(Resource)]
pub struct CameraSettings {
    pub zoom_level: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
}

/// resource for storing the camera movement state
#[derive(Resource)]
pub struct CameraMovementState {
    pub is_right_button_pressed: bool,
    pub last_mouse_position: Option<Vec2>,
    pub movement_speed: f32,
    pub manual_camera_mode: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            zoom_level: 1.0,
            min_zoom: 0.5,
            max_zoom: 10.0,
            zoom_speed: 0.1,
        }
    }
}

impl Default for CameraMovementState {
    fn default() -> Self {
        Self {
            is_right_button_pressed: false,
            last_mouse_position: None,
            movement_speed: 0.02,
            manual_camera_mode: false,
        }
    }
}