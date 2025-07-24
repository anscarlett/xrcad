//! Geometric primitives for CAD modeling
//!
//! This module provides functions to generate common geometric shapes
//! as BREP (Boundary Representation) structures suitable for CAD operations.

pub mod platonic;
pub mod basic;

// Re-export commonly used types and functions
pub use platonic::{PrimitiveResult, cube, tetrahedron, octahedron, icosahedron, dodecahedron};
pub use basic::{box_primitive, cylinder, sphere, cone};
