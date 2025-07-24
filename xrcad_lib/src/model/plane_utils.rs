//! Shared plane utilities for both B-rep planes and construction planes
//!
//! This module provides common functionality for plane mathematics and rendering
//! that can be used by both geometric B-rep planes and visual construction planes.

use bevy::prelude::*;
use nalgebra::{Point3, Vector3};

// Type alias for cleaner code
type NaVector3<T> = Vector3<T>;

/// Generic result of plane construction calculations
/// Stores data in the canonical nalgebra format and provides conversions
#[derive(Debug, Clone)]
pub struct PlaneConstructionResult {
    pub normal: Vector3<f64>,
    pub origin: Point3<f64>,
    pub d_coefficient: f64, // For plane equation: normal·point + d = 0
}

impl PlaneConstructionResult {
    /// Get the normal as a Bevy Vec3
    pub fn normal_bevy(&self) -> Vec3 {
        na_vec3_to_bevy(&self.normal)
    }
    
    /// Get the origin as a Bevy Vec3
    pub fn origin_bevy(&self) -> Vec3 {
        na_point3_to_bevy(&self.origin)
    }
    
    /// Get the normal as a nalgebra Vector3 (identity function for ergonomics)
    pub fn normal_nalgebra(&self) -> Vector3<f64> {
        self.normal
    }
    
    /// Get the origin as a nalgebra Point3 (identity function for ergonomics)
    pub fn origin_nalgebra(&self) -> Point3<f64> {
        self.origin
    }
    
    /// Convert to a typed result for convenient destructuring
    pub fn as_nalgebra(&self) -> (Vector3<f64>, Point3<f64>, f64) {
        (self.normal, self.origin, self.d_coefficient)
    }
    
    /// Convert to Bevy types for convenient destructuring
    pub fn as_bevy(&self) -> (Vec3, Vec3, f64) {
        (self.normal_bevy(), self.origin_bevy(), self.d_coefficient)
    }
}

/// Get two perpendicular vectors to a normal for plane visualization
/// Works with both Bevy Vec3 and nalgebra Vector3
pub fn compute_plane_axes_bevy(normal: Vec3) -> (Vec3, Vec3) {
    let u = if normal.x.abs() < 0.9 {
        normal.cross(Vec3::X).normalize()
    } else {
        normal.cross(Vec3::Y).normalize()
    };
    let v = normal.cross(u).normalize();
    (u, v)
}

/// Get two perpendicular vectors to a normal for plane visualization (nalgebra version)
pub fn compute_plane_axes_nalgebra(normal: &NaVector3<f64>) -> (NaVector3<f64>, NaVector3<f64>) {
    let n = normal.normalize();
    let u = if n.x.abs() < 0.9 {
        n.cross(&NaVector3::x()).normalize()
    } else {
        n.cross(&NaVector3::y()).normalize()
    };
    let v = n.cross(&u).normalize();
    (u, v)
}

/// Convert nalgebra Vector3 to Bevy Vec3
pub fn na_vec3_to_bevy(v: &NaVector3<f64>) -> Vec3 {
    Vec3::new(v.x as f32, v.y as f32, v.z as f32)
}

/// Convert nalgebra Point3 to Bevy Vec3
pub fn na_point3_to_bevy(p: &Point3<f64>) -> Vec3 {
    Vec3::new(p.x as f32, p.y as f32, p.z as f32)
}

/// Render a grid pattern on a plane
pub fn render_plane_grid(
    gizmos: &mut bevy::gizmos::gizmos::Gizmos,
    origin: Vec3,
    normal: Vec3,
    size: f32,
    grid_spacing: f32,
    color: Color,
) {
    let (u, v) = compute_plane_axes_bevy(normal);
    let grid_lines = (size / grid_spacing) as i32;

    for i in -grid_lines..=grid_lines {
        let offset = i as f32 * grid_spacing;
        // Lines parallel to u axis
        let start1 = origin + v * size + u * offset;
        let end1 = origin - v * size + u * offset;
        gizmos.line(start1, end1, color);
        
        // Lines parallel to v axis
        let start2 = origin + u * size + v * offset;
        let end2 = origin - u * size + v * offset;
        gizmos.line(start2, end2, color);
    }
}

