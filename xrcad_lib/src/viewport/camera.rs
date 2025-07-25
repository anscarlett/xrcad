
use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*,};
use crate::ui_font::UiFont;
// use crate::render::lighting::CustomLightController;
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


pub fn camera_control_and_ui_panel(
    mut text_query: Query<&mut Text, With<CameraPanelText>>, 
    mut camera_query: Query<(&mut Transform, &mut CustomCameraController, &Camera, &GlobalTransform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
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
    for (mut transform, mut controller, camera, cam_transform) in camera_query.iter_mut() {
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

        // Keyboard sensitivity controls
        if keyboard.just_pressed(KeyCode::KeyP) {
            controller.pan_sensitivity += 0.1;
        }
        if keyboard.just_pressed(KeyCode::KeyO) {
            controller.pan_sensitivity -= 0.1;
        }
        if keyboard.just_pressed(KeyCode::KeyT) {
            controller.rotate_sensitivity += 0.1;
        }
        if keyboard.just_pressed(KeyCode::KeyY) {
            controller.rotate_sensitivity -= 0.1;
        }
        if keyboard.just_pressed(KeyCode::KeyZ) {
            controller.zoom_sensitivity += 0.1;
        }
        if keyboard.just_pressed(KeyCode::KeyX) {
            controller.zoom_sensitivity -= 0.1;
        }
        if keyboard.just_pressed(KeyCode::F1) {
            controller.is_xr = !controller.is_xr;
        }
        if keyboard.just_pressed(KeyCode::F2) {
            controller.is_stereo = !controller.is_stereo;
        }

        // Arrow key camera rotation around origin (only when Shift or Control is NOT pressed)
        let rotation_speed: f32 = 90.0; // degrees per second
        let dt = time.delta_secs();
        let shift_pressed = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        let ctrl_pressed = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);
        let arrow_pressed = !shift_pressed && !ctrl_pressed && (keyboard.pressed(KeyCode::ArrowLeft)
            || keyboard.pressed(KeyCode::ArrowRight)
            || keyboard.pressed(KeyCode::ArrowUp)
            || keyboard.pressed(KeyCode::ArrowDown));
        if arrow_pressed {
            let current_pos = transform.translation;
            let distance = current_pos.length();
            let mut azimuth = current_pos.z.atan2(current_pos.x);
            let mut elevation = (current_pos.y / distance).asin();
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
            azimuth = azimuth % (2.0 * std::f32::consts::PI);
            if azimuth < 0.0 {
                azimuth += 2.0 * std::f32::consts::PI;
            }
            let max_elevation = 85.0_f32.to_radians();
            elevation = elevation.clamp(-max_elevation, max_elevation);
            let cos_elevation = elevation.cos();
            let new_position = Vec3::new(
                distance * cos_elevation * azimuth.cos(),
                distance * elevation.sin(),
                distance * cos_elevation * azimuth.sin(),
            );
            transform.translation = new_position;
            *transform = transform.looking_at(Vec3::ZERO, Vec3::Y);
        }
    }

    // Update UI text panel with camera info
    if let Some(mut text) = text_query.iter_mut().next() {
        if let Some((_, controller, _, _)) = camera_query.iter_mut().next() {
            let mut content = String::from("Camera Controls:\n");
            content.push_str(&format!("Pan Sensitivity: {:.2} (P/O)\n", controller.pan_sensitivity));
            content.push_str(&format!("Rotate Sensitivity: {:.2} (T/Y)\n", controller.rotate_sensitivity));
            content.push_str(&format!("Zoom Sensitivity: {:.2} (Z/X)\n", controller.zoom_sensitivity));
            content.push_str(&format!("XR Enabled: {} (F1)\n", controller.is_xr));
            content.push_str(&format!("Stereo Enabled: {} (F2)\n", controller.is_stereo));
            content.push_str("\nArrow Keys: Rotate camera around origin\n");
            content.push_str("↑↓: Elevation (±85°)\n");
            content.push_str("←→: Azimuth (360°)\n");
            text.0 = content;
        }
    }
}
#[derive(Component)]
pub struct CameraPanelText;

pub fn setup_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d {
            screen_space_specular_transmission_steps: 0,
            ..default()
        },
        Transform::from_xyz(-500.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        CustomCameraController::default(),
    ));
    // Camera panel (top right)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(20.0),
            height: Val::Percent(50.0),
            right: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..Default::default()
            },
        BackgroundColor(Color::srgba(0.15, 0.1, 0.15, 0.5)),
        // ControlsPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Camera Controls\n"),
            TextFont {
                font: asset_server.load("fonts/FiraCode-Light.ttf"),
                font_size: 10.0,
                ..default()
            },
            CameraPanelText,
        ));
    });
}