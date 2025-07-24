//! Platonic solids primitive generation
//!
//! The five Platonic solids are the only regular convex polyhedra:
//! - Tetrahedron (4 triangular faces)
//! - Cube/Hexahedron (6 square faces) 
//! - Octahedron (8 triangular faces)
//! - Dodecahedron (12 pentagonal faces)
//! - Icosahedron (20 triangular faces)

use nalgebra::Vector3;
use crate::model::brep::topology::{vertex::Vertex, edge::Edge, edge_loop::EdgeLoop, face::Face};

/// Result of primitive generation containing all BREP topology
#[derive(Debug, Clone, bevy::prelude::Resource)]
pub struct PrimitiveResult {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub edge_loops: Vec<EdgeLoop>,
    pub faces: Vec<Face>,
}

impl PrimitiveResult {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            edge_loops: Vec::new(),
            faces: Vec::new(),
        }
    }
}

/// Generate a cube (hexahedron) with given size
/// 
/// Creates a cube centered at origin with edges of length `size`
pub fn cube(size: f64) -> PrimitiveResult {
    let half = size / 2.0;
    
    // Create 8 vertices of a cube
    let vertices = vec![
        Vertex { id: 0, position: Vector3::new(-half, -half, -half) }, // Bottom face
        Vertex { id: 1, position: Vector3::new( half, -half, -half) },
        Vertex { id: 2, position: Vector3::new( half,  half, -half) },
        Vertex { id: 3, position: Vector3::new(-half,  half, -half) },
        Vertex { id: 4, position: Vector3::new(-half, -half,  half) }, // Top face
        Vertex { id: 5, position: Vector3::new( half, -half,  half) },
        Vertex { id: 6, position: Vector3::new( half,  half,  half) },
        Vertex { id: 7, position: Vector3::new(-half,  half,  half) },
    ];
    
    // Create 12 edges (4 for each face direction)
    let edges = vec![
        // Bottom face edges (z = -half)
        Edge { id: 0, vertices: (0, 1) },
        Edge { id: 1, vertices: (1, 2) },
        Edge { id: 2, vertices: (2, 3) },
        Edge { id: 3, vertices: (3, 0) },
        // Top face edges (z = +half)
        Edge { id: 4, vertices: (4, 5) },
        Edge { id: 5, vertices: (5, 6) },
        Edge { id: 6, vertices: (6, 7) },
        Edge { id: 7, vertices: (7, 4) },
        // Vertical edges connecting top and bottom
        Edge { id: 8, vertices: (0, 4) },
        Edge { id: 9, vertices: (1, 5) },
        Edge { id: 10, vertices: (2, 6) },
        Edge { id: 11, vertices: (3, 7) },
    ];
    
    // Create 6 edge loops (one for each face)
    let edge_loops = vec![
        EdgeLoop::new(0, vec![vec![0, 1, 2, 3]]),      // Bottom face (-Z)
        EdgeLoop::new(1, vec![vec![4, 5, 6, 7]]),      // Top face (+Z)
        EdgeLoop::new(2, vec![vec![0, 9, 4, 8]]),      // Front face (-Y)
        EdgeLoop::new(3, vec![vec![2, 11, 6, 10]]),    // Back face (+Y)
        EdgeLoop::new(4, vec![vec![3, 11, 7, 8]]),     // Left face (-X)
        EdgeLoop::new(5, vec![vec![1, 10, 5, 9]]),     // Right face (+X)
    ];
    
    // Create 6 faces
    let faces = vec![
        Face { id: 0, edge_loops: vec![0] }, // Bottom
        Face { id: 1, edge_loops: vec![1] }, // Top
        Face { id: 2, edge_loops: vec![2] }, // Front
        Face { id: 3, edge_loops: vec![3] }, // Back
        Face { id: 4, edge_loops: vec![4] }, // Left
        Face { id: 5, edge_loops: vec![5] }, // Right
    ];
    
    PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    }
}