/// Render a Highlighted plane
pub fn render_plane_highlighted(
    gizmos: &mut bevy::gizmos::gizmos::Gizmos,
    origin: Vec3,
    normal: Vec3,
    size: f32,
    color: Color,
) {
    let (u, v) = compute_plane_axes_bevy(normal);
    let corners = [
        origin + u * size + v * size,
        origin - u * size + v * size,
        origin - u * size - v * size,
        origin + u * size - v * size,
    ];

    for i in 0..4 {
        gizmos.line(corners[i], corners[(i + 1) % 4], color);
    }
}

/// Render a ghosted plane
pub fn render_plane_ghosted(
    gizmos: &mut bevy::gizmos::gizmos::Gizmos,
    origin: Vec3,
    normal: Vec3,
    size: f32,
    color: Color,
) {
    let (u, v) = compute_plane_axes_bevy(normal);
    let corners = [
        origin + u * size + v * size,
        origin - u * size + v * size,
        origin - u * size - v * size,
        origin + u * size - v * size,
    ];

    for i in 0..4 {
        gizmos.line(corners[i], corners[(i + 1) % 4], color);
    }
}

/// Render a wireframe outline of a plane
pub fn render_plane_wireframe(
    gizmos: &mut bevy::gizmos::gizmos::Gizmos,
    origin: Vec3,
    normal: Vec3,
    size: f32,
    color: Color,
) {
    let (u, v) = compute_plane_axes_bevy(normal);
    let corners = [
        origin + u * size + v * size,
        origin - u * size + v * size,
        origin - u * size - v * size,
        origin + u * size - v * size,
    ];
    
    for i in 0..4 {
        gizmos.line(corners[i], corners[(i + 1) % 4], color);
    }
}

/// Render a grid pattern on a plane (nalgebra version for B-rep planes)
pub fn render_plane_grid_nalgebra(
    gizmos: &mut bevy::gizmos::gizmos::Gizmos,
    center: &Point3<f64>,
    normal: &NaVector3<f64>,
    size: f64,
    steps: i32,
    color: Color,
    alpha: f32,
) {
    let (u, v) = compute_plane_axes_nalgebra(normal);
    
    for i in -steps..=steps {
        let t = i as f64 / steps as f64 * size;
        // u lines
        let start1 = center + u * t + v * size;
        let end1 = center + u * t - v * size;
        gizmos.line(
            na_point3_to_bevy(&start1),
            na_point3_to_bevy(&end1),
            color.with_alpha(alpha * 0.7),
        );
        
        // v lines
        let start2 = center + v * t + u * size;
        let end2 = center + v * t - u * size;
        gizmos.line(
            na_point3_to_bevy(&start2),
            na_point3_to_bevy(&end2),
            color.with_alpha(alpha * 0.7),
        );
    }
}

/// Render a wireframe outline of a plane (nalgebra version for B-rep planes)
pub fn render_plane_wireframe_nalgebra(
    gizmos: &mut bevy::gizmos::gizmos::Gizmos,
    center: &Point3<f64>,
    normal: &NaVector3<f64>,
    size: f64,
    color: Color,
    alpha: f32,
) {
    let (u, v) = compute_plane_axes_nalgebra(normal);
    let corners = [
        center + u * size + v * size,
        center - u * size + v * size,
        center - u * size - v * size,
        center + u * size - v * size,
    ];
    
    // Draw quad outline
    for i in 0..4 {
        gizmos.line(
            na_point3_to_bevy(&corners[i]),
            na_point3_to_bevy(&corners[(i + 1) % 4]),
            color.with_alpha(alpha),
        );
    }
}

// ============================================================================
// PLANE CONSTRUCTION FUNCTIONS
// ============================================================================

/// Construct plane from a point and normal vector
pub fn construct_plane_from_point_normal(
    point: Point3<f64>, 
    normal: Vector3<f64>,
    offset: Option<f64>
) -> PlaneConstructionResult {
    let n = normal.normalize();
    let d = -n.dot(&point.coords) + offset.unwrap_or(0.0);
    
    PlaneConstructionResult {
        normal: n,
        origin: point,
        d_coefficient: d,
    }
}

