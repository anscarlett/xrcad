use bevy::prelude::*;

use nalgebra::{Vector3};


use xrcad_lib::{BrepModel, Vertex, Edge, Face, EdgeLoop, Workspace};

// use topology::{Vertex, Edge, Face, BrepModel};
// use components::workspace::Workspace;

// use crate::components::axes::draw;

fn main() {
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
        .insert_resource(Workspace::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, update_ui_panel)
        .add_systems(Update, BrepModel::render)
        .add_systems(Update, BrepModel::vertex_drag)
        .add_systems(Update, Workspace::workspace_render_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-500.0, 500.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
}

#[derive(Component)]
struct ControlsPanel;

#[derive(Component)]
struct PanelText;

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node::default(),
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        ControlsPanel,
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("BREP Controls\n\nVertices:\n"),
            PanelText,
        ));
    });
}


fn update_ui_panel(
    brep: Res<BrepModel>,
    mut query: Query<&mut Text, With<PanelText>>,
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

