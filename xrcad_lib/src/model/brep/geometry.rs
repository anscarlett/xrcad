//! Geometric elements of the B-rep structure
//!
//! This module contains the geometric objects that topology references:
//! - Point: 3D coordinates
//! - Curve: 1D geometric shapes (lines, circles, splines)
//! - Surface: 2D geometric shapes (planes, cylinders, NURBS surfaces)

pub mod point;
pub mod line;
pub mod circle;
pub mod polygon;
pub mod rectangle;

// Import IDs from centralized location
pub use crate::model::brep::id::{PointId, LineId, CircleId, SurfaceId, CurveId};
// TODO: Re-export when implemented
// pub use point::Point;
// pub use line::Line;
// pub use circle::Circle;
