// ...existing code...
impl Default for Plane {
    fn default() -> Self {
        Self::xy()
    }
}
use bevy::{color::Alpha};
use bevy::prelude::Gizmos;

use crate::color::*;
use crate::model::brep_model::na_vec3_to_bevy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneRenderMode {
    Simple,
    Ghosted,
    Highlighted,
    Grid,
}

use nalgebra::{Vector3, Point3};

/// A geometric plane in 3D, defined by normal and distance from origin (ax + by + cz + d = 0)

#[derive(Debug, Clone, PartialEq)]
pub enum PlaneOrigin {
    PointNormal { point: Point3<f64>, normal: Vector3<f64>, offset: Option<f64> },
    ThreePoints { a: Point3<f64>, b: Point3<f64>, c: Point3<f64> },
    LineAngle { point: Point3<f64>, direction: Vector3<f64>, angle: f64 },
    StandardXY,
    StandardYZ,
    StandardZX,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub normal: Vector3<f64>,
    pub d: f64,
    pub origin: PlaneOrigin,
    /// In-plane rotation angle (radians, CCW about normal, relative to construction reference)
    pub rotation: f64,
    /// True if normal is as constructed, false if flipped
    pub facing: bool,
    /// If true, render the plane
    pub visible: bool,
    /// Current render mode
    pub render_mode: PlaneRenderMode,
}

impl Plane {

    /// Set the render mode of the plane
    pub fn set_render_mode(&mut self, mode: PlaneRenderMode) {
        self.render_mode = mode;
    }

    /// Returns a new plane rotated around its normal by the given angle (radians), about the given center point (default: origin of plane)
    pub fn rotate_around_normal(&self, angle: f64, center: Option<Point3<f64>>) -> Self {
        use nalgebra::{Rotation3, Unit};
        let axis = Unit::new_normalize(self.normal);
        let rot = Rotation3::from_axis_angle(&axis, angle);
        let c = center.unwrap_or_else(|| {
            if let PlaneOrigin::PointNormal { point, .. } = &self.origin {
                *point
            } else {
                Point3::origin() - self.normal * self.d
            }
        });

        // Rotate the reference point (if present) around the normal axis by the given angle
        let new_origin = match &self.origin {
            PlaneOrigin::PointNormal { point, offset, .. } => {
                let rel = point - c;
                let rotated = rot * rel;
                let new_point = c + rotated;
                PlaneOrigin::PointNormal { point: new_point, normal: self.normal, offset: *offset }
            },
            PlaneOrigin::ThreePoints { a, b, c: c3 } => {
                let rel_a = a - c;
                let rel_b = b - c;
                let rel_c = c3 - c;
                let new_a = c + rot * rel_a;
                let new_b = c + rot * rel_b;
                let new_c = c + rot * rel_c;
                PlaneOrigin::ThreePoints { a: new_a, b: new_b, c: new_c }
            },
            PlaneOrigin::LineAngle { point, direction, angle: orig_angle } => {
                let rel = point - c;
                let new_point = c + rot * rel;
                let new_dir = rot * direction;
                PlaneOrigin::LineAngle { point: new_point, direction: new_dir, angle: *orig_angle }
            },
            other => other.clone(),
        };

        // Recompute d if possible
        let (new_d, new_normal) = match &new_origin {
            PlaneOrigin::PointNormal { point, offset, .. } => {
                let n = self.normal.normalize();
                let d = -n.dot(&point.coords) + offset.unwrap_or(0.0);
                (d, self.normal)
            },
            PlaneOrigin::ThreePoints { a, b, c } => {
                let ab = b - a;
                let ac = c - a;
                let n = ab.cross(&ac).normalize();
                let d = -n.dot(&a.coords);
                (d, n)
            },
            PlaneOrigin::LineAngle { point, direction, angle: orig_angle } => {
                let dir = direction.normalize();
                let up = if dir.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
                let perp = dir.cross(&up).normalize();
                let n = (dir * orig_angle.cos() + perp * orig_angle.sin()).normalize();
                let d = -n.dot(&point.coords);
                (d, n)
            },
            _ => (self.d, self.normal),
        };

        let mut plane = self.clone();
        plane.origin = new_origin;
        plane.d = new_d;
        plane.normal = new_normal;
        plane.rotation += angle;
        plane
    }

    /// Returns a new plane with its normal aligned to the given vector, keeping the same point
    pub fn align_to_vector(&self, new_normal: Vector3<f64>) -> Self {
        let n = new_normal.normalize();
        let point = if let PlaneOrigin::PointNormal { point, .. } = &self.origin {
            *point
        } else {
            // Fallback: pick a point on the plane
            Point3::origin() - self.normal * self.d
        };
        let mut plane = Plane::from_point_normal(point, n, None);
        plane.rotation = self.rotation;
        plane.facing = self.facing;
        plane
    }

