use bevy::prelude::*;
// Light control marker component
#[derive(Component)]
#[derive(Resource)]
pub struct CustomLightController{
    // Marker component for light control
    pub enabled: bool,
    pub move_speed: f32,
}

impl Default for CustomLightController {
    fn default() -> Self {
        Self {
            enabled: true,
            move_speed: 200.0, // units per second
        }
    }
}


/// Combined system for light movement (translation and orbit) and UI panel update
pub fn light_control_and_ui_panel(
    mut text_query: Query<&mut Text, With<LightPanelText>>,
    mut light_query: Query<(&mut Transform, &mut CustomLightController), With<DirectionalLight>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut light_transform, _light_controller)) = light_query.single_mut() {
        let move_speed = 200.0;
        let dt = time.delta_secs();

        let shift_pressed = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        let ctrl_pressed = keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight);

        // Shift+arrows/PageUp/PageDown: translate light
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
                *light_transform = light_transform.looking_at(Vec3::ZERO, Vec3::Y);
            }
        }

        // Control+arrows: orbit light
        if ctrl_pressed && (keyboard.pressed(KeyCode::ArrowLeft)
            || keyboard.pressed(KeyCode::ArrowRight)
            || keyboard.pressed(KeyCode::ArrowUp)
            || keyboard.pressed(KeyCode::ArrowDown)) {
            let rotation_speed: f32 = 90.0;
            let current_pos = light_transform.translation;
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
            light_transform.translation = new_position;
            *light_transform = light_transform.looking_at(Vec3::ZERO, Vec3::Y);
        }
    }

    // Update UI text panel with camera and light info
    if let Some(mut text) = text_query.iter_mut().next() {
        if let Some((light_transform, _)) = light_query.iter_mut().next() {
            let mut content = String::from("Light Controls:\n");
            content.push_str("Shift+Arrow Keys: Move light X/Z\n");
            content.push_str("PageUp/PageDown: Move light Y\n");
            content.push_str("Ctrl+Arrow Keys: Orbit light (azimuth/elevation)\n");
            let pos = light_transform.translation;
            content.push_str(&format!("Light Position: ({:.0}, {:.0}, {:.0})\n", pos.x, pos.y, pos.z));
            text.0 = content;
        }
    }
}

// System to render light position axes using gizmos
pub fn render_light_axes(
    mut gizmos: Gizmos,
    light_query: Query<&Transform, (With<DirectionalLight>, With<CustomLightController>)>,
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

#[derive(Component)]
pub struct LightPanelText;



pub fn setup_lighting(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn a directional light so objects are visible
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(200.0, 200.0, 200.0).looking_at(Vec3::ZERO, Vec3::Y),
        CustomLightController::default(),
    ));
        commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(20.0),
            height: Val::Percent(50.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..Default::default()
            },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.1, 0.5)),
        // ControlsPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Light Controls\n"),
            TextFont {
                font: asset_server.load("fonts/FiraCode-Light.ttf"),
                font_size: 10.0,
                ..default()
            }, 
            LightPanelText,
        ));
    });
}