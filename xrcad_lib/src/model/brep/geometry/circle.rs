// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::core::geom::circle

use bevy::math::Vec3 as Vec3_sp;
use nalgebra::Point3;

#[derive(Debug, Default, Clone)]
pub struct Circle{
    pub position: Point3<f64>,
    pub radius: f64,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            position: Point3::default(),
            radius: 1.0,
        }
    }
    // ...other inherent methods...

    /// Convert to Bevy-compatible types (Vec3, f32 radius)
    pub fn as_sp(&self) -> (Vec3_sp, f32) {
        (
            Vec3_sp::new(
                self.position.x as f32,
                self.position.y as f32,
                self.position.z as f32,
            ),
            self.radius as f32,
        )
    }
    /// Example: add two radii using f64
    pub fn add_radius_hp(&self, other: &Self) -> f64 {
        self.radius + other.radius
    }
    /// Example: get position as array of f32
    pub fn position_as_sp(&self) -> [f32; 3] {
        [self.position.x as f32, self.position.y as f32, self.position.z as f32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_circle_new() {
        let c = Circle::new();
        assert_eq!(c.position, Point3::new(0.0, 0.0, 0.0));
        assert_eq!(c.radius, 1.0);
    }
    #[test]
    fn test_as_sp() {
        let c = Circle {
            position: Point3::new(1.5, 2.5, 3.5),
            radius: 4.5,
        };
        let (vec, r) = c.as_sp();
        assert_eq!(vec, bevy::math::Vec3::new(1.5_f32, 2.5_f32, 3.5_f32));
        assert_eq!(r, 4.5_f32);
    }
    #[test]
    fn test_add_radius_hp() {
        let a = Circle { position: Point3::new(0.0,0.0,0.0), radius: 2.0 };
        let b = Circle { position: Point3::new(0.0,0.0,0.0), radius: 3.0 };
        assert_eq!(a.add_radius_hp(&b), 5.0);
    }
    #[test]
    fn test_position_as_sp() {
        let c = Circle { position: Point3::new(1.0, 2.0, 3.0), radius: 1.0 };
        assert_eq!(c.position_as_sp(), [1.0_f32, 2.0_f32, 3.0_f32]);
    }
}
