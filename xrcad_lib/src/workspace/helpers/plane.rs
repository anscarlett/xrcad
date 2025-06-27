// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: workspace::helpers::plane

use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct Plane;

impl Plane {
    pub fn render(&self, gizmos: &mut Gizmos) {
        let origin = Vec3::ZERO;
        let normal = Vec3::Z;
        let plane_size = 120.0;
        let (u, v) = plane_axes(normal);
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

fn plane_axes(normal: Vec3) -> (Vec3, Vec3) {
    let u = if normal.x.abs() < 0.9 {
        normal.cross(Vec3::X).normalize()
    } else {
        normal.cross(Vec3::Y).normalize()
    };
    let v = normal.cross(u).normalize();
    (u, v)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plane_default() {
        let plane = Plane::default();
        let _ = plane;
    }    
}    
