use bevy::prelude::*;

use xrcad_lib::{
    viewport::camera_control::{
        CustomCameraController, 
        camera_control_system
    },
    Workbench,
    cube,
    PrimitiveResult,
};

// Camera UI state resource is now moved to xrcad_lib::viewport::camera_control

// Light control marker component
#[derive(Component)]
struct LightController;








fn main() {
    // Insert default camera UI state
    let camera_ui_state = CameraUiState::default();
    let workbench = Workbench::default();

    // Create a cube using the primitives library
    let cube_geometry = cube(50.0); // 200mm cube
    println!("Created cube with {} vertices, {} edges, {} faces", 
             cube_geometry.vertices.len(), 
             cube_geometry.edges.len(), 
             cube_geometry.faces.len());
    
    // Print cube details for testing
    println!("Cube vertices:");
    for (i, vertex) in cube_geometry.vertices.iter().enumerate() {
        println!("  {}: ({:.1}, {:.1}, {:.1})", i, vertex.position.x, vertex.position.y, vertex.position.z);
    }
    
    println!("Cube edges:");
    for edge in cube_geometry.edges.iter() {
        println!("  {}: {:?}", edge.id, edge.vertices);
    }
    
    println!("Cube faces:");
    for face in cube_geometry.faces.iter() {
        println!("  {}: edge_loops {:?}", face.id, face.edge_loops);
    }
    
    let mut app = App::new();
    app
        .insert_resource(cube_geometry)
        .insert_resource(workbench)
        .add_plugins(DefaultPlugins)
        .insert_resource(camera_ui_state)
        .add_systems(Update, camera_control_system)
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, update_ui_panel)
        .add_systems(Update, camera_ui_panel)
        .add_systems(Update, light_control_system)
        .add_systems(Update, render_light_axes)
        .add_systems(Startup, render_brep_geometry)
        .add_systems(Update, Workbench::workbench_render_system);

    // Conditionally add XR plugin if the feature is enabled
    #[cfg(feature = "openxr")]
    {
        use bevy_openxr::OpenXrPlugin;
use xrcad_lib::viewport::camera_control::CameraUiState;
        app.add_plugins(OpenXrPlugin);
    }

    app.run();
}

