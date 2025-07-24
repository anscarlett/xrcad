//! Topological elements of the B-rep structure
//! 
//! This module contains the topological hierarchy:
//! - Body: Complete 3D object (top level)
//! - Shell: Connected collection of faces
//! - Face: Bounded surface region
//! - Loop: Closed chain of edges forming face boundary
//! - Edge: Curve segment between vertices
//! - Vertex: Point in the topology

pub mod body;
pub mod shell;
pub mod face;
pub mod edge;
pub mod vertex;
pub mod edge_loop;
pub mod plane;

// Re-export commonly used types
pub use body::{Body, BodyType, ShellRef};
pub use shell::{Shell, ShellOrientation, FaceRef, EdgeRef};
// Import IDs from centralized location
pub use crate::model::brep::id::{BodyId, ShellId, FaceId, EdgeId, VertexId};
// TODO: Re-export when implemented
// pub use face::Face;
// pub use edge::Edge;
// pub use vertex::Vertex;
