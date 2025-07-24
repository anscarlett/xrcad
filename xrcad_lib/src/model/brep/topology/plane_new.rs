//! BREP geometric plane topology
//!
//! This module re-exports the unified BrepPlane from the geometry module.
//! The actual implementation has been moved to model::geometry::plane for consistency.
//! 
//! BrepPlane provides geometric operations needed for solid modeling while
//! building on the shared CorePlane foundation.

// Re-export the unified BREP plane types
pub use crate::model::geometry::{
    BrepPlane as Plane, 
    CorePlane, 
    PlaneRenderMode, 
    PlaneOrigin
};
