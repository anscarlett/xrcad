// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::core::topo::face



#[derive(Debug, Default, Clone)]
pub struct Face{
    pub id: usize,
    pub edge_loops: Vec<usize>,
}

impl Face {
    pub fn new(id: usize, edge_loops: Vec<usize>) -> Self {
        Self { id, edge_loops }
    }
    // ...other inherent methods...
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;
    use crate::model::brep::topology::{edge_loop::EdgeLoop,edge::Edge};
    #[test]
    fn test_face_new() {
        let _vertpool = vec![
            Point3::new(0.0, 0.0, 0.0), 
            Point3::new(1.0, 0.0, 0.0), 
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            ];
        let edgepool = vec![
            Edge::new(0, 0, 1),
            Edge::new(1, 1, 2),
            Edge::new(2, 2, 3),
            Edge::new(3, 3, 0),
        ];
        let edge_loop = EdgeLoop::new(1, vec![edgepool.iter().map(|e| e.id).collect::<Vec<usize>>()]);
        let _ = Face::new(1, vec![edge_loop.id]);
        // Add more meaningful tests as needed
    }
}
