use bevy::prelude::*;
use bevy::prelude::{DirectionalLight};

// Camera UI state resource
#[derive(Resource)]
struct CameraUiState {
    pub pan_sensitivity: f32,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub is_xr: bool,
    pub is_stereo: bool,
}

impl Default for CameraUiState {
    fn default() -> Self {
        CameraUiState {
            pan_sensitivity: 0.5,
            rotate_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            is_xr: false,
            is_stereo: false,
        }
    }
}

use xrcad_lib::viewport::camera_control::{CustomCameraController, camera_control_system};

use xrcad_lib::{
    Workbench,
    HelperKind,
    ConstructionPlane, PlaneRenderMode,
    cube, // Only import what we're using
    PrimitiveResult, // Import the result type
};


use nalgebra::Point3;


use nalgebra::{Vector3};



fn main() {
    // Insert default camera UI state
    let camera_ui_state = CameraUiState::default();
    // --- Plane test cases ---
    let plane_yz = ConstructionPlane::yz();
    let plane_3pts = ConstructionPlane::from_points(
        Point3::new(0.0, 0.0, -100.0),
        Point3::new(100.0, 0.0, 0.0),
        Point3::new(100.0, 100.0, 0.0),
    ).expect("Failed to create plane from 3 points");

    let plane_rot = {
        let mut p = ConstructionPlane::from_point_normal(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 1.0), None)
            .expect("Failed to create plane from point and normal");
        p.rotation = std::f64::consts::FRAC_PI_4; // 45 deg
        p.render_mode = PlaneRenderMode::Highlighted;
        p
    };
    // Add test planes to the workspace as helpers so they get rendered
    let mut workbench = Workbench::default();
    workbench.add_helper("test_plane_xy", HelperKind::Plane(plane_yz));
    workbench.add_helper("test_plane_3pts", HelperKind::Plane(plane_3pts));
    workbench.add_helper("test_plane_rot", HelperKind::Plane(plane_rot));

    // Set render modes for the test planes
    workbench.set_plane_render_mode("test_plane_xy", PlaneRenderMode::Grid);
    workbench.set_plane_render_mode("test_plane_3pts", PlaneRenderMode::Ghosted);
    workbench.set_plane_render_mode("test_plane_rot", PlaneRenderMode::Highlighted);

    // Create a cube using the primitives library
    let cube_geometry = cube(200.0); // 200mm cube
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
        .add_systems(Startup, render_brep_geometry)
        .add_systems(Update, Workbench::workbench_render_system);

    // Conditionally add XR plugin if the feature is enabled
    #[cfg(feature = "openxr")]
    {
        use bevy_openxr::OpenXrPlugin;
        app.add_plugins(OpenXrPlugin);
    }

    app.run();
}

