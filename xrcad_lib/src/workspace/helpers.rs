use bevy::prelude::*;
use crate::workspace::workspace::Workspace;

/// System to render workspace helpers (axes, planes, etc.)
pub fn workspace_render_system(
    mut gizmos: Gizmos,
    workspace: Res<Workspace>,
) {
    // Example: render all planes
    let plane_size = 120.0;
    for helper in &workspace.helpers {
        if let crate::workspace::workspace::HelperKind::Plane(plane) = &helper.kind {
            let (u, v) = plane_axes(Vec3::Z); // Replace Vec3::Z with actual normal if available
            let origin = Vec3::ZERO; // Replace with actual origin if available
            let corners = [
                origin + u * plane_size + v * plane_size,
                origin - u * plane_size + v * plane_size,
                origin - u * plane_size - v * plane_size,
                origin + u * plane_size - v * plane_size,
            ];
            for i in 0..4 {
                gizmos.line(corners[i], corners[(i + 1) % 4], Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}

/// Helper: get two perpendicular vectors to the normal for plane drawing
pub fn plane_axes(normal: Vec3) -> (Vec3, Vec3) {
    let u = if normal.x.abs() < 0.9 {
        normal.cross(Vec3::X).normalize()
    } else {
        normal.cross(Vec3::Y).normalize()
    };
    let v = normal.cross(u).normalize();
    (u, v)
}
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: workspace::helpers

