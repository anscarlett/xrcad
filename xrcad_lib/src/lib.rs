// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! xrcad core library

// Core modules
pub mod color;
pub mod input;
pub mod interaction;
pub mod model;
pub mod render;
pub mod viewport;
pub mod workbench;

// Re-exports for ergonomic use
pub use color::*;
pub use model::brep::{Body, BodyId, BodyType, Shell, ShellId, ShellOrientation};
pub use model::geometry::{CorePlane, BrepPlane, ConstructionPlane, PlaneRenderMode, PlaneOrigin};
pub use model::{Material, BodyProperties, BodyPropertiesCollection, plane_utils};
pub use model::primitives::{PrimitiveResult, cube, tetrahedron, octahedron, box_primitive, cylinder, sphere, cone};
pub use workbench::{helpers, Workbench, WorkbenchHelper, HelperKind};


