
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::brep::topology::{vertex::Vertex, edge::Edge, edge_loop::EdgeLoop, face::Face};
use nalgebra as na;
use crate::color::{YELLOW, WHITE};
use crate::PrimitiveResult;

#[derive(Resource)]
pub struct BrepModel {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub edgeloops: Vec<EdgeLoop>,
    pub faces: Vec<Face>,
    /// Currently selected vertex (by id/index), if any
    pub selected_vertex: Option<usize>,
}

// --- Conversion helpers for f64 <-> f32 (nalgebra <-> bevy) ---
pub fn na_vec3_to_bevy(v: &na::Vector3<f64>) -> bevy::prelude::Vec3 {
    bevy::prelude::Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}
pub fn bevy_vec3_to_na(v: &bevy::prelude::Vec3) -> na::Vector3<f64> {
    na::Vector3::new(v.x as f64, v.y as f64, v.z as f64)
}


impl BrepModel {
        pub fn render(
        mut gizmos: Gizmos,
        brepmodel: Res<BrepModel>,
    ) {
        for edge in &brepmodel.edges {
            let v0 = &brepmodel.vertices[edge.vertices.0];
            let v1 = &brepmodel.vertices[edge.vertices.1];
            gizmos.line(na_vec3_to_bevy(&v0.position), na_vec3_to_bevy(&v1.position), WHITE);
        }
        for v in &brepmodel.vertices {
            gizmos.circle(na_vec3_to_bevy(&v.position), 8.0, YELLOW);
        }
    }

    pub fn vertex_drag(
        mouse: Res<ButtonInput<MouseButton>>,
        window_q: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform)>,
        mut brepmodel: ResMut<BrepModel>,
    ) {
        let Ok(window) = window_q.single() else { return; };
        let Ok((camera, camera_transform)) = q_camera.single() else { return; };
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let denom = ray.direction.z;
                if denom.abs() > 1e-6 {
                    let t = -ray.origin.z / denom;
                    let world_pos = ray.origin + ray.direction * t;
                    if mouse.just_pressed(MouseButton::Left) {
                        if let Some(selected_id) = brepmodel.vertices.iter_mut().find(|v| (na_vec3_to_bevy(&v.position).xy() - world_pos.xy()).length() < 12.0).map(|v| v.id as usize) {
                            brepmodel.selected_vertex = Some(selected_id);
                        }
                    }
                    if mouse.pressed(MouseButton::Left) {
                        if let Some(id) = brepmodel.selected_vertex {
                            if let Some(v) = brepmodel.vertices.iter_mut().find(|v| v.id as usize == id) {
                                v.position = bevy_vec3_to_na(&world_pos);
                            }
                        }
                    }
                    if mouse.just_released(MouseButton::Left) {
                        brepmodel.selected_vertex = None;
                    }
                }
            }
        }
    }
}

/// System to render BREP geometry as Bevy meshes
pub fn render_brep_geometry(
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
pub struct BrepPanelText;

pub fn setup_brep_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // BREP panel (top left)
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(20.0),
            height: Val::Percent(50.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..Default::default()
            },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.5)),

    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("BREP Controls\n\nVertices:\n"),
            TextFont {
                font: asset_server.load("fonts/FiraCode-Light.ttf"),
                font_size: 10.0,
                ..default()
            },
            BrepPanelText,
        ));
    });
}

pub fn brep_ui_panel(
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
