// Moved from xrcad_app/src/camera_control.rs
use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};
use crate::render::lighting::LightController;
#[derive(Component)]
#[derive(Resource)]
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

#[derive(Component)]
pub struct CameraPanelText;

// Camera UI panel system (Bevy UI only)
pub fn camera_ui_panel(
    mut ui_state: ResMut<CustomCameraController>,
    mut text_query: Query<&mut Text, With<CameraPanelText>>,
    mut camera_query: Query<(&mut Transform, &mut CustomCameraController)>,
    light_query: Query<&Transform, (With<DirectionalLight>, With<LightController>, Without<CustomCameraController>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Adjust camera parameters with keys (Bevy 0.13+ KeyCode)
    if keyboard.just_pressed(KeyCode::KeyP) {
        ui_state.pan_sensitivity += 0.1;
    }
    if keyboard.just_pressed(KeyCode::KeyO) {
        ui_state.pan_sensitivity -= 0.1;
    }
    if keyboard.just_pressed(KeyCode::KeyT) {
        ui_state.rotate_sensitivity += 0.1;
    }
    if keyboard.just_pressed(KeyCode::KeyY) {
        ui_state.rotate_sensitivity -= 0.1;
    }
    if keyboard.just_pressed(KeyCode::KeyZ) {
        ui_state.zoom_sensitivity += 0.1;
    }
    if keyboard.just_pressed(KeyCode::KeyX) {
        ui_state.zoom_sensitivity -= 0.1;
    }
    if keyboard.just_pressed(KeyCode::F1) {
        ui_state.is_xr = !ui_state.is_xr;
    }
    if keyboard.just_pressed(KeyCode::F2) {
        ui_state.is_stereo = !ui_state.is_stereo;
    }

    // Arrow key camera rotation around origin (only when Shift is NOT pressed)
    if let Ok((mut camera_transform, mut camera_controller)) = camera_query.single_mut() {
        let rotation_speed: f32 = 90.0; // degrees per second
        let dt = time.delta_secs();
        
        let shift_pressed = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        
        // Check if any arrow keys are pressed (but not with Shift)
        let arrow_pressed = !shift_pressed && (keyboard.pressed(KeyCode::ArrowLeft) || 
                           keyboard.pressed(KeyCode::ArrowRight) || 
                           keyboard.pressed(KeyCode::ArrowUp) || 
                           keyboard.pressed(KeyCode::ArrowDown));
        
        if arrow_pressed {
            // Get current camera position relative to origin
            let current_pos = camera_transform.translation;
            let distance = current_pos.length();
            
            // Convert current position to spherical coordinates
            let mut azimuth = current_pos.z.atan2(current_pos.x); // angle around Y axis
            let mut elevation = (current_pos.y / distance).asin(); // angle from XZ plane
            
            // Apply arrow key rotations
            if keyboard.pressed(KeyCode::ArrowLeft) {
                azimuth += rotation_speed.to_radians() * dt;
            }
            if keyboard.pressed(KeyCode::ArrowRight) {
                azimuth -= rotation_speed.to_radians() * dt;
            }
            if keyboard.pressed(KeyCode::ArrowUp) {
                elevation += rotation_speed.to_radians() * dt;
            }
            if keyboard.pressed(KeyCode::ArrowDown) {
                elevation -= rotation_speed.to_radians() * dt;
            }
            
            // Wrap azimuth around (0 to 2π)
            azimuth = azimuth % (2.0 * std::f32::consts::PI);
            if azimuth < 0.0 {
                azimuth += 2.0 * std::f32::consts::PI;
            }
            
            // Clamp elevation to ±85 degrees
            let max_elevation = 85.0_f32.to_radians();
            elevation = elevation.clamp(-max_elevation, max_elevation);
            
            // Convert back to Cartesian coordinates
            let cos_elevation = elevation.cos();
            let new_position = Vec3::new(
                distance * cos_elevation * azimuth.cos(),
                distance * elevation.sin(),
                distance * cos_elevation * azimuth.sin(),
            );
            
            // Update camera position and make it look at origin
            camera_transform.translation = new_position;
            *camera_transform = camera_transform.looking_at(Vec3::ZERO, Vec3::Y);
        }

        // Update camera controller with new sensitivities
        camera_controller.pan_sensitivity = ui_state.pan_sensitivity;
        camera_controller.rotate_sensitivity = ui_state.rotate_sensitivity;
        camera_controller.zoom_sensitivity = ui_state.zoom_sensitivity;
        camera_controller.is_xr = ui_state.is_xr;
        camera_controller.is_stereo = ui_state.is_stereo;
    }

    // Update UI text panel with camera and light info
    if let Some(mut text) = text_query.iter_mut().next() {
        let mut content = String::from("Camera Controls:\n");
        content.push_str(&format!("Pan Sensitivity: {:.2} (P/O)\n", ui_state.pan_sensitivity));
        content.push_str(&format!("Rotate Sensitivity: {:.2} (T/Y)\n", ui_state.rotate_sensitivity));
        content.push_str(&format!("Zoom Sensitivity: {:.2} (Z/X)\n", ui_state.zoom_sensitivity));
        content.push_str(&format!("XR Enabled: {} (F1)\n", ui_state.is_xr));
        content.push_str(&format!("Stereo Enabled: {} (F2)\n", ui_state.is_stereo));
        content.push_str("\nArrow Keys: Rotate camera around origin\n");
        content.push_str("↑↓: Elevation (±85°)\n");
        content.push_str("←→: Azimuth (360°)\n");
        content.push_str("\nLight Controls:\n");
        content.push_str("Shift+Arrow Keys: Move light X/Z\n");
        content.push_str("PageUp/PageDown: Move light Y\n");
        
        // Show light position if available
        if let Ok(light_transform) = light_query.single() {
            let pos = light_transform.translation;
            content.push_str(&format!("Light Position: ({:.0}, {:.0}, {:.0})\n", pos.x, pos.y, pos.z));
        }
        
        text.0 = content;
    }
}

pub fn setup_camera_ui(mut commands: Commands) {
    // Camera panel (top right)
    commands.spawn((
        Node::default(),
        BackgroundColor(Color::srgb(0.15, 0.1, 0.1)),
        // ControlsPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Camera Controls\n"),
            CameraPanelText,
        ));
    });
}