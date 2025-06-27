// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: workspace

     

use bevy::ecs::resource::Resource;
use bevy::ecs::system::Res;
use bevy::gizmos::gizmos::Gizmos;
use super::helpers::axes::Axes;
use super::helpers::coordinate_system::CoordinateSystem;
use super::helpers::grid::Grid;
use super::helpers::marker::Marker;
use super::helpers::origin::Origin;
use crate::model::brep::topology::plane::Plane;


#[derive(Debug, Clone)]
pub enum HelperKind {
    Axes(Axes),
    CoordinateSystem(CoordinateSystem),
    Grid(Grid),
    Marker(Marker),
    Origin(Origin),
    Plane(Plane),
}

#[derive(Debug, Clone)]
pub struct WorkspaceHelper {
    pub id: String,
    pub kind: HelperKind,
}

#[derive(Resource)]
pub struct Workspace {
    pub helpers: Vec<WorkspaceHelper>,
}

impl Default for Workspace {
    fn default() -> Self {
        let mut ws = Workspace { helpers: Vec::new() };
        ws.add_helper("coordinate_system", HelperKind::CoordinateSystem(CoordinateSystem::default()));
        ws.add_helper("axes", HelperKind::Axes(Axes::default()));
        ws.add_helper("grid", HelperKind::Grid(Grid::default()));
        let default_plane_names = ["front", "right", "top"];
        for name in default_plane_names {
            ws.add_helper(name, HelperKind::Plane(Plane::default()));
        }
        ws
    }
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            helpers: Vec::new(),
        }
    }
    pub fn add_helper(&mut self, id: impl Into<String>, kind: HelperKind) {
        self.helpers.push(WorkspaceHelper {
            id: id.into(),
            kind,
        });
    }

    pub fn workspace_render_system(
        mut gizmos: Gizmos,
        workspace: Res<Workspace>,
    ) {
        for helper in &workspace.helpers {
            match &helper.kind {
                HelperKind::Axes(axes) => axes.render(&mut gizmos),
                HelperKind::Plane(plane) => plane.render(&mut gizmos),
                _ => {}
            }
        }
    }
    /// Set the render mode of a helper plane by id
    pub fn set_plane_render_mode(&mut self, id: &str, mode: crate::model::brep::topology::plane::PlaneRenderMode) {
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
    fn test_workspace_new() {
        let w = Workspace::new();
        let _ = w;
    }
}
