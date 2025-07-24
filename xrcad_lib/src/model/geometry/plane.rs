//! Unified plane geometry for XRCAD
//!
//! This module provides a consolidated plane representation that serves as the foundation
//! for both BREP geometric operations and visual construction helpers.
//! 
//! The design eliminates duplication while maintaining clear separation of concerns:
//! - Core geometric representation (CorePlane)
//! - BREP operations (BrepPlane) 
//! - Visual helpers (ConstructionPlane)

use bevy::prelude::*;
use nalgebra::{Point3, Vector3};
use crate::model::plane_utils;

/// Unified render modes for all plane types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaneRenderMode {
    #[default]
    Grid,
    Solid,
    Highlighted,
    Ghosted,
    Wireframe,
    Hidden,
}

/// Methods for constructing a plane
#[derive(Debug, Clone, PartialEq)]
pub enum PlaneOrigin {
    PointNormal { point: Point3<f64>, normal: Vector3<f64>, offset: Option<f64> },
    ThreePoints { a: Point3<f64>, b: Point3<f64>, c: Point3<f64> },
    LineAngle { point: Point3<f64>, direction: Vector3<f64>, angle: f64 },
    StandardXY,
    StandardYZ,
    StandardXZ,
    Unknown,
}

/// Core geometric plane representation
/// This is the fundamental plane type that other plane types build upon
#[derive(Debug, Clone, PartialEq)]
pub struct CorePlane {
    /// Plane normal vector (normalized)
    pub normal: Vector3<f64>,
    /// Distance coefficient for plane equation: normal·point + d = 0
    pub d: f64,
    /// Origin point on the plane (for reference)
    pub origin: Point3<f64>,
    /// How this plane was constructed (for reconstruction/editing)
    pub construction_origin: PlaneOrigin,
}

impl CorePlane {
    /// Create a new plane from normal and origin point
    pub fn new(normal: Vector3<f64>, origin: Point3<f64>) -> Self {
        let normal = normal.normalize();
        let d = -normal.dot(&origin.coords);
        Self {
            normal,
            d,
            origin,
            construction_origin: PlaneOrigin::PointNormal { 
                point: origin, 
                normal, 
                offset: None 
            },
        }
    }

    /// Construct from a point and normal (optionally offset by distance along normal)
    pub fn from_point_normal(point: Point3<f64>, normal: Vector3<f64>, offset: Option<f64>) -> Option<Self> {
        // Check for degenerate normal vector
        if normal.norm() < 1e-10 {
            return None;
        }
        
        let result = plane_utils::construct_plane_from_point_normal(point, normal, offset);
        Some(Self {
            normal: result.normal,
            d: result.d_coefficient,
            origin: result.origin,
            construction_origin: PlaneOrigin::PointNormal { point, normal, offset },
        })
    }

