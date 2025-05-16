use bevy::{prelude::*, input::mouse::MouseWheel};
use bevy::input::mouse::MouseMotion;

use crate::game::{MainCamera, CameraSettings, CameraMovementState, SelectedEntity};

/// system for handling mouse wheel scrolling and changing the camera zoom
pub fn camera_zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut camera_settings: ResMut<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    _selected_entity_res: Res<SelectedEntity>,
    _transform_query: Query<&Transform, Without<MainCamera>>,
    camera_movement_state: Res<CameraMovementState>,
) {
    let mut zoom_delta = 0.0;
    
    for event in mouse_wheel_events.read() {
        zoom_delta += event.y;
    }
    
    if zoom_delta != 0.0 {
        camera_settings.zoom_level -= zoom_delta * camera_settings.zoom_speed;
        
        camera_settings.zoom_level = camera_settings.zoom_level
            .clamp(camera_settings.min_zoom, camera_settings.max_zoom);
        
        if camera_movement_state.manual_camera_mode || camera_movement_state.is_right_button_pressed {
            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
                let forward = camera_transform.forward();
                let look_target = camera_transform.translation + forward * 10.0;
                
                let zoom_movement = forward * zoom_delta * camera_settings.zoom_speed * 5.0;
                camera_transform.translation += zoom_movement;
                
                camera_transform.look_at(look_target, Vec3::Y);
                
                info!("camera_zoom_system: Camera transform updated - pos: {:?}, forward: {:?}, zoom_level: {}", 
                       camera_transform.translation, camera_transform.forward(), camera_settings.zoom_level);
            }
        }
    }
}

/// system for updating the camera position to follow the selected object
pub fn camera_follow_selected(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    selected_entity_res: Res<SelectedEntity>,
    transform_query: Query<&Transform, Without<MainCamera>>,
    camera_settings: Res<CameraSettings>,
    camera_movement_state: Res<CameraMovementState>,
) {
    if camera_movement_state.is_right_button_pressed || camera_movement_state.manual_camera_mode {
        return;
    }
    
    if let Some(selected_entity) = selected_entity_res.0 {
        if let Ok(selected_transform) = transform_query.get(selected_entity) {
            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
    
                let base_offset = Vec3::new(-2.5, 4.5, 4.0);
                
                // apply scaling to the offset based on the zoom level
                let scaled_offset = base_offset * camera_settings.zoom_level;
                
                // set the camera position with the offset
                camera_transform.translation = selected_transform.translation + scaled_offset;
                
                // look at the selected object
                camera_transform.look_at(selected_transform.translation, Vec3::Y);
                
                info!("camera_follow_selected: Camera following entity - camera pos: {:?}, target pos: {:?}", 
                       camera_transform.translation, selected_transform.translation);
            }
        }
    }
}

/// system for controlling the camera with the right mouse button
pub fn camera_right_button_movement(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut motion_events: EventReader<MouseMotion>,
    mut camera_movement_state: ResMut<CameraMovementState>,
    _time: Res<Time>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        camera_movement_state.is_right_button_pressed = true;
        camera_movement_state.last_mouse_position = None;
        info!("camera_right_button_movement: Right mouse button pressed, entering camera movement mode");
    } else if mouse_buttons.just_released(MouseButton::Right) {
        camera_movement_state.is_right_button_pressed = false;
        camera_movement_state.last_mouse_position = None;
        camera_movement_state.manual_camera_mode = true;
        info!("camera_right_button_movement: Right mouse button released, entering manual camera mode");
    }

    if camera_movement_state.is_right_button_pressed {
        let mut camera_transform = if let Ok(transform) = camera_query.get_single_mut() {
            transform
        } else {
            return;
        };
        
        let mut movement = Vec2::ZERO;
        for event in motion_events.read() {
            movement += event.delta;
        }
        
        if movement != Vec2::ZERO {
            let right = camera_transform.right();
            let up = Vec3::Y;
            let forward = camera_transform.forward().reject_from(up).normalize();
            
            camera_transform.translation -= right * movement.x * camera_movement_state.movement_speed;
            camera_transform.translation += forward * movement.y * camera_movement_state.movement_speed;
            
            let look_dir = camera_transform.forward();
            let target = camera_transform.translation + look_dir * 10.0;
            camera_transform.look_at(target, Vec3::Y);
            
            info!("camera_right_button_movement: Camera moved - pos: {:?}, movement: {:?}", 
                   camera_transform.translation, movement);
        }
    }
}