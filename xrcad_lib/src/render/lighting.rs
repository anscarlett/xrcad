use bevy::prelude::*;
// Light control marker component
#[derive(Component)]
pub struct LightController{
    // Marker component for light control
    pub enabled: bool,
    pub move_speed: f32,
}

impl Default for LightController {
    fn default() -> Self {
        Self {
            enabled: true,
            move_speed: 200.0, // units per second
        }
    }
}

// Light control system
pub fn light_control_system(
    mut light_query: Query<&mut Transform, (With<DirectionalLight>, With<LightController>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut light_transform) = light_query.single_mut() {
        let move_speed = 200.0; // units per second
        let dt = time.delta_secs();
        
        // Check for Shift+Arrow keys and PageUp/PageDown
        let shift_pressed = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        
        if shift_pressed {
            let mut movement = Vec3::ZERO;
            
            if keyboard.pressed(KeyCode::ArrowLeft) {
                movement.x -= move_speed * dt;
            }
            if keyboard.pressed(KeyCode::ArrowRight) {
                movement.x += move_speed * dt;
            }
            if keyboard.pressed(KeyCode::ArrowUp) {
                movement.z -= move_speed * dt;
            }
            if keyboard.pressed(KeyCode::ArrowDown) {
                movement.z += move_speed * dt;
            }
            if keyboard.pressed(KeyCode::PageUp) {
                movement.y += move_speed * dt;
            }
            if keyboard.pressed(KeyCode::PageDown) {
                movement.y -= move_speed * dt;
            }
            
            if movement != Vec3::ZERO {
                light_transform.translation += movement;
                // Keep the light looking at origin
                *light_transform = light_transform.looking_at(Vec3::ZERO, Vec3::Y);
            }
        }
    }
}

// System to render light position axes using gizmos
pub fn render_light_axes(
    mut gizmos: Gizmos,
    light_query: Query<&Transform, (With<DirectionalLight>, With<LightController>)>,
) {
    if let Ok(light_transform) = light_query.single() {
        let position = light_transform.translation;
        let axis_length = 100.0;
        
        // Draw coordinate axes at light position
        // X-axis (Red)
        gizmos.line(
            position,
            position + Vec3::X * axis_length,
            Color::srgb(1.0, 0.0, 0.0),
        );
        
        // Y-axis (Green)
        gizmos.line(
            position,
            position + Vec3::Y * axis_length,
            Color::srgb(0.0, 1.0, 0.0),
        );
        
        // Z-axis (Blue)
        gizmos.line(
            position,
            position + Vec3::Z * axis_length,
            Color::srgb(0.0, 0.0, 1.0),
        );
        
        // Draw a small sphere at the light position
        gizmos.sphere(position, 20.0, Color::srgb(1.0, 1.0, 0.8));
        
        // Draw a line from light to origin to show light direction
        gizmos.line(
            position,
            Vec3::ZERO,
            Color::srgb(1.0, 1.0, 0.0).with_alpha(0.5),
        );
    }
}