/// Construct plane from three non-collinear points
pub fn construct_plane_from_points(
    a: Point3<f64>, 
    b: Point3<f64>, 
    c: Point3<f64>
) -> PlaneConstructionResult {
    let ab = b - a;
    let ac = c - a;
    let n = ab.cross(&ac);
    if n.norm() < 1e-10 {
         // Degenerate (collinear points)
        return PlaneConstructionResult {
            normal: Vector3::zeros(),
            origin: Point3::origin(),
            d_coefficient: 0.0,
        };
    }
    
    construct_plane_from_point_normal(a, n, None)
}

/// Construct plane from a line (point + direction) and an angle from the direction
pub fn construct_plane_from_line_angle(
    point: Point3<f64>, 
    direction: Vector3<f64>, 
    angle: f64
) -> PlaneConstructionResult {
    let dir = direction.normalize();
    let up = if dir.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
    let perp = dir.cross(&up).normalize();
    let normal = (dir * angle.cos() + perp * angle.sin()).normalize();
    
    construct_plane_from_point_normal(point, normal, None)
}

/// Construct the XY plane (z=0) in nalgebra coordinates
pub fn construct_xy_plane() -> PlaneConstructionResult {
    PlaneConstructionResult {
        normal: Vector3::z(),
        origin: Point3::origin(),
        d_coefficient: 0.0,
    }
}

/// Construct the YZ plane (x=0) in nalgebra coordinates
pub fn construct_yz_plane() -> PlaneConstructionResult {
    PlaneConstructionResult {
        normal: Vector3::x(),
        origin: Point3::origin(),
        d_coefficient: 0.0,
    }
}

/// Construct the XZ plane (y=0) in nalgebra coordinates
pub fn construct_xz_plane() -> PlaneConstructionResult {
    PlaneConstructionResult {
        normal: Vector3::y(),
        origin: Point3::origin(),
        d_coefficient: 0.0,
    }
}

/// Construct plane from Bevy Vec3 normal and origin (convenience for construction planes)
pub fn construct_plane_from_bevy_vectors(
    normal: Vec3, 
    origin: Vec3
) -> PlaneConstructionResult {
    let normal_na = Vector3::new(normal.x as f64, normal.y as f64, normal.z as f64);
    let origin_na = Point3::new(origin.x as f64, origin.y as f64, origin.z as f64);
    
    construct_plane_from_point_normal(origin_na, normal_na, None)
}

// ============================================================================
// BEVY-NATIVE CONSTRUCTION FUNCTIONS (for visual aids like ConstructionPlane)
// ============================================================================

/// Simple result for Bevy-native plane construction (visual aids only)
#[derive(Debug, Clone)]
pub struct BevyPlaneResult {
    pub normal: Vec3,
    pub origin: Vec3,
}

/// Construct plane from three Bevy Vec3 points (Bevy-native, f32 precision)
pub fn construct_bevy_plane_from_points(a: Vec3, b: Vec3, c: Vec3) -> Option<BevyPlaneResult> {
    let ab = b - a;
    let ac = c - a;
    let normal = ab.cross(ac);
    
    if normal.length() < 1e-6 {
        return None; // Degenerate (collinear points)
    }
    
    Some(BevyPlaneResult {
        normal: normal.normalize(),
        origin: a,
    })
}

/// Construct XY plane in Bevy coordinates (z=0)
pub fn construct_bevy_xy_plane() -> BevyPlaneResult {
    BevyPlaneResult {
        normal: Vec3::Z,
        origin: Vec3::ZERO,
    }
}

/// Construct YZ plane in Bevy coordinates (x=0)
pub fn construct_bevy_yz_plane() -> BevyPlaneResult {
    BevyPlaneResult {
        normal: Vec3::X,
        origin: Vec3::ZERO,
    }
}

/// Construct XZ plane in Bevy coordinates (y=0)
pub fn construct_bevy_xz_plane() -> BevyPlaneResult {
    BevyPlaneResult {
        normal: Vec3::Y,
        origin: Vec3::ZERO,
    }
}
