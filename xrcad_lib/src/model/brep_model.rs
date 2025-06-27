
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::brep::topology::{vertex::Vertex, edge::Edge, edge_loop::EdgeLoop, face::Face};
use nalgebra as na;
use crate::color::{YELLOW, WHITE};

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