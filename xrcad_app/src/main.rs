
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

use xrcad_lib::model::brep::topology::plane::{Plane, PlaneRenderMode};
use nalgebra::Point3;


use nalgebra::{Vector3};


use xrcad_lib::{BrepModel, Vertex, Edge, Face, EdgeLoop, Workspace};

fn main() {
    // Insert default camera UI state
    let camera_ui_state = CameraUiState::default();
    // --- Plane test cases ---
    let plane_yz = Plane::yz();
    let plane_3pts = Plane::from_points(
        Point3::new(0.0, 0.0, -100.0),
        Point3::new(100.0, 0.0, 0.0),
        Point3::new(100.0, 100.0, 0.0),
    ).unwrap();
    let plane_rot = {
        let mut p = Plane::from_point_normal(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 1.0), None);
        p.rotation = std::f64::consts::FRAC_PI_4; // 45 deg
        p.render_mode = PlaneRenderMode::Highlighted;
        p
    };
    // Add test planes to the workspace as helpers so they get rendered
    let mut workspace = Workspace::default();
    workspace.add_helper("test_plane_xy", xrcad_lib::workspace::workspace::HelperKind::Plane(plane_yz));
    workspace.add_helper("test_plane_3pts", xrcad_lib::workspace::workspace::HelperKind::Plane(plane_3pts));
    workspace.add_helper("test_plane_rot", xrcad_lib::workspace::workspace::HelperKind::Plane(plane_rot));

    // Set render modes for the test planes
    workspace.set_plane_render_mode("test_plane_xy", PlaneRenderMode::Grid);
    workspace.set_plane_render_mode("test_plane_3pts", PlaneRenderMode::Ghosted);
    workspace.set_plane_render_mode("test_plane_rot", PlaneRenderMode::Highlighted);

    let vertices = vec![
        Vertex { id: 0, position: Vector3::new(-100.0, -100.0, 0.0) },
        Vertex { id: 1, position: Vector3::new(100.0, -100.0, 0.0) },
        Vertex { id: 2, position: Vector3::new(100.0, 100.0, 0.0) },
        Vertex { id: 3, position: Vector3::new(-100.0, 100.0, 0.0) },
    ];
    let edges = vec![
        Edge { id: 0, vertices: (0, 1) },
        Edge { id: 1, vertices: (1, 2) },
        Edge { id: 2, vertices: (2, 3) },
        Edge { id: 3, vertices: (3, 0) },
    ];
    let edgeloops = vec![EdgeLoop::new(1, vec![edges.iter().map(|e| e.id).collect()])];
    let faces = edgeloops.iter().enumerate().map(|(i, l)| Face { id: i as usize, edge_loops: vec![l.id] }).collect::<Vec<Face>>();
    App::new()
        .insert_resource(BrepModel {
            vertices,
            edges,
            edgeloops,
            faces,
            selected_vertex: None,
        })
        .insert_resource(workspace)
        .add_plugins(DefaultPlugins)
        .insert_resource(camera_ui_state)
        .add_systems(Update, camera_control_system)
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, update_ui_panel)
        .add_systems(Update, camera_ui_panel)
        .add_systems(Update, BrepModel::render)
        .add_systems(Update, BrepModel::vertex_drag)
        .add_systems(Update, Workspace::workspace_render_system)
        .run();
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
        Transform::from_xyz(0.0, 1000.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

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
    brep: Res<BrepModel>,
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