    /// Returns a new plane with the normal flipped (reversed direction)
    pub fn flip_normal(&self) -> Self {
        let n = -self.normal;
        let d = -self.d;
        let mut flipped = self.clone();
        flipped.normal = n;
        flipped.d = d;
        flipped.facing = !self.facing;
        flipped
    }
    /// Construct from a point and normal (optionally offset by distance along normal)
    pub fn from_point_normal(point: Point3<f64>, normal: Vector3<f64>, offset: Option<f64>) -> Self {
        let n = normal.normalize();
        let d = -n.dot(&point.coords) + offset.unwrap_or(0.0);
        Self {
            normal: n,
            d,
            origin: PlaneOrigin::PointNormal { point, normal, offset },
            rotation: 0.0,
            facing: true,
            visible: true,
            render_mode: PlaneRenderMode::Simple,
        }
    }

    /// Construct from three non-collinear points
    pub fn from_points(a: Point3<f64>, b: Point3<f64>, c: Point3<f64>) -> Option<Self> {
        let ab = b - a;
        let ac = c - a;
        let n = ab.cross(&ac);
        if n.norm() < 1e-10 {
            return None; // Degenerate
        }
        let mut plane = Self::from_point_normal(a, n, None);
        plane.origin = PlaneOrigin::ThreePoints { a, b, c };
        plane.rotation = 0.0;
        plane.facing = true;
        plane.visible = true;
        plane.render_mode = PlaneRenderMode::Simple;
        Some(plane)
    }

    /// Construct from a line (point + direction) and an angle (radians) from the direction
    pub fn from_line_angle(point: Point3<f64>, direction: Vector3<f64>, angle: f64) -> Self {
        let dir = direction.normalize();
        let up = if dir.x.abs() < 0.9 { Vector3::x() } else { Vector3::y() };
        let perp = dir.cross(&up).normalize();
        let normal = (dir * angle.cos() + perp * angle.sin()).normalize();
        let mut plane = Self::from_point_normal(point, normal, None);
        plane.origin = PlaneOrigin::LineAngle { point, direction, angle };
        plane.rotation = 0.0;
        plane.facing = true;
        plane.visible = true;
        plane.render_mode = PlaneRenderMode::Simple;
        plane
    }

    /// Construct the XY plane (z=0)
    pub fn xy() -> Self {
        Self {
            normal: Vector3::z(),
            d: 0.0,
            origin: PlaneOrigin::StandardXY,
            rotation: 0.0,
            facing: true,
            visible: true,
            render_mode: PlaneRenderMode::Simple,
        }
    }

    /// Construct the YZ plane (x=0)
    pub fn yz() -> Self {
        Self {
            normal: Vector3::x(),
            d: 0.0,
            origin: PlaneOrigin::StandardYZ,
            rotation: 0.0,
            facing: true,
            visible: true,
            render_mode: PlaneRenderMode::Simple,
        }
    }

    /// Construct the ZX plane (y=0)
    pub fn zx() -> Self {
        Self {
            normal: Vector3::y(),
            d: 0.0,
            origin: PlaneOrigin::StandardZX,
            rotation: 0.0,
            facing: true,
            visible: true,
            render_mode: PlaneRenderMode::Simple,
        }
    }
    /// Render the plane using Bevy gizmos, with mode and visibility toggle
    pub fn render(&self, gizmos: &mut Gizmos) {
        if !self.visible {
            return;
        }
        // Pick a color and style based on mode
        let (color, alpha) = match self.render_mode {
            PlaneRenderMode::Simple => (CYAN, 0.5),
            PlaneRenderMode::Ghosted => (GREEN, 0.15),
            PlaneRenderMode::Highlighted => (YELLOW, 0.7),
            PlaneRenderMode::Grid => (MAGENTA, 0.3),
        };
        // Draw a quad in the plane (centered at origin or construction point)
        let center = if let PlaneOrigin::PointNormal { point, .. } = &self.origin {
            *point
        } else {
            Point3::origin() - self.normal * self.d
        };
        // Get two perpendicular axes in the plane
        let n = self.normal.normalize();
        let u = if n.x.abs() < 0.9 {
            n.cross(&Vector3::x()).normalize()
        } else {
            n.cross(&Vector3::y()).normalize()
        };
        let v = n.cross(&u).normalize();
        let size = 100.0; // TODO: parameterize
        let corners = [
            center + u * size + v * size,
            center - u * size + v * size,
            center - u * size - v * size,
            center + u * size - v * size,
        ];
        // Draw quad outline
        for i in 0..4 {
            gizmos.line(
                na_vec3_to_bevy(&(corners[i].coords)),
                na_vec3_to_bevy(&(corners[(i + 1) % 4].coords)),
                color.with_alpha(alpha),
            );
        }
        // Optionally draw grid
        if self.render_mode == PlaneRenderMode::Grid {
            let steps = 10;
            for i in -steps..=steps {
                let t = i as f64 / steps as f64 * size;
                // u lines
                gizmos.line(
                    na_vec3_to_bevy(&((center + u * t + v * size).coords)),
                    na_vec3_to_bevy(&((center + u * t - v * size).coords)),
                    color.with_alpha(alpha * 0.7),
                );
                // v lines
                gizmos.line(
                    na_vec3_to_bevy(&((center + v * t + u * size).coords)),
                    na_vec3_to_bevy(&((center + v * t - u * size).coords)),
                    color.with_alpha(alpha * 0.7),
                );
            }
        }
    }
    

    /// Signed distance from a point to the plane
    pub fn distance(&self, point: &Point3<f64>) -> f64 {
        self.normal.dot(&point.coords) + self.d
    }
}

