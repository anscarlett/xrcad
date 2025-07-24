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
use crate::color::{RED as xrcad_red, GREEN as xrcad_green, BLUE as xrcad_blue};

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
        
        // Add the three standard construction planes with colors and wireframe mode
        let mut plane_xy = ConstructionPlane::xy();
        plane_xy.set_render_mode(PlaneRenderMode::Grid);
        plane_xy.set_render_color_alpha(xrcad_red, 0.15);
        wb.add_helper("plane_xy", HelperKind::Plane(plane_xy));
        
        let mut plane_xz = ConstructionPlane::xz();
        plane_xz.set_render_mode(PlaneRenderMode::Grid);
        plane_xz.set_render_color_alpha(xrcad_green, 0.15);
        wb.add_helper("plane_xz", HelperKind::Plane(plane_xz));
        
        let mut plane_yz = ConstructionPlane::yz();
        plane_yz.set_render_mode(PlaneRenderMode::Grid);
        plane_yz.set_render_color_alpha(xrcad_blue, 0.15);
        wb.add_helper("plane_yz", HelperKind::Plane(plane_yz));
        
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

    pub fn set_plane_render_colour(&mut self, id: &str, color: bevy::prelude::Color) {
        for helper in &mut self.helpers {
            if helper.id == id {
                if let HelperKind::Plane(plane) = &mut helper.kind {
                    plane.set_render_color(color);
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