// Camera UI panel system (Bevy UI only)
fn camera_ui_panel(
    mut ui_state: ResMut<CameraUiState>,
    mut text_query: Query<&mut Text, With<CameraPanelText>>,
    mut camera_query: Query<&mut CustomCameraController>,
    keyboard: Res<ButtonInput<KeyCode>>,
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
    // Update camera controller with new sensitivities
    for mut cam in camera_query.iter_mut() {
        cam.pan_sensitivity = ui_state.pan_sensitivity;
        cam.rotate_sensitivity = ui_state.rotate_sensitivity;
        cam.zoom_sensitivity = ui_state.zoom_sensitivity;
        cam.is_xr = ui_state.is_xr;
        cam.is_stereo = ui_state.is_stereo;
    }
    // Update UI text panel with camera info
    if let Some(mut text) = text_query.iter_mut().next() {
        let mut content = String::from("Camera Controls:\n");
        content.push_str(&format!("Pan Sensitivity: {:.2} (P/O)\n", ui_state.pan_sensitivity));
        content.push_str(&format!("Rotate Sensitivity: {:.2} (T/Y)\n", ui_state.rotate_sensitivity));
        content.push_str(&format!("Zoom Sensitivity: {:.2} (Z/X)\n", ui_state.zoom_sensitivity));
        content.push_str(&format!("XR Enabled: {} (F1)\n", ui_state.is_xr));
        content.push_str(&format!("Stereo Enabled: {} (F2)\n", ui_state.is_stereo));
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
    ));

}
/// System to render BREP geometry as Bevy meshes
fn render_brep_geometry(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    brep: Res<PrimitiveResult>,
) {
    // Create vertices for the mesh
    let mut mesh_vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    
    // For proper cube normals, we need to duplicate vertices for each face
    // since each face has different normals
    
    // Bottom face (normal: [0, 0, -1])
    let bottom_normal = [0.0f32, 0.0f32, -1.0f32];
    mesh_vertices.extend_from_slice(&[
        [brep.vertices[0].position.x as f32, brep.vertices[0].position.y as f32, brep.vertices[0].position.z as f32],
        [brep.vertices[1].position.x as f32, brep.vertices[1].position.y as f32, brep.vertices[1].position.z as f32],
        [brep.vertices[2].position.x as f32, brep.vertices[2].position.y as f32, brep.vertices[2].position.z as f32],
        [brep.vertices[3].position.x as f32, brep.vertices[3].position.y as f32, brep.vertices[3].position.z as f32],
    ]);
    normals.extend_from_slice(&[bottom_normal, bottom_normal, bottom_normal, bottom_normal]);
    indices.extend_from_slice(&[0, 2, 1, 0, 3, 2]);
    
    // Top face (normal: [0, 0, 1])
    let top_normal = [0.0f32, 0.0f32, 1.0f32];
    let base_idx = mesh_vertices.len() as u32;
    mesh_vertices.extend_from_slice(&[
        [brep.vertices[4].position.x as f32, brep.vertices[4].position.y as f32, brep.vertices[4].position.z as f32],
        [brep.vertices[5].position.x as f32, brep.vertices[5].position.y as f32, brep.vertices[5].position.z as f32],
        [brep.vertices[6].position.x as f32, brep.vertices[6].position.y as f32, brep.vertices[6].position.z as f32],
        [brep.vertices[7].position.x as f32, brep.vertices[7].position.y as f32, brep.vertices[7].position.z as f32],
    ]);
    normals.extend_from_slice(&[top_normal, top_normal, top_normal, top_normal]);
    indices.extend_from_slice(&[base_idx, base_idx+1, base_idx+2, base_idx, base_idx+2, base_idx+3]);
    
    // Front face (normal: [0, -1, 0])
    let front_normal = [0.0f32, -1.0f32, 0.0f32];
    let base_idx = mesh_vertices.len() as u32;
    mesh_vertices.extend_from_slice(&[
        [brep.vertices[0].position.x as f32, brep.vertices[0].position.y as f32, brep.vertices[0].position.z as f32],
        [brep.vertices[1].position.x as f32, brep.vertices[1].position.y as f32, brep.vertices[1].position.z as f32],
        [brep.vertices[5].position.x as f32, brep.vertices[5].position.y as f32, brep.vertices[5].position.z as f32],
        [brep.vertices[4].position.x as f32, brep.vertices[4].position.y as f32, brep.vertices[4].position.z as f32],
    ]);
    normals.extend_from_slice(&[front_normal, front_normal, front_normal, front_normal]);
    indices.extend_from_slice(&[base_idx, base_idx+2, base_idx+1, base_idx, base_idx+3, base_idx+2]);
    
    // Back face (normal: [0, 1, 0])
    let back_normal = [0.0f32, 1.0f32, 0.0f32];
    let base_idx = mesh_vertices.len() as u32;
    mesh_vertices.extend_from_slice(&[
        [brep.vertices[3].position.x as f32, brep.vertices[3].position.y as f32, brep.vertices[3].position.z as f32],
        [brep.vertices[2].position.x as f32, brep.vertices[2].position.y as f32, brep.vertices[2].position.z as f32],
        [brep.vertices[6].position.x as f32, brep.vertices[6].position.y as f32, brep.vertices[6].position.z as f32],
        [brep.vertices[7].position.x as f32, brep.vertices[7].position.y as f32, brep.vertices[7].position.z as f32],
    ]);
    normals.extend_from_slice(&[back_normal, back_normal, back_normal, back_normal]);
    indices.extend_from_slice(&[base_idx, base_idx+1, base_idx+2, base_idx, base_idx+2, base_idx+3]);
    
    // Left face (normal: [-1, 0, 0])
    let left_normal = [-1.0f32, 0.0f32, 0.0f32];
    let base_idx = mesh_vertices.len() as u32;
    mesh_vertices.extend_from_slice(&[
        [brep.vertices[0].position.x as f32, brep.vertices[0].position.y as f32, brep.vertices[0].position.z as f32],
        [brep.vertices[3].position.x as f32, brep.vertices[3].position.y as f32, brep.vertices[3].position.z as f32],
        [brep.vertices[7].position.x as f32, brep.vertices[7].position.y as f32, brep.vertices[7].position.z as f32],
        [brep.vertices[4].position.x as f32, brep.vertices[4].position.y as f32, brep.vertices[4].position.z as f32],
    ]);
    normals.extend_from_slice(&[left_normal, left_normal, left_normal, left_normal]);
    indices.extend_from_slice(&[base_idx, base_idx+2, base_idx+1, base_idx, base_idx+3, base_idx+2]);
    
    // Right face (normal: [1, 0, 0])
    let right_normal = [1.0f32, 0.0f32, 0.0f32];
    let base_idx = mesh_vertices.len() as u32;
    mesh_vertices.extend_from_slice(&[
        [brep.vertices[1].position.x as f32, brep.vertices[1].position.y as f32, brep.vertices[1].position.z as f32],
        [brep.vertices[2].position.x as f32, brep.vertices[2].position.y as f32, brep.vertices[2].position.z as f32],
        [brep.vertices[6].position.x as f32, brep.vertices[6].position.y as f32, brep.vertices[6].position.z as f32],
        [brep.vertices[5].position.x as f32, brep.vertices[5].position.y as f32, brep.vertices[5].position.z as f32],
    ]);
    normals.extend_from_slice(&[right_normal, right_normal, right_normal, right_normal]);
    indices.extend_from_slice(&[base_idx, base_idx+1, base_idx+2, base_idx, base_idx+2, base_idx+3]);
    
    // Create UVs (simple planar mapping for each face)
    let mut uvs = Vec::new();
    for _ in 0..6 { // 6 faces
        uvs.extend_from_slice(&[
            [0.0f32, 0.0f32], [1.0f32, 0.0f32], 
            [1.0f32, 1.0f32], [0.0f32, 1.0f32]
        ]);
    }
    
    // Create the Bevy mesh
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    // Create material
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        metallic: 0.1,
        perceptual_roughness: 0.5,
        ..default()
    });
    
    // Spawn the mesh entity
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(material),
        Transform::default(),
    ));
    
    println!("Rendered cube geometry as Bevy mesh with proper face normals");
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