// Light control system
fn light_control_system(
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
fn render_light_axes(
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

// Camera UI panel system (Bevy UI only)
fn camera_ui_panel(
    mut ui_state: ResMut<CameraUiState>,
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

fn setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn((
        // Camera3d::default(),
        Camera3d {
            screen_space_specular_transmission_steps: 0,
            ..Default::default()
        },
            
        Transform::from_xyz(-500.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        CustomCameraController::default(),
    ));

    // Spawn a directional light so objects are visible
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1000.0, 1000.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        LightController,
    ));

}

/// System to render BREP geometry as Bevy meshes
fn render_brep_geometry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    brep: Res<PrimitiveResult>,
) {
    // Create 6 different colored materials for each face
    // Your requested color scheme: +X red, -X magenta, +Y green, -Y yellow, +Z blue, -Z cyan
    let materials_vec = vec![
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.0), // Yellow - Bottom face (-Y)
            metallic: 0.1,
            perceptual_roughness: 0.5,
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0), // Green - Top face (+Y) 
            metallic: 0.1,
            perceptual_roughness: 0.5,
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 1.0), // Cyan - Back face (-Z)
            metallic: 0.1,
            perceptual_roughness: 0.5,
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0), // Blue - Front face (+Z)
            metallic: 0.1,
            perceptual_roughness: 0.5,
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 1.0), // Magenta - Left face (-X)
            metallic: 0.1,
            perceptual_roughness: 0.5,
            ..default()
        }),
        materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0), // Red - Right face (+X)
            metallic: 0.1,
            perceptual_roughness: 0.5,
            ..default()
        }),
    ];

    // Cube vertex layout (from platonic.rs):
    // 0: (-half, -half, -half)  1: (+half, -half, -half)  
    // 2: (+half, +half, -half)  3: (-half, +half, -half)
    // 4: (-half, -half, +half)  5: (+half, -half, +half)
    // 6: (+half, +half, +half)  7: (-half, +half, +half)

    let face_data = [
        // Bottom face (YELLOW) - Y = -half, normal: [0, -1, 0] 
        // Vertices: 0,1,5,4 (counter-clockwise when viewed from below)
        (
            vec![
                [brep.vertices[0].position.x as f32, brep.vertices[0].position.y as f32, brep.vertices[0].position.z as f32],
                [brep.vertices[1].position.x as f32, brep.vertices[1].position.y as f32, brep.vertices[1].position.z as f32],
                [brep.vertices[5].position.x as f32, brep.vertices[5].position.y as f32, brep.vertices[5].position.z as f32],
                [brep.vertices[4].position.x as f32, brep.vertices[4].position.y as f32, brep.vertices[4].position.z as f32],
            ],
            vec![[0.0f32, -1.0f32, 0.0f32]; 4],
            vec![0u32, 1, 2, 0, 2, 3], // Two triangles
            0, // Yellow material index
        ),
        // Top face (GREEN) - Y = +half, normal: [0, 1, 0]
        // Vertices: 7,6,2,3 (counter-clockwise when viewed from above)
        (
            vec![
                [brep.vertices[7].position.x as f32, brep.vertices[7].position.y as f32, brep.vertices[7].position.z as f32],
                [brep.vertices[6].position.x as f32, brep.vertices[6].position.y as f32, brep.vertices[6].position.z as f32],
                [brep.vertices[2].position.x as f32, brep.vertices[2].position.y as f32, brep.vertices[2].position.z as f32],
                [brep.vertices[3].position.x as f32, brep.vertices[3].position.y as f32, brep.vertices[3].position.z as f32],
            ],
            vec![[0.0f32, 1.0f32, 0.0f32]; 4],
            vec![0u32, 1, 2, 0, 2, 3], // Two triangles
            1, // Green material index
        ),
        // Back face (CYAN) - Z = -half, normal: [0, 0, -1]
        // Vertices: 3,2,1,0 (counter-clockwise when viewed from back)
        (
            vec![
                [brep.vertices[3].position.x as f32, brep.vertices[3].position.y as f32, brep.vertices[3].position.z as f32],
                [brep.vertices[2].position.x as f32, brep.vertices[2].position.y as f32, brep.vertices[2].position.z as f32],
                [brep.vertices[1].position.x as f32, brep.vertices[1].position.y as f32, brep.vertices[1].position.z as f32],
                [brep.vertices[0].position.x as f32, brep.vertices[0].position.y as f32, brep.vertices[0].position.z as f32],
            ],
            vec![[0.0f32, 0.0f32, -1.0f32]; 4],
            vec![0u32, 1, 2, 0, 2, 3], // Two triangles
            2, // Cyan material index
        ),
        // Front face (BLUE) - Z = +half, normal: [0, 0, 1]
        // Vertices: 4,5,6,7 (counter-clockwise when viewed from front)
        (
            vec![
                [brep.vertices[4].position.x as f32, brep.vertices[4].position.y as f32, brep.vertices[4].position.z as f32],
                [brep.vertices[5].position.x as f32, brep.vertices[5].position.y as f32, brep.vertices[5].position.z as f32],
                [brep.vertices[6].position.x as f32, brep.vertices[6].position.y as f32, brep.vertices[6].position.z as f32],
                [brep.vertices[7].position.x as f32, brep.vertices[7].position.y as f32, brep.vertices[7].position.z as f32],
            ],
            vec![[0.0f32, 0.0f32, 1.0f32]; 4],
            vec![0u32, 1, 2, 0, 2, 3], // Two triangles
            3, // Blue material index
        ),
        // Left face (MAGENTA) - X = -half, normal: [-1, 0, 0]
        // Vertices: 0,4,7,3 (counter-clockwise when viewed from left)
        (
            vec![
                [brep.vertices[0].position.x as f32, brep.vertices[0].position.y as f32, brep.vertices[0].position.z as f32],
                [brep.vertices[4].position.x as f32, brep.vertices[4].position.y as f32, brep.vertices[4].position.z as f32],
                [brep.vertices[7].position.x as f32, brep.vertices[7].position.y as f32, brep.vertices[7].position.z as f32],
                [brep.vertices[3].position.x as f32, brep.vertices[3].position.y as f32, brep.vertices[3].position.z as f32],
            ],
            vec![[-1.0f32, 0.0f32, 0.0f32]; 4],
            vec![0u32, 1, 2, 0, 2, 3], // Two triangles
            4, // Magenta material index
        ),
        // Right face (RED) - X = +half, normal: [1, 0, 0]
        // Vertices: 1,2,6,5 (counter-clockwise when viewed from right)
        (
            vec![
                [brep.vertices[1].position.x as f32, brep.vertices[1].position.y as f32, brep.vertices[1].position.z as f32],
                [brep.vertices[2].position.x as f32, brep.vertices[2].position.y as f32, brep.vertices[2].position.z as f32],
                [brep.vertices[6].position.x as f32, brep.vertices[6].position.y as f32, brep.vertices[6].position.z as f32],
                [brep.vertices[5].position.x as f32, brep.vertices[5].position.y as f32, brep.vertices[5].position.z as f32],
            ],
            vec![[1.0f32, 0.0f32, 0.0f32]; 4],
            vec![0u32, 1, 2, 0, 2, 3], // Two triangles
            5, // Red material index
        ),
    ];

    // Create each face as a separate mesh entity
    for (vertices, normals, indices, material_idx) in face_data {
        // Create UVs
        let uvs = vec![
            [0.0f32, 0.0f32], [1.0f32, 0.0f32], 
            [1.0f32, 1.0f32], [0.0f32, 1.0f32]
        ];

        // Create the Bevy mesh
        let mut mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));

        // Spawn the mesh entity with the appropriate colored material
        commands.spawn((
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials_vec[material_idx].clone()),
            Transform::default(),
        ));
    }

    println!("Rendered cube with your requested color scheme:");
    println!("  YELLOW = Bottom face (-Y)");
    println!("  GREEN = Top face (+Y)");
    println!("  CYAN = Back face (-Z)");
    println!("  BLUE = Front face (+Z)");
    println!("  MAGENTA = Left face (-X)");
    println!("  RED = Right face (+X)");
}

#[derive(Component)]
struct ControlsPanel;

#[derive(Component)]
struct BrepPanelText;

#[derive(Component)]
struct CameraPanelText;

fn setup_ui(mut commands: Commands) {
    // BREP panel (top left)
    commands.spawn((
        Node::default(),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ControlsPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("BREP Controls\n\nVertices:\n"),
            BrepPanelText,
        ));
    });

    // Camera panel (top right)
    commands.spawn((
        Node::default(),
        BackgroundColor(Color::srgb(0.15, 0.1, 0.1)),
        ControlsPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("Camera Controls\n"),
            CameraPanelText,
        ));
    });
}


fn update_ui_panel(
    brep: Res<PrimitiveResult>,
    mut query: Query<&mut Text, With<BrepPanelText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        let mut content = String::from("BREP Controls\n\nVertices:\n");
        for v in &brep.vertices {
            content.push_str(&format!("{}: ({:.1}, {:.1}, {:.1})\n", v.id, v.position.x, v.position.y, v.position.z));
        }
        content.push_str("\nEdges:\n");
        for e in &brep.edges {
            content.push_str(&format!("{}: {:?}\n", e.id, e.vertices));
        }
        text.0 = content;
    }
}

