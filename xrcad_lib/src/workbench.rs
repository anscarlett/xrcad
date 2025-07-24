// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Workbench system for XRCAD
//!
//! The workbench is the primary working environment for CAD operations.
//! It manages:
//! - Helper objects (axes, grids, coordinate systems)
//! - Visual aids and guides
//! - Reference systems
//! - Construction planes

pub mod helpers;

// Re-export commonly used types
pub use helpers::{
    axes::Axes,
    coordinate_system::CoordinateSystem, 
    grid::Grid,
    marker::Marker,
    origin::Origin,
    plane::{ConstructionPlane as HelperConstructionPlane, ConstructionPlaneRenderMode},
};

use bevy::ecs::resource::Resource;
use bevy::ecs::system::Res;
use bevy::gizmos::gizmos::Gizmos;
use crate::model::geometry::{ConstructionPlane, PlaneRenderMode};

#[derive(Debug, Clone)]
pub enum HelperKind {
    Axes(Axes),
    CoordinateSystem(CoordinateSystem),
    Grid(Grid),
    Marker(Marker),
    Origin(Origin),
    Plane(ConstructionPlane),
}

#[derive(Debug, Clone)]
pub struct WorkbenchHelper {
    pub id: String,
    pub kind: HelperKind,
}

#[derive(Resource)]
pub struct Workbench {
    pub helpers: Vec<WorkbenchHelper>,
}

impl Default for Workbench {
    fn default() -> Self {
        let mut wb = Workbench { helpers: Vec::new() };
        wb.add_helper("coordinate_system", HelperKind::CoordinateSystem(CoordinateSystem::default()));
        wb.add_helper("axes", HelperKind::Axes(Axes::default()));
        wb.add_helper("grid", HelperKind::Grid(Grid::default()));
        
        // Add default construction planes
        let default_plane_names = ["front", "right", "top"];
        for name in default_plane_names {
            wb.add_helper(name, HelperKind::Plane(ConstructionPlane::default()));
        }
        wb
    }
}

impl Workbench {
    pub fn new() -> Self {
        Workbench {
            helpers: Vec::new(),
        }
    }
    pub fn add_helper(&mut self, id: impl Into<String>, kind: HelperKind) {
        self.helpers.push(WorkbenchHelper {
            id: id.into(),
            kind,
        });
    }

    pub fn workbench_render_system(
        mut gizmos: Gizmos,
        workbench: Res<Workbench>,
    ) {
        for helper in &workbench.helpers {
            match &helper.kind {
                HelperKind::Axes(axes) => axes.render(&mut gizmos),
                HelperKind::Plane(plane) => plane.render(&mut gizmos),
                _ => {}
            }
        }
    }
    
    /// Set the render mode of a helper plane by id
    pub fn set_plane_render_mode(&mut self, id: &str, mode: PlaneRenderMode) {
        for helper in &mut self.helpers {
            if helper.id == id {
                if let HelperKind::Plane(plane) = &mut helper.kind {
                    plane.set_render_mode(mode);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_workbench_new() {
        let wb = Workbench::new();
        let _ = wb;
    }
}