/// Generate a tetrahedron with given edge length
/// 
/// Creates a regular tetrahedron centered at origin
pub fn tetrahedron(edge_length: f64) -> PrimitiveResult {
    let a = edge_length / (2.0 * 2_f64.sqrt() / 3_f64.sqrt()); // Coordinate scaling factor
    
    // 4 vertices of a regular tetrahedron
    let vertices = vec![
        Vertex { id: 0, position: Vector3::new( a,  a,  a) },
        Vertex { id: 1, position: Vector3::new(-a, -a,  a) },
        Vertex { id: 2, position: Vector3::new(-a,  a, -a) },
        Vertex { id: 3, position: Vector3::new( a, -a, -a) },
    ];
    
    // 6 edges
    let edges = vec![
        Edge { id: 0, vertices: (0, 1) },
        Edge { id: 1, vertices: (0, 2) },
        Edge { id: 2, vertices: (0, 3) },
        Edge { id: 3, vertices: (1, 2) },
        Edge { id: 4, vertices: (1, 3) },
        Edge { id: 5, vertices: (2, 3) },
    ];
    
    // 4 triangular edge loops
    let edge_loops = vec![
        EdgeLoop::new(0, vec![vec![0, 3, 1]]), // Face 0-1-2
        EdgeLoop::new(1, vec![vec![0, 4, 2]]), // Face 0-1-3
        EdgeLoop::new(2, vec![vec![1, 5, 2]]), // Face 0-2-3
        EdgeLoop::new(3, vec![vec![3, 4, 5]]), // Face 1-2-3
    ];
    
    // 4 triangular faces
    let faces = vec![
        Face { id: 0, edge_loops: vec![0] },
        Face { id: 1, edge_loops: vec![1] },
        Face { id: 2, edge_loops: vec![2] },
        Face { id: 3, edge_loops: vec![3] },
    ];
    
    PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    }
}

/// Generate an octahedron with given edge length
///
/// Creates a regular octahedron centered at origin
pub fn octahedron(edge_length: f64) -> PrimitiveResult {
    let a = edge_length / 2_f64.sqrt(); // Distance from center to vertex
    
    // 6 vertices of a regular octahedron
    let vertices = vec![
        Vertex { id: 0, position: Vector3::new( a,  0.0,  0.0) }, // +X
        Vertex { id: 1, position: Vector3::new(-a,  0.0,  0.0) }, // -X
        Vertex { id: 2, position: Vector3::new( 0.0,  a,  0.0) }, // +Y
        Vertex { id: 3, position: Vector3::new( 0.0, -a,  0.0) }, // -Y
        Vertex { id: 4, position: Vector3::new( 0.0,  0.0,  a) }, // +Z
        Vertex { id: 5, position: Vector3::new( 0.0,  0.0, -a) }, // -Z
    ];
    
    // 12 edges
    let edges = vec![
        Edge { id: 0, vertices: (0, 2) }, Edge { id: 1, vertices: (2, 1) },
        Edge { id: 2, vertices: (1, 3) }, Edge { id: 3, vertices: (3, 0) },
        Edge { id: 4, vertices: (0, 4) }, Edge { id: 5, vertices: (2, 4) },
        Edge { id: 6, vertices: (1, 4) }, Edge { id: 7, vertices: (3, 4) },
        Edge { id: 8, vertices: (0, 5) }, Edge { id: 9, vertices: (2, 5) },
        Edge { id: 10, vertices: (1, 5) }, Edge { id: 11, vertices: (3, 5) },
    ];
    
    // 8 triangular edge loops (one for each face)
    let edge_loops = vec![
        EdgeLoop::new(0, vec![vec![0, 5, 4]]),   // Top-front-right
        EdgeLoop::new(1, vec![vec![5, 1, 6]]),   // Top-front-left
        EdgeLoop::new(2, vec![vec![1, 2, 6]]),   // Top-back-left
        EdgeLoop::new(3, vec![vec![2, 0, 4]]),   // Top-back-right
        EdgeLoop::new(4, vec![vec![3, 7, 8]]),   // Bottom-front-right
        EdgeLoop::new(5, vec![vec![7, 6, 10]]),  // Bottom-front-left
        EdgeLoop::new(6, vec![vec![6, 9, 10]]),  // Bottom-back-left
        EdgeLoop::new(7, vec![vec![9, 3, 11]]),  // Bottom-back-right
    ];
    
    // 8 triangular faces
    let faces = (0..8).map(|i| Face { id: i, edge_loops: vec![i] }).collect();
    
    PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    }
}

