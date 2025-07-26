```rust

// =====================================================
// File: src/lib.rs
// =====================================================

pub mod geometry;
pub mod topology;

// Re-export commonly used items
pub use geometry::*;
pub use topology::*;

// =====================================================
// File: src/geometry/mod.rs
// =====================================================

pub mod traits_2d;
pub mod traits_3d;
pub mod points;
pub mod curves_2d;
pub mod curves_3d;
pub mod surfaces_3d;
pub mod transformations;
pub mod tessellation;

// Re-export geometric primitives
pub use traits_2d::*;
pub use traits_3d::*;
pub use points::*;
pub use curves_2d::*;
pub use curves_3d::*;
pub use surfaces_3d::*;
pub use transformations::*;
pub use tessellation::*;

// =====================================================
// File: src/geometry/traits_2d.rs
// =====================================================

use nalgebra::{Point2, Vector2};

/// Core trait for all 2D geometric curves
pub trait Curve2D {
    /// Get the actual start point where the curve begins
    fn start_point(&self) -> Point2<f64>;
    
    /// Get the actual end point where the curve ends
    fn end_point(&self) -> Point2<f64>;
    
    /// Check if this is a closed curve (start == end)
    fn is_closed(&self) -> bool {
        let start = self.start_point();
        let end = self.end_point();
        (start - end).norm() < 1e-10
    }
    
    /// Get the parameter range (typically 0.0 to 1.0)
    fn parameter_range(&self) -> (f64, f64);
    
    /// Evaluate point on curve at given parameter
    fn evaluate_at(&self, t: f64) -> Point2<f64>;
    
    /// Get tangent vector at parameter t
    fn tangent_at(&self, t: f64) -> Vector2<f64>;
    
    /// Get curve degree/order
    fn degree(&self) -> usize;
    
    /// Get approximate arc length
    fn arc_length(&self) -> f64;
    
    /// Get curvature at parameter t
    fn curvature_at(&self, t: f64) -> f64 { 0.0 }
    
    /// Get geometric bounding box
    fn bounding_box(&self) -> (Point2<f64>, Point2<f64>) {
        let start = self.start_point();
        let end = self.end_point();
        (
            Point2::new(start.x.min(end.x), start.y.min(end.y)),
            Point2::new(start.x.max(end.x), start.y.max(end.y))
        )
    }
}

/// Trait for 2D geometric transformations
pub trait Transformable2D {
    /// Apply 2D transformation matrix
    fn transform(&mut self, matrix: &nalgebra::Matrix3<f64>);
    
    /// Translate by vector
    fn translate(&mut self, offset: Vector2<f64>) {
        let translation = nalgebra::Matrix3::new(
            1.0, 0.0, offset.x,
            0.0, 1.0, offset.y,
            0.0, 0.0, 1.0
        );
        self.transform(&translation);
    }
    
    /// Rotate around point
    fn rotate(&mut self, angle: f64, center: Point2<f64>) {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        let to_origin = nalgebra::Matrix3::new(
            1.0, 0.0, -center.x,
            0.0, 1.0, -center.y,
            0.0, 0.0, 1.0
        );
        
        let rotation = nalgebra::Matrix3::new(
            cos_a, -sin_a, 0.0,
            sin_a, cos_a, 0.0,
            0.0, 0.0, 1.0
        );
        
        let from_origin = nalgebra::Matrix3::new(
            1.0, 0.0, center.x,
            0.0, 1.0, center.y,
            0.0, 0.0, 1.0
        );
        
        let combined = from_origin * rotation * to_origin;
        self.transform(&combined);
    }
    
    /// Scale around point
    fn scale(&mut self, factor: f64, center: Point2<f64>) {
        let to_origin = nalgebra::Matrix3::new(
            1.0, 0.0, -center.x,
            0.0, 1.0, -center.y,
            0.0, 0.0, 1.0
        );
        
        let scaling = nalgebra::Matrix3::new(
            factor, 0.0, 0.0,
            0.0, factor, 0.0,
            0.0, 0.0, 1.0
        );
        
        let from_origin = nalgebra::Matrix3::new(
            1.0, 0.0, center.x,
            0.0, 1.0, center.y,
            0.0, 0.0, 1.0
        );
        
        let combined = from_origin * scaling * to_origin;
        self.transform(&combined);
    }
}

/// Trait for 2D geometric analysis
pub trait Analyzable2D {
    /// Calculate area (for closed curves)
    fn area(&self) -> Option<f64> { None }
    
    /// Calculate centroid
    fn centroid(&self) -> Option<Point2<f64>> { None }
    
    /// Calculate moment of inertia
    fn moment_of_inertia(&self) -> Option<f64> { None }
    
    /// Check if point is inside (for closed curves)
    fn contains_point(&self, point: Point2<f64>) -> bool { false }
}

/// Trait for 2D tessellation/discretization
pub trait Tessellatable2D {
    /// Tessellate to line segments for rendering/analysis
    fn tessellate(&self, tolerance: f64) -> Vec<Point2<f64>>;
    
    /// Get adaptive tessellation based on curvature
    fn adaptive_tessellate(&self, max_error: f64, min_segments: usize, max_segments: usize) -> Vec<Point2<f64>>;
}

// =====================================================
// File: src/geometry/traits_3d.rs
// =====================================================

use nalgebra::{Point3, Vector3};

/// Core trait for all 3D geometric curves
pub trait Curve3D {
    fn start_point(&self) -> Point3<f64>;
    fn end_point(&self) -> Point3<f64>;
    fn is_closed(&self) -> bool {
        let start = self.start_point();
        let end = self.end_point();
        (start - end).norm() < 1e-10
    }
    fn parameter_range(&self) -> (f64, f64);
    fn evaluate_at(&self, t: f64) -> Point3<f64>;
    fn tangent_at(&self, t: f64) -> Vector3<f64>;
    fn degree(&self) -> usize;
    fn arc_length(&self) -> f64;
    fn curvature_at(&self, t: f64) -> f64 { 0.0 }
    fn bounding_box(&self) -> (Point3<f64>, Point3<f64>) {
        let start = self.start_point();
        let end = self.end_point();
        (
            Point3::new(start.x.min(end.x), start.y.min(end.y), start.z.min(end.z)),
            Point3::new(start.x.max(end.x), start.y.max(end.y), start.z.max(end.z))
        )
    }
}

/// Interface for 3D surfaces (geometric definition)
pub trait Surface3D {
    /// Evaluate point on surface at parameters (u, v)
    fn evaluate_at(&self, u: f64, v: f64) -> Point3<f64>;
    
    /// Get normal vector at parameters (u, v)
    fn normal_at(&self, u: f64, v: f64) -> Vector3<f64>;
    
    /// Get parameter ranges
    fn parameter_ranges(&self) -> ((f64, f64), (f64, f64));
    
    /// Get surface area
    fn area(&self) -> f64;
    
    /// Get bounding box
    fn bounding_box(&self) -> (Point3<f64>, Point3<f64>);
    
    /// Get principal curvatures at point
    fn principal_curvatures(&self, u: f64, v: f64) -> (f64, f64) { (0.0, 0.0) }
}

/// Trait for 3D geometric transformations
pub trait Transformable3D {
    /// Apply 3D transformation matrix
    fn transform(&mut self, matrix: &nalgebra::Matrix4<f64>);
    
    /// Translate by vector
    fn translate(&mut self, offset: Vector3<f64>) {
        let translation = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, offset.x,
            0.0, 1.0, 0.0, offset.y,
            0.0, 0.0, 1.0, offset.z,
            0.0, 0.0, 0.0, 1.0
        );
        self.transform(&translation);
    }
    
    /// Rotate around axis
    fn rotate(&mut self, axis: Vector3<f64>, angle: f64, center: Point3<f64>) {
        let rotation = nalgebra::Matrix4::from_axis_angle(&nalgebra::Unit::new_normalize(axis), angle);
        
        let to_origin = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, -center.x,
            0.0, 1.0, 0.0, -center.y,
            0.0, 0.0, 1.0, -center.z,
            0.0, 0.0, 0.0, 1.0
        );
        
        let from_origin = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, center.x,
            0.0, 1.0, 0.0, center.y,
            0.0, 0.0, 1.0, center.z,
            0.0, 0.0, 0.0, 1.0
        );
        
        let combined = from_origin * rotation * to_origin;
        self.transform(&combined);
    }
    
    /// Scale around point
    fn scale(&mut self, factor: f64, center: Point3<f64>) {
        let to_origin = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, -center.x,
            0.0, 1.0, 0.0, -center.y,
            0.0, 0.0, 1.0, -center.z,
            0.0, 0.0, 0.0, 1.0
        );
        
        let scaling = nalgebra::Matrix4::new(
            factor, 0.0, 0.0, 0.0,
            0.0, factor, 0.0, 0.0,
            0.0, 0.0, factor, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        
        let from_origin = nalgebra::Matrix4::new(
            1.0, 0.0, 0.0, center.x,
            0.0, 1.0, 0.0, center.y,
            0.0, 0.0, 1.0, center.z,
            0.0, 0.0, 0.0, 1.0
        );
        
        let combined = from_origin * scaling * to_origin;
        self.transform(&combined);
    }
}

/// Trait for 3D geometric analysis
pub trait Analyzable3D {
    /// Calculate volume (for closed surfaces/solids)
    fn volume(&self) -> Option<f64> { None }
    
    /// Calculate surface area
    fn surface_area(&self) -> Option<f64> { None }
    
    /// Calculate centroid
    fn centroid(&self) -> Option<Point3<f64>> { None }
    
    /// Calculate mass properties (assuming unit density)
    fn mass_properties(&self) -> Option<MassProperties3D> { None }
}

/// Mass properties for 3D objects
#[derive(Debug, Clone)]
pub struct MassProperties3D {
    pub mass: f64,
    pub centroid: Point3<f64>,
    pub inertia_tensor: nalgebra::Matrix3<f64>,
}

/// Trait for 3D tessellation/discretization
pub trait Tessellatable3D {
    /// Tessellate to triangular mesh
    fn tessellate(&self, tolerance: f64) -> (Vec<Point3<f64>>, Vec<[usize; 3]>);
    
    /// Get adaptive tessellation based on curvature
    fn adaptive_tessellate(&self, max_error: f64) -> (Vec<Point3<f64>>, Vec<[usize; 3]>);
}

// =====================================================
// File: src/topology/mod.rs
// =====================================================

pub mod traits;
pub mod vertex;
pub mod edge;
pub mod face;
pub mod loop_topology;
pub mod shell;
pub mod solid;
pub mod wire;
pub mod constraints_2d;
pub mod constraints_3d;
pub mod relationships;

// Re-export topological structures
pub use traits::*;
pub use vertex::*;
pub use edge::*;
pub use face::*;
pub use loop_topology::*;
pub use shell::*;
pub use solid::*;
pub use wire::*;
pub use constraints_2d::*;
pub use constraints_3d::*;
pub use relationships::*;

// =====================================================
// File: src/topology/traits.rs
// =====================================================

use bevy::prelude::Entity;

/// Core topological entity trait
pub trait TopologyEntity {
    /// Get unique identifier
    fn id(&self) -> String;
    
    /// Get entity type name
    fn entity_type(&self) -> &'static str;
    
    /// Check if entity is valid
    fn is_valid(&self) -> bool;
    
    /// Get connected entities of higher dimension
    fn parents(&self) -> Vec<Entity> { Vec::new() }
    
    /// Get connected entities of lower dimension  
    fn children(&self) -> Vec<Entity> { Vec::new() }
}

/// Trait for entities that can be constrained
pub trait Constrainable {
    /// Get applied constraints
    fn constraints(&self) -> Vec<Entity>;
    
    /// Add constraint
    fn add_constraint(&mut self, constraint: Entity);
    
    /// Remove constraint
    fn remove_constraint(&mut self, constraint: Entity) -> bool;
    
    /// Check if fully constrained
    fn is_fully_constrained(&self) -> bool;
    
    /// Get degrees of freedom remaining
    fn degrees_of_freedom(&self) -> i32;
}

/// Trait for constraint evaluation
pub trait Constraint {
    /// Check if constraint is satisfied within tolerance
    fn is_satisfied(&self) -> bool;
    
    /// Get constraint error (deviation from target)
    fn error(&self) -> f64;
    
    /// Get constraint priority/weight
    fn priority(&self) -> f32 { 1.0 }
    
    /// Check if constraint conflicts with others
    fn conflicts_with(&self, other: &dyn Constraint) -> bool { false }
    
    /// Get entities involved in this constraint
    fn entities(&self) -> Vec<Entity>;
}

/// Trait for topological validation
pub trait Validatable {
    /// Check topological validity
    fn is_topologically_valid(&self) -> bool;
    
    /// Get validation errors
    fn validation_errors(&self) -> Vec<ValidationError>;
    
    /// Attempt to repair topology
    fn repair(&mut self) -> Result<(), String> { Err("Repair not implemented".to_string()) }
}

/// Validation error types
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Disconnected edges in loop
    DisconnectedEdges,
    /// Wrong winding direction
    InvalidWinding,
    /// Self-intersecting geometry
    SelfIntersection,
    /// Invalid face orientation
    InvalidOrientation,
    /// Missing geometric backing
    MissingGeometry,
}

/// Trait for hierarchical relationships
pub trait Hierarchical {
    /// Get parent entity
    fn parent(&self) -> Option<Entity>;
    
    /// Get child entities
    fn children(&self) -> Vec<Entity>;
    
    /// Add child entity
    fn add_child(&mut self, child: Entity);
    
    /// Remove child entity
    fn remove_child(&mut self, child: Entity) -> bool;
    
    /// Check if entity is ancestor of another
    fn is_ancestor_of(&self, other: Entity) -> bool { false }
}

// =====================================================
// File: src/topology/vertex.rs
// =====================================================

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use nalgebra::{Point2, Point3};
use crate::topology::traits::*;

/// Topological vertex - 0D entity
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Vertex {
    pub id: String,
    /// Reference to geometric point
    pub geometry_2d: Option<Point2<f64>>,
    pub geometry_3d: Option<Point3<f64>>,
    /// Connected edges
    pub edges: Vec<Entity>,
    /// Applied constraints
    pub constraints: Vec<Entity>,
}

impl TopologyEntity for Vertex {
    fn id(&self) -> String { self.id.clone() }
    fn entity_type(&self) -> &'static str { "Vertex" }
    fn is_valid(&self) -> bool { 
        self.geometry_2d.is_some() || self.geometry_3d.is_some() 
    }
    fn parents(&self) -> Vec<Entity> { self.edges.clone() }
}

impl Constrainable for Vertex {
    fn constraints(&self) -> Vec<Entity> { self.constraints.clone() }
    fn add_constraint(&mut self, constraint: Entity) { self.constraints.push(constraint); }
    fn remove_constraint(&mut self, constraint: Entity) -> bool {
        if let Some(pos) = self.constraints.iter().position(|&c| c == constraint) {
            self.constraints.remove(pos);
            true
        } else {
            false
        }
    }
    fn is_fully_constrained(&self) -> bool { 
        // 2D vertex needs 2 constraints, 3D needs 3
        if self.geometry_2d.is_some() {
            self.constraints.len() >= 2
        } else {
            self.constraints.len() >= 3
        }
    }
    fn degrees_of_freedom(&self) -> i32 {
        if self.geometry_2d.is_some() {
            2 - self.constraints.len() as i32
        } else {
            3 - self.constraints.len() as i32
        }
    }
}

impl Validatable for Vertex {
    fn is_topologically_valid(&self) -> bool {
        self.geometry_2d.is_some() || self.geometry_3d.is_some()
    }
    
    fn validation_errors(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.geometry_2d.is_none() && self.geometry_3d.is_none() {
            errors.push(ValidationError::MissingGeometry);
        }
        errors
    }
}

// =====================================================
// File: src/topology/edge.rs
// =====================================================

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::topology::traits::*;
use crate::geometry::edges_2d::Edge2D;
use crate::geometry::edges_3d::Edge3D;

/// Topological edge - 1D entity connecting two vertices
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    /// Start and end vertices
    pub vertices: [Entity; 2],
    /// Reference to geometric curve
    pub geometry_2d: Option<Edge2D>,
    pub geometry_3d: Option<Edge3D>,
    /// Connected faces
    pub faces: Vec<Entity>,
    /// Applied constraints
    pub constraints: Vec<Entity>,
    /// Edge properties
    pub properties: EdgeProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeProperties {
    /// Whether edge is construction geometry
    pub construction: bool,
    /// Layer assignment
    pub layer: String,
    /// Visual properties
    pub visible: bool,
    pub color: [f32; 4],
    pub line_width: f32,
}

impl Default for EdgeProperties {
    fn default() -> Self {
        Self {
            construction: false,
            layer: "Default".to_string(),
            visible: true,
            color: [1.0, 1.0, 1.0, 1.0],
            line_width: 1.0,
        }
    }
}

impl TopologyEntity for Edge {
    fn id(&self) -> String { self.id.clone() }
    fn entity_type(&self) -> &'static str { "Edge" }
    fn is_valid(&self) -> bool { 
        (self.geometry_2d.is_some() || self.geometry_3d.is_some()) &&
        !self.vertices.is_empty()
    }
    fn children(&self) -> Vec<Entity> { self.vertices.to_vec() }
    fn parents(&self) -> Vec<Entity> { self.faces.clone() }
}

impl Constrainable for Edge {
    fn constraints(&self) -> Vec<Entity> { self.constraints.clone() }
    fn add_constraint(&mut self, constraint: Entity) { self.constraints.push(constraint); }
    fn remove_constraint(&mut self, constraint: Entity) -> bool {
        if let Some(pos) = self.constraints.iter().position(|&c| c == constraint) {
            self.constraints.remove(pos);
            true
        } else {
            false
        }
    }
    fn is_fully_constrained(&self) -> bool { 
        // Edge constraints depend on type (length, angle, etc.)
        !self.constraints.is_empty()
    }
    fn degrees_of_freedom(&self) -> i32 {
        // Simplified - actual DOF calculation is complex
        if self.constraints.is_empty() { 2 } else { 0 }
    }
}

impl Validatable for Edge {
    fn is_topologically_valid(&self) -> bool {
        (self.geometry_2d.is_some() || self.geometry_3d.is_some()) &&
        self.vertices.len() == 2
    }
    
    fn validation_errors(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.geometry_2d.is_none() && self.geometry_3d.is_none() {
            errors.push(ValidationError::MissingGeometry);
        }
        if self.vertices.len() != 2 {
            errors.push(ValidationError::DisconnectedEdges);
        }
        errors
    }
}

// =====================================================
// File: src/topology/loop_topology.rs
// =====================================================

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::topology::traits::*;

/// Topological loop - ordered sequence of edges forming a closed boundary
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Loop {
    pub id: String,
    /// Ordered edges that form the loop
    pub edges: Vec<Entity>,
    /// Whether loop is outer boundary or inner (hole)
    pub loop_type: LoopType,
    /// Parent face
    pub face: Option<Entity>,
    /// Applied constraints
    pub constraints: Vec<Entity>,
    /// Loop validation status
    pub is_closed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LoopType {
    /// Outer boundary - defines the external shape
    Outer,
    /// Inner boundary - defines a hole/cutout
    Inner,
    /// Construction geometry - not part of final shape
    Construction,
}

impl TopologyEntity for Loop {
    fn id(&self) -> String { self.id.clone() }
    fn entity_type(&self) -> &'static str { "Loop" }
    fn is_valid(&self) -> bool { 
        !self.edges.is_empty() && self.is_closed
    }
    fn children(&self) -> Vec<Entity> { self.edges.clone() }
    fn parents(&self) -> Vec<Entity> { 
        if let Some(face) = self.face { vec![face] } else { Vec::new() }
    }
}

impl Constrainable for Loop {
    fn constraints(&self) -> Vec<Entity> { self.constraints.clone() }
    fn add_constraint(&mut self, constraint: Entity) { self.constraints.push(constraint); }
    fn remove_constraint(&mut self, constraint: Entity) -> bool {
        if let Some(pos) = self.constraints.iter().position(|&c| c == constraint) {
            self.constraints.remove(pos);
            true
        } else {
            false
        }
    }
    fn is_fully_constrained(&self) -> bool { 
        // Loop is fully constrained if all its edges are constrained
        !self.constraints.is_empty()
    }
    fn degrees_of_freedom(&self) -> i32 {
        // Simplified calculation
        if self.is_fully_constrained() { 0 } else { self.edges.len() as i32 }
    }
}

impl Validatable for Loop {
    fn is_topologically_valid(&self) -> bool {
        !self.edges.is_empty() && self.is_closed
    }
    
    fn validation_errors(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if self.edges.is_empty() {
            errors.push(ValidationError::DisconnectedEdges);
        }
        if !self.is_closed {
            errors.push(ValidationError::DisconnectedEdges);
        }
        errors
    }
}

// =====================================================  
// File: src/topology/face.rs
// =====================================================

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::topology::traits::*;
use crate::geometry::surfaces_3d::*;

/// Topological face - 2D entity bounded by loops
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Face {
    pub id: String,
    /// Outer boundary loop
    pub outer_loop: Entity,
    /// Inner loops (holes)
    pub inner_loops: Vec<Entity>,
    /// Reference to geometric surface
    pub surface: Option<Box<dyn Surface3D>>,
    /// Connected shells/solids
    pub shells: Vec<Entity>,
    /// Applied constraints
    pub constraints: Vec<Entity>,
    /// Face properties
    pub properties: FaceProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceProperties {
    /// Material properties
    pub material_id: Option<String>,
    /// Visual properties
    pub visible: bool,
    pub color: [f32; 4],
    pub transparency: f32,
    /// Face normal orientation
    pub outward_normal: bool,
}

impl Default for FaceProperties {
    fn default() -> Self {
        Self {
            material_id: None,
            visible: true,
            color: [0.8, 0.8, 0.8, 1.0],
            transparency: 0.0,
            outward_normal: true,
        }
    }
}

impl TopologyEntity for Face {
    fn id(&self) -> String { self.id.clone() }
    fn entity_type(&self) -> &'static str { "Face" }
    fn is_valid(&self) -> bool { 
        // Face needs at least an outer loop
        true // Simplified
    }
    fn children(&self) -> Vec<Entity> { 
        let mut loops = vec![self.outer_loop];
        loops.extend(self.inner_loops.clone());
        loops
    }
    fn parents(&self) -> Vec<Entity> { self.shells.clone() }
}

impl Constrainable for Face {
    fn constraints(&self) -> Vec<Entity> { self.constraints.clone() }
    fn add_constraint(&mut self, constraint: Entity) { self.constraints.push(constraint); }
    fn remove_constraint(&mut self, constraint: Entity) -> bool {
        if let Some(pos) = self.constraints.iter().position(|&c| c == constraint) {
            self.constraints.remove(pos);
            true
        } else {
            false
        }
    }
    fn is_fully_constrained(&self) -> bool { false } // Faces are typically not directly constrained
    fn degrees_of_freedom(&self) -> i32 { 0 }
}

impl Validatable for Face {
    fn is_topologically_valid(&self) -> bool {
        // Face validation would check loop connectivity, orientation, etc.
        true
    }
    
    fn validation_errors(&self) -> Vec<ValidationError> {
        Vec::new() // Simplified
    }
}

```