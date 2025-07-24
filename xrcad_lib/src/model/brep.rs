//! Boundary Representation (B-rep) modeling for XRCAD
//!
//! This module implements B-rep as a modeling approach within the broader model system.
//! B-rep represents solid objects by their boundaries, separating topology from geometry.

// Core ID system
pub mod id;

// Topology modules
pub mod topology;

// Geometry modules  
pub mod geometry;

// TODO: Add these modules when they're needed
// pub mod operations;
// pub mod constraints;

// Re-export ID types for convenient access
pub use id::*;

// Re-export topology types for convenient access
pub mod topology_exports {
    pub use super::topology::body::{Body, BodyType};
    pub use super::topology::shell::{Shell, ShellOrientation};
    pub use super::id::{BodyId, ShellId, FaceId, EdgeId, VertexId};
    // TODO: Add other topology exports when implemented
    // pub use super::topology::face::Face;
    // pub use super::topology::edge::Edge;
    // pub use super::topology::vertex::Vertex;
}

// Re-export geometry types for convenient access
pub mod geometry_exports {
    pub use super::id::{PointId, LineId, CircleId, SurfaceId, CurveId};
    // TODO: Add geometry exports when implemented
    // pub use super::geometry::point::Point;
    // pub use super::geometry::line::Line;
    // pub use super::geometry::circle::Circle;
}

// Main exports - include working types and IDs
pub use topology_exports::*;
// TODO: Uncomment when geometry types are implemented
// pub use geometry_exports::*;