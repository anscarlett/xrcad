// Moved from xrcad_app/src/camera_control.rs
use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};

#[derive(Component)]
pub struct CustomCameraController {
    pub pan_sensitivity: f32,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub is_xr: bool,
    pub is_stereo: bool,
}

impl Default for CustomCameraController {
    fn default() -> Self {
        Self {
            pan_sensitivity: 1.0,
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 1.0,
            is_xr: false,
            is_stereo: false,
        }
    }
}

pub fn camera_control_system(
    mut query: Query<(&mut Transform, &mut CustomCameraController, &Camera, &GlobalTransform)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll_evr: EventReader<MouseWheel>,
    windows: Query<&Window>,
) {
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => return,
    };
    let mouse_pos = window.cursor_position();
    let mut delta = Vec2::ZERO;
    for ev in mouse_motion_events.read() {
        delta += ev.delta;
    }
    for (mut transform, controller, camera, cam_transform) in query.iter_mut() {
        // Pan (MMB or Shift+LMB)
        if mouse_button.pressed(MouseButton::Middle)
            || (mouse_button.pressed(MouseButton::Left) && keys.pressed(KeyCode::ShiftLeft))
        {
            let right = transform.rotation * Vec3::X;
            let up = transform.rotation * Vec3::Y;
            transform.translation -= right * delta.x * 0.5 * controller.pan_sensitivity;
            transform.translation += up * delta.y * 0.5 * controller.pan_sensitivity;
        }
        // Orbit (LMB)
        else if mouse_button.pressed(MouseButton::Left) {
            let yaw = -delta.x * 0.01 * controller.rotate_sensitivity;
            let pitch = -delta.y * 0.01 * controller.rotate_sensitivity;
            transform.rotate_y(yaw);
            transform.rotate_local_x(pitch);
        }
        // Zoom (scroll)
        for ev in scroll_evr.read() {
            let zoom_dir = if let Some(mouse_pos) = mouse_pos {
                if let Ok(ray) = camera.viewport_to_world(cam_transform, mouse_pos) {
                    ray.direction
                } else {
                    transform.forward()
                }
            } else {
                transform.forward()
            };
            transform.translation += zoom_dir * ev.y * controller.zoom_sensitivity * 5.0;
        }
        // XR stub: if is_xr, you could override transform with XR pose here
        if controller.is_xr {
            // XR device pose integration stub
        }
        // Stereo stub: if is_stereo, you could adjust camera projection here
        if controller.is_stereo {
            // Stereo rendering integration stub
        }
    }
}
