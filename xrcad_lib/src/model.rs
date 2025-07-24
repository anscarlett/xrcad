//! Model system for XRCAD
//!
//! This module contains different modeling approaches and organizational structures:
//! - brep: Boundary representation modeling
//! - geometry: Fundamental geometric primitives and operations
//! - material: Material properties system
//! - body_properties: Properties that attach to geometric objects
//! - plane_utils: Shared utilities for plane operations
//! - assembly: Hierarchical organization of model objects (future)

pub mod brep;
pub mod brep_model;
pub mod geometry;
pub mod material;
pub mod body_properties;
pub mod plane_utils;
pub mod primitives;

#[cfg(test)]


// Re-export commonly used types
pub use brep::{Body, BodyId, BodyType, Shell, ShellId, ShellOrientation};
pub use geometry::{CorePlane, BrepPlane, ConstructionPlane, PlaneRenderMode, PlaneOrigin};
pub use material::Material;
pub use body_properties::{BodyProperties, BodyPropertiesCollection};
