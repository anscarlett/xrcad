//! Workbench helpers for XRCAD
//!
//! This module provides helper objects and visual aids for the workbench:
//! - Axes visualization
//! - Grid systems
//! - Coordinate system indicators
//! - Markers and origin points
//! - Reference planes

use bevy::prelude::*;

pub mod axes;
pub mod coordinate_system;
pub mod grid;
pub mod marker;
pub mod origin;
pub mod plane;

// Re-export commonly used types
pub use axes::*;
pub use coordinate_system::*;
pub use grid::*;
pub use marker::*;
pub use origin::*;
pub use plane::{ConstructionPlane, ConstructionPlaneRenderMode};

/// System to render workbench helpers (axes, planes, etc.)
pub fn workbench_render_system(
    mut gizmos: Gizmos,
    // TODO: Add workbench resource when available
) {
    // Example: render coordinate axes
    let axis_length = 100.0;
    gizmos.line(Vec3::ZERO, Vec3::X * axis_length, Color::srgb(1.0, 0.0, 0.0)); // X axis - red
    gizmos.line(Vec3::ZERO, Vec3::Y * axis_length, Color::srgb(0.0, 1.0, 0.0)); // Y axis - green
    gizmos.line(Vec3::ZERO, Vec3::Z * axis_length, Color::srgb(0.0, 0.0, 1.0)); // Z axis - blue
}

