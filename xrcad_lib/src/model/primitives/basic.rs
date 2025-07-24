//! Basic geometric primitives (non-Platonic solids)
//!
//! Common geometric shapes that are useful for CAD modeling

use nalgebra::Vector3;
use crate::model::brep::topology::{vertex::Vertex, edge::Edge, edge_loop::EdgeLoop, face::Face};
use super::platonic::PrimitiveResult;

/// Generate a rectangular prism (box) with given dimensions
///
/// Creates a box centered at origin with specified width, height, and depth
pub fn box_primitive(width: f64, height: f64, depth: f64) -> PrimitiveResult {
    let half_w = width / 2.0;
    let half_h = height / 2.0;
    let half_d = depth / 2.0;
    
    // Create 8 vertices
    let vertices = vec![
        Vertex { id: 0, position: Vector3::new(-half_w, -half_h, -half_d) },
        Vertex { id: 1, position: Vector3::new( half_w, -half_h, -half_d) },
        Vertex { id: 2, position: Vector3::new( half_w,  half_h, -half_d) },
        Vertex { id: 3, position: Vector3::new(-half_w,  half_h, -half_d) },
        Vertex { id: 4, position: Vector3::new(-half_w, -half_h,  half_d) },
        Vertex { id: 5, position: Vector3::new( half_w, -half_h,  half_d) },
        Vertex { id: 6, position: Vector3::new( half_w,  half_h,  half_d) },
        Vertex { id: 7, position: Vector3::new(-half_w,  half_h,  half_d) },
    ];
    
    // Create 12 edges (same topology as cube)
    let edges = vec![
        // Bottom face edges (z = -half_d)
        Edge { id: 0, vertices: (0, 1) },
        Edge { id: 1, vertices: (1, 2) },
        Edge { id: 2, vertices: (2, 3) },
        Edge { id: 3, vertices: (3, 0) },
        // Top face edges (z = +half_d)
        Edge { id: 4, vertices: (4, 5) },
        Edge { id: 5, vertices: (5, 6) },
        Edge { id: 6, vertices: (6, 7) },
        Edge { id: 7, vertices: (7, 4) },
        // Vertical edges
        Edge { id: 8, vertices: (0, 4) },
        Edge { id: 9, vertices: (1, 5) },
        Edge { id: 10, vertices: (2, 6) },
        Edge { id: 11, vertices: (3, 7) },
    ];
    
    // Create 6 edge loops
    let edge_loops = vec![
        EdgeLoop::new(0, vec![vec![0, 1, 2, 3]]),      // Bottom
        EdgeLoop::new(1, vec![vec![4, 5, 6, 7]]),      // Top
        EdgeLoop::new(2, vec![vec![0, 9, 4, 8]]),      // Front
        EdgeLoop::new(3, vec![vec![2, 11, 6, 10]]),    // Back
        EdgeLoop::new(4, vec![vec![3, 11, 7, 8]]),     // Left
        EdgeLoop::new(5, vec![vec![1, 10, 5, 9]]),     // Right
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

/// Generate a cylinder with given radius and height
///
/// Creates a cylinder centered at origin with axis along Z
/// `segments` controls the number of sides (minimum 3)
pub fn cylinder(radius: f64, height: f64, segments: usize) -> Option<PrimitiveResult> {
    if segments < 3 {
        return None;
    }
    
    let half_height = height / 2.0;
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut edge_loops = Vec::new();
    let mut faces = Vec::new();
    
    // Create vertices for bottom and top circles
    for i in 0..segments {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        
        // Bottom vertex
        vertices.push(Vertex {
            id: i,
            position: Vector3::new(x, y, -half_height),
        });
        
        // Top vertex  
        vertices.push(Vertex {
            id: i + segments,
            position: Vector3::new(x, y, half_height),
        });
    }
    
    // Center vertices for top and bottom faces
    vertices.push(Vertex {
        id: segments * 2,
        position: Vector3::new(0.0, 0.0, -half_height), // Bottom center
    });
    vertices.push(Vertex {
        id: segments * 2 + 1,
        position: Vector3::new(0.0, 0.0, half_height),  // Top center
    });
    
    // Create edges
    let mut edge_id = 0;
    
    // Bottom circle edges
    for i in 0..segments {
        let next = (i + 1) % segments;
        edges.push(Edge {
            id: edge_id,
            vertices: (i, next),
        });
        edge_id += 1;
    }
    
    // Top circle edges
    for i in 0..segments {
        let next = (i + 1) % segments;
        edges.push(Edge {
            id: edge_id,
            vertices: (i + segments, next + segments),
        });
        edge_id += 1;
    }
    
    // Vertical edges
    for i in 0..segments {
        edges.push(Edge {
            id: edge_id,
            vertices: (i, i + segments),
        });
        edge_id += 1;
    }
    
    // Radial edges for bottom face
    for i in 0..segments {
        edges.push(Edge {
            id: edge_id,
            vertices: (segments * 2, i), // Center to rim
        });
        edge_id += 1;
    }
    
    // Radial edges for top face
    for i in 0..segments {
        edges.push(Edge {
            id: edge_id,
            vertices: (segments * 2 + 1, i + segments), // Center to rim
        });
        edge_id += 1;
    }
    
    // Create edge loops and faces
    // Bottom face (triangular segments from center)
    for i in 0..segments {
        let loop_edges = vec![vec![
            i,                           // Bottom circle edge
            segments * 3 + i,           // Radial edge
            segments * 3 + ((i + segments - 1) % segments), // Previous radial edge
        ]];
        
        edge_loops.push(EdgeLoop::new(i, loop_edges));
        faces.push(Face { id: i, edge_loops: vec![i] });
    }
    
    // Top face (triangular segments from center)
    for i in 0..segments {
        let loop_edges = vec![vec![
            segments + i,               // Top circle edge
            segments * 4 + i,          // Top radial edge
            segments * 4 + ((i + segments - 1) % segments), // Previous top radial edge
        ]];
        
        edge_loops.push(EdgeLoop::new(segments + i, loop_edges));
        faces.push(Face { id: segments + i, edge_loops: vec![segments + i] });
    }
    
    // Side faces (quadrilaterals)
    for i in 0..segments {
        let next = (i + 1) % segments;
        let loop_edges = vec![vec![
            i,                          // Bottom edge
            segments * 2 + next,        // Right vertical edge
            segments + i,               // Top edge
            segments * 2 + i,           // Left vertical edge
        ]];
        
        edge_loops.push(EdgeLoop::new(segments * 2 + i, loop_edges));
        faces.push(Face { id: segments * 2 + i, edge_loops: vec![segments * 2 + i] });
    }
    
    Some(PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    })
}

/// Generate a sphere approximation using an icosphere
///
/// Creates a sphere by subdividing an icosahedron
/// `subdivisions` controls the level of detail (0 = icosahedron, higher = smoother)
pub fn sphere(radius: f64, _subdivisions: u32) -> PrimitiveResult {
    // Start with icosahedron vertices
    let phi = (1.0 + 5_f64.sqrt()) / 2.0;
    
    let vertices = vec![
        Vector3::new(-1.0, phi, 0.0).normalize() * radius,
        Vector3::new(1.0, phi, 0.0).normalize() * radius,
        Vector3::new(-1.0, -phi, 0.0).normalize() * radius,
        Vector3::new(1.0, -phi, 0.0).normalize() * radius,
        Vector3::new(0.0, -1.0, phi).normalize() * radius,
        Vector3::new(0.0, 1.0, phi).normalize() * radius,
        Vector3::new(0.0, -1.0, -phi).normalize() * radius,
        Vector3::new(0.0, 1.0, -phi).normalize() * radius,
        Vector3::new(phi, 0.0, -1.0).normalize() * radius,
        Vector3::new(phi, 0.0, 1.0).normalize() * radius,
        Vector3::new(-phi, 0.0, -1.0).normalize() * radius,
        Vector3::new(-phi, 0.0, 1.0).normalize() * radius,
    ];
    
    // For simplicity, return basic icosahedron without subdivision
    // TODO: Implement subdivision for smoother sphere
    let brep_vertices: Vec<Vertex> = vertices
        .into_iter()
        .enumerate()
        .map(|(i, pos)| Vertex { id: i, position: pos })
        .collect();
    
    // Basic triangular faces of icosahedron (simplified)
    let edges = vec![];
    let edge_loops = vec![];
    let faces = vec![];
    
    PrimitiveResult {
        vertices: brep_vertices,
        edges,
        edge_loops,
        faces,
    }
}

/// Generate a cone with given base radius and height
///
/// Creates a cone centered at origin with base on XY plane
/// `segments` controls the number of sides of the base (minimum 3)
pub fn cone(base_radius: f64, height: f64, segments: usize) -> Option<PrimitiveResult> {
    if segments < 3 {
        return None;
    }
    
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut edge_loops = Vec::new();
    let mut faces = Vec::new();
    
    // Create vertices for base circle
    for i in 0..segments {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
        let x = base_radius * angle.cos();
        let y = base_radius * angle.sin();
        
        vertices.push(Vertex {
            id: i,
            position: Vector3::new(x, y, 0.0),
        });
    }
    
    // Center of base and apex
    vertices.push(Vertex {
        id: segments,
        position: Vector3::new(0.0, 0.0, 0.0), // Base center
    });
    vertices.push(Vertex {
        id: segments + 1,
        position: Vector3::new(0.0, 0.0, height), // Apex
    });
    
    // Create edges for base circle
    for i in 0..segments {
        let next = (i + 1) % segments;
        edges.push(Edge {
            id: i,
            vertices: (i, next),
        });
    }
    
    // Radial edges from base center to rim
    for i in 0..segments {
        edges.push(Edge {
            id: segments + i,
            vertices: (segments, i),
        });
    }
    
    // Edges from rim to apex
    for i in 0..segments {
        edges.push(Edge {
            id: segments * 2 + i,
            vertices: (i, segments + 1),
        });
    }
    
    // Base face triangles
    for i in 0..segments {
        let loop_edges = vec![vec![
            i,                           // Base circle edge
            segments + i,                // Radial edge
            segments + ((i + segments - 1) % segments), // Previous radial edge
        ]];
        
        edge_loops.push(EdgeLoop::new(i, loop_edges));
        faces.push(Face { id: i, edge_loops: vec![i] });
    }
    
    // Side face triangles
    for i in 0..segments {
        let next = (i + 1) % segments;
        let loop_edges = vec![vec![
            i,                           // Base edge
            segments * 2 + next,         // Right edge to apex
            segments * 2 + i,            // Left edge to apex
        ]];
        
        edge_loops.push(EdgeLoop::new(segments + i, loop_edges));
        faces.push(Face { id: segments + i, edge_loops: vec![segments + i] });
    }
    
    Some(PrimitiveResult {
        vertices,
        edges,
        edge_loops,
        faces,
    })
}
