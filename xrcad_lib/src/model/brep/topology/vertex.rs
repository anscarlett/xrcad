// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::core::topo::vertex

use nalgebra::Vector3;

#[derive(Debug, Default, Clone)]
pub struct Vertex{
    pub id: usize,
    pub position: Vector3<f64>,
}

impl Vertex {
    pub fn new() -> Self {
        Self {
            id: 0,
            position: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vertex_new() {
        let _ = Vertex::new();
        // Add more meaningful tests as needed
    }
}