    /// Construct from three non-collinear points
    pub fn from_points(a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Option<Self> {
        let result = plane_utils::construct_plane_from_points(a, b, c);
        
        // Check for degenerate case (collinear points)
        if result.normal.norm() < 1e-10 {
            return None;
        }
        
        Some(Self {
            normal: result.normal,
            d: result.d_coefficient,
            origin: result.origin,
            construction_origin: PlaneOrigin::ThreePoints { a, b, c },
        })
    }

    /// Construct from a line (point + direction) and an angle (radians) from the direction
    pub fn from_line_angle(point: Point3<f64>, direction: Vector3<f64>, angle: f64) -> Option<Self> {
        // Check for degenerate direction vector
        if direction.norm() < 1e-10 {
            return None;
        }
        
        let result = plane_utils::construct_plane_from_line_angle(point, direction, angle);
        Some(Self {
            normal: result.normal,
            d: result.d_coefficient,
            origin: result.origin,
            construction_origin: PlaneOrigin::LineAngle { point, direction, angle },
        })
    }

    /// Construct the XY plane (z=0)
    pub fn xy() -> Self {
        let result = plane_utils::construct_xy_plane();
        Self {
            normal: result.normal,
            d: result.d_coefficient,
            origin: result.origin,
            construction_origin: PlaneOrigin::StandardXY,
        }
    }

    /// Construct the YZ plane (x=0)
    pub fn yz() -> Self {
        let result = plane_utils::construct_yz_plane();
        Self {
            normal: result.normal,
            d: result.d_coefficient,
            origin: result.origin,
            construction_origin: PlaneOrigin::StandardYZ,
        }
    }

    /// Construct the XZ plane (y=0)
    pub fn xz() -> Self {
        let result = plane_utils::construct_xz_plane();
        Self {
            normal: result.normal,
            d: result.d_coefficient,
            origin: result.origin,
            construction_origin: PlaneOrigin::StandardXZ,
        }
    }

    /// Signed distance from a point to the plane
    pub fn distance(&self, point: &Point3<f64>) -> f64 {
        self.normal.dot(&point.coords) + self.d
    }

    /// Get normal as Bevy Vec3 for rendering
    pub fn normal_bevy(&self) -> Vec3 {
        plane_utils::na_vec3_to_bevy(&self.normal)
    }

    /// Get origin as Bevy Vec3 for rendering  
    pub fn origin_bevy(&self) -> Vec3 {
        plane_utils::na_point3_to_bevy(&self.origin)
    }
}

impl Default for CorePlane {
    fn default() -> Self {
        Self::xy()
    }
}

/// BREP plane for geometric operations
/// Extends CorePlane with advanced geometric operations needed for solid modeling
#[derive(Debug, Clone, PartialEq)]
pub struct BrepPlane {
    /// Core geometric representation
    pub core: CorePlane,
    /// In-plane rotation angle (radians, CCW about normal, relative to construction reference)
    pub rotation: f64,
    /// True if normal is as constructed, false if flipped
    pub facing: bool,
    /// If true, render the plane (for debugging/visualization)
    pub visible: bool,
    /// Current render mode (for debugging/visualization)
    pub render_mode: PlaneRenderMode,
    pub render_color: Color,
}

impl BrepPlane {
    /// Create a new BREP plane from a core plane
    pub fn new(core: CorePlane) -> Self {
        Self {
            core,
            rotation: 0.0,
            facing: true,
            visible: false, // BREP planes are typically not visible by default
            render_mode: PlaneRenderMode::Wireframe,
            render_color: Color::srgba(0.5, 0.5, 0.5, 0.3), // Default gray color
        }
    }

    /// Create from point and normal
    pub fn from_point_normal(point: Point3<f64>, normal: Vector3<f64>, offset: Option<f64>) -> Option<Self> {
        CorePlane::from_point_normal(point, normal, offset).map(Self::new)
    }

