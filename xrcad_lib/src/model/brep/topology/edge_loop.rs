// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::core::topo::edge_loop

#[derive(Debug, Default, Clone)]
pub struct EdgeLoop{
    pub id: usize,
    pub edges: Vec<Vec<usize>>,
}

impl EdgeLoop {
    pub fn new(id: usize, edges: Vec<Vec<usize>>) -> Self {
        Self { id, edges }
    }
    // ...other inherent methods...
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_loop_new() {
        let _ = EdgeLoop::new(1, vec![vec![1, 2, 3]]);
        // Add more meaningful tests as needed
    }
}
