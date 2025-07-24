//! Construction plane helper for workbench visualization
//!
//! This module re-exports the unified ConstructionPlane from the geometry module.
//! The actual implementation has been moved to model::geometry::plane for consistency.

// Re-export the unified construction plane and render mode
pub use crate::model::geometry::{ConstructionPlane, PlaneRenderMode as ConstructionPlaneRenderMode};