    /// Create from three points
    pub fn from_points(a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Option<Self> {
        CorePlane::from_points(a, b, c).map(Self::new)
    }

    /// Create from line and angle
    pub fn from_line_angle(point: Point3<f64>, direction: Vector3<f64>, angle: f64) -> Option<Self> {
        CorePlane::from_line_angle(point, direction, angle).map(Self::new)
    }

    /// Create standard planes
    pub fn xy() -> Self { Self::new(CorePlane::xy()) }
    pub fn yz() -> Self { Self::new(CorePlane::yz()) }
    pub fn plane_xz() -> Self { Self::new(CorePlane::xz()) }

    /// Returns a new plane rotated around its normal by the given angle (radians)
    pub fn rotate_around_normal(&self, angle: f64, center: Option<Point3<f64>>) -> Self {
        use nalgebra::{Rotation3, Unit};
        let axis = Unit::new_normalize(self.core.normal);
        let rot = Rotation3::from_axis_angle(&axis, angle);
        let c = center.unwrap_or(self.core.origin);

        // Rotate the origin around the center
        let rel = self.core.origin - c;
        let rotated = rot * rel;
        let new_origin = c + rotated;

        // Create new core plane with rotated origin
        let new_core = CorePlane::new(self.core.normal, new_origin);
        
        let mut plane = Self::new(new_core);
        plane.rotation = self.rotation + angle;
        plane.facing = self.facing;
        plane.visible = self.visible;
        plane.render_mode = self.render_mode;
        plane
    }

    /// Returns a new plane with its normal aligned to the given vector
    pub fn align_to_vector(&self, new_normal: Vector3<f64>) -> Self {
        let new_core = CorePlane::new(new_normal.normalize(), self.core.origin);
        let mut plane = Self::new(new_core);
        plane.rotation = self.rotation;
        plane.facing = self.facing;
        plane.visible = self.visible;
        plane.render_mode = self.render_mode;
        plane
    }

    /// Returns a new plane with the normal flipped
    pub fn flip_normal(&self) -> Self {
        let new_core = CorePlane::new(-self.core.normal, self.core.origin);
        let mut plane = Self::new(new_core);
        plane.rotation = self.rotation;
        plane.facing = !self.facing;
        plane.visible = self.visible;
        plane.render_mode = self.render_mode;
        plane
    }

    /// Set the render mode (for debugging/visualization)
    pub fn set_render_mode(&mut self, mode: PlaneRenderMode) {
        self.render_mode = mode;
    }
    
    pub fn set_render_color(&mut self, color: Color) {
        self.render_color = color;
    }

    /// Signed distance from a point to the plane
    pub fn distance(&self, point: &Point3<f64>) -> f64 {
        self.core.distance(point)
    }

    /// Render the plane (for debugging/visualization)
    pub fn render(&self, gizmos: &mut Gizmos) {
        if !self.visible {
            return;
        }

        use crate::color::*;
        let (color, alpha) = match self.render_mode {
            PlaneRenderMode::Wireframe => (CYAN, 0.5),
            PlaneRenderMode::Ghosted => (GREEN, 0.15),
            PlaneRenderMode::Highlighted => (YELLOW, 0.7),
            PlaneRenderMode::Grid => (MAGENTA, 0.3),
            PlaneRenderMode::Solid => (BLUE, 0.4),
            PlaneRenderMode::Hidden => return,
        };

        let size = 100.0; // TODO: parameterize

        match self.render_mode {
            PlaneRenderMode::Grid => {
                plane_utils::render_plane_grid_nalgebra(
                    gizmos,
                    &self.core.origin,
                    &self.core.normal,
                    size,
                    10, // steps
                    color,
                    alpha,
                );
            }
            _ => {
                plane_utils::render_plane_wireframe_nalgebra(
                    gizmos,
                    &self.core.origin,
                    &self.core.normal,
                    size,
                    color,
                    alpha,
                );
            }
        }
    }
}

impl Default for BrepPlane {
    fn default() -> Self {
        Self::new(CorePlane::default())
    }
}

/// Construction plane for workbench visualization
/// A visual aid that uses CorePlane for geometry but adds visualization-specific features
#[derive(Debug, Clone)]
pub struct ConstructionPlane {
    /// Core geometric representation
    pub core: CorePlane,
    /// Size for rendering (in world units)
    pub size: f32,
    /// In-plane rotation angle (radians, CCW about normal)
    pub rotation: f64,
    /// Render mode
    pub render_mode: PlaneRenderMode,
    /// Render color
    pub render_color: Color,
}

impl ConstructionPlane {
    /// Create a new construction plane from a core plane
    pub fn new(core: CorePlane) -> Self {
        Self {
            core,
            size: 100.0,
            rotation: 0.0,
            render_mode: PlaneRenderMode::Grid,
            render_color: Color::srgba(0.5, 0.5, 0.5, 0.3),
        }
    }

    /// Create from normal and origin
    pub fn from_normal_origin(normal: Vector3<f64>, origin: Point3<f64>) -> Self {
        Self::new(CorePlane::new(normal, origin))
    }

