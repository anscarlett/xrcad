// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::core::topo::edge


#[derive(Debug, Default, Clone)]
pub struct Edge{
    pub id: usize,
    pub vertices: (usize, usize), // IDs of the start and end vertices
}

impl Edge {
    pub fn new(id: usize, start: usize, end: usize) -> Self {
        Self { id, vertices: (start, end) }
    }
    // ...other inherent methods...
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point3;
    #[test]
    fn test_edge_new() {
        let _vertpool = vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
        let _ = Edge::new(1, 0, 1);
        // Add more meaningful tests as needed
    }
}