/// Generate an icosahedron with given edge length (simplified version)
///
/// Creates a regular icosahedron centered at origin
/// Note: This is a complex geometry - simplified implementation
pub fn icosahedron(edge_length: f64) -> PrimitiveResult {
    let phi = (1.0 + 5_f64.sqrt()) / 2.0; // Golden ratio
    let a = edge_length / (2.0 * phi.sin());
    
    // 12 vertices arranged in 3 perpendicular golden rectangles
    let vertices = vec![
        // Rectangle in YZ plane
        Vertex { id: 0, position: Vector3::new(0.0, a, a * phi) },
        Vertex { id: 1, position: Vector3::new(0.0, -a, a * phi) },
        Vertex { id: 2, position: Vector3::new(0.0, a, -a * phi) },
        Vertex { id: 3, position: Vector3::new(0.0, -a, -a * phi) },
        // Rectangle in XZ plane
        Vertex { id: 4, position: Vector3::new(a * phi, 0.0, a) },
        Vertex { id: 5, position: Vector3::new(-a * phi, 0.0, a) },
        Vertex { id: 6, position: Vector3::new(a * phi, 0.0, -a) },
        Vertex { id: 7, position: Vector3::new(-a * phi, 0.0, -a) },
        // Rectangle in XY plane
        Vertex { id: 8, position: Vector3::new(a, a * phi, 0.0) },
        Vertex { id: 9, position: Vector3::new(-a, a * phi, 0.0) },
        Vertex { id: 10, position: Vector3::new(a, -a * phi, 0.0) },
        Vertex { id: 11, position: Vector3::new(-a, -a * phi, 0.0) },
    ];
    
    // For brevity, creating a simplified edge/face structure
    // In a full implementation, this would have 30 edges and 20 faces
    let edges = vec![]; // TODO: Implement complete icosahedron topology
    let edge_loops = vec![];
    let faces = vec![];
    
    PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    }
}

/// Generate a dodecahedron with given edge length (simplified version)  
///
/// Creates a regular dodecahedron centered at origin
/// Note: This is a complex geometry - simplified implementation
pub fn dodecahedron(edge_length: f64) -> PrimitiveResult {
    let phi = (1.0 + 5_f64.sqrt()) / 2.0; // Golden ratio
    let a = edge_length / (2.0 * (3.0 - phi).sqrt());
    
    // 20 vertices (simplified positioning)
    let vertices = vec![
        // 8 vertices of cube
        Vertex { id: 0, position: Vector3::new(a, a, a) },
        Vertex { id: 1, position: Vector3::new(a, a, -a) },
        Vertex { id: 2, position: Vector3::new(a, -a, a) },
        Vertex { id: 3, position: Vector3::new(a, -a, -a) },
        Vertex { id: 4, position: Vector3::new(-a, a, a) },
        Vertex { id: 5, position: Vector3::new(-a, a, -a) },
        Vertex { id: 6, position: Vector3::new(-a, -a, a) },
        Vertex { id: 7, position: Vector3::new(-a, -a, -a) },
        // 12 additional vertices on face centers (simplified)
        // TODO: Implement correct dodecahedron vertex positioning
    ];
    
    // For brevity, creating a simplified structure
    // In a full implementation, this would have 30 edges and 12 pentagonal faces
    let edges = vec![];
    let edge_loops = vec![];
    let faces = vec![];
    
    PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    }
}