    /// Create from Bevy Vec3 inputs (convenience method)
    pub fn from_bevy(normal: Vec3, origin: Vec3) -> Self {
        let normal_na = Vector3::new(normal.x as f64, normal.y as f64, normal.z as f64);
        let origin_na = Point3::new(origin.x as f64, origin.y as f64, origin.z as f64);
        Self::from_normal_origin(normal_na, origin_na)
    }

    /// Create from point and normal
    pub fn from_point_normal(point: Point3<f64>, normal: Vector3<f64>, rotation: Option<f64>) -> Option<Self> {
        let core_plane = CorePlane::from_point_normal(point, normal, None)?;
        let mut plane = Self::new(core_plane);
        if let Some(rot) = rotation {
            plane.rotation = rot;
        }
        Some(plane)
    }

    /// Create from three points
    pub fn from_points(a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Option<Self> {
        CorePlane::from_points(a, b, c).map(Self::new)
    }

    /// Create from line and angle
    pub fn from_line_angle(point: Point3<f64>, direction: Vector3<f64>, angle: f64) -> Option<Self> {
        CorePlane::from_line_angle(point, direction, angle).map(Self::new)
    }

    /// Create standard planes
    pub fn xy() -> Self { Self::new(CorePlane::xy()) }
    pub fn yz() -> Self { Self::new(CorePlane::yz()) }
    pub fn xz() -> Self { Self::new(CorePlane::xz()) }

    /// Set the render mode
    pub fn set_render_mode(&mut self, mode: PlaneRenderMode) {
        self.render_mode = mode;
    }

    /// Set the render color
    pub fn set_render_color(&mut self, color: Color) {
        self.render_color = color;
    }

    /// Set the render color
    pub fn set_render_color_alpha(&mut self, color: Color, alpha: f32) {
        self.render_color = color;
        self.render_color.set_alpha(alpha);
    }

    /// Render the construction plane
    pub fn render(&self, gizmos: &mut Gizmos) {
        match self.render_mode {
            PlaneRenderMode::Hidden => return,
            PlaneRenderMode::Grid => self.render_grid(gizmos),
            PlaneRenderMode::Highlighted => self.render_highlighted(gizmos),
            PlaneRenderMode::Ghosted => self.render_ghosted(gizmos),
            PlaneRenderMode::Wireframe => self.render_wireframe(gizmos),
            PlaneRenderMode::Solid => self.render_solid(gizmos),
        }
    }

    fn render_grid(&self, gizmos: &mut Gizmos) {
        plane_utils::render_plane_grid(
            gizmos,
            self.core.origin_bevy(),
            self.core.normal_bevy(),
            self.size,
            10.0, // grid spacing
            self.render_color,
        );
    }

    fn render_highlighted(&self, gizmos: &mut Gizmos) {
        plane_utils::render_plane_highlighted(
            gizmos,
            self.core.origin_bevy(),
            self.core.normal_bevy(),
            self.size,
            self.render_color,
        );
    }

    fn render_ghosted(&self, gizmos: &mut Gizmos) {
        plane_utils::render_plane_ghosted(
            gizmos,
            self.core.origin_bevy(),
            self.core.normal_bevy(),
            self.size,
            self.render_color,
        );
    }

    fn render_wireframe(&self, gizmos: &mut Gizmos) {
        plane_utils::render_plane_wireframe(
            gizmos,
            self.core.origin_bevy(),
            self.core.normal_bevy(),
            self.size,
            self.render_color,
        );
    }

    fn render_solid(&self, gizmos: &mut Gizmos) {
        // TODO: Implement solid plane rendering when gizmos supports filled shapes
        // For now, fall back to wireframe
        self.render_wireframe(gizmos);
    }
}

impl Default for ConstructionPlane {
    fn default() -> Self {
        Self::new(CorePlane::default())
    }
}

// Type aliases for backward compatibility and ergonomics
pub type Plane = BrepPlane; // For BREP operations
pub type WorkbenchPlane = ConstructionPlane; // For workbench helpers
