use std::collections::HashMap;
use crate::model::brep::id::{ShellId, FaceId, EdgeId};

/// Reference to a face in the B-rep structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceRef(pub FaceId);

/// Reference to an edge in the B-rep structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeRef(pub EdgeId);

/// Orientation of a shell's normal vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellOrientation {
    /// Normal vectors point away from material (typical for outer shells)
    Outward,
    /// Normal vectors point toward material (typical for cavity shells)
    Inward,
}

/// A shell represents a connected collection of faces forming a boundary
#[derive(Debug, Clone)]
pub struct Shell {
    /// Unique identifier for this shell
    pub id: ShellId,
    
    /// Collection of faces that make up this shell
    pub faces: Vec<FaceRef>,
    
    /// Whether this shell forms a closed boundary (no free edges)
    pub is_closed: bool,
    
    /// Orientation of the shell's normal vectors
    pub orientation: ShellOrientation,
    
    /// Cache of boundary edges (only meaningful for open shells)
    boundary_edges: Option<Vec<EdgeRef>>,
}

impl Shell {
    /// Create a new shell
    pub fn new(id: ShellId, faces: Vec<FaceRef>, orientation: ShellOrientation) -> Self {
        Self {
            id,
            faces,
            is_closed: false, // Will be computed
            orientation,
            boundary_edges: None,
        }
    }
    
    /// Check if this shell is closed (no boundary edges)
    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
    
    /// Get boundary edges for open shells
    pub fn boundary_edges(&self) -> Option<&[EdgeRef]> {
        self.boundary_edges.as_deref()
    }
    
    /// Add a face to this shell
    pub fn add_face(&mut self, face: FaceRef) {
        self.faces.push(face);
        self.invalidate_cache();
    }
    
    /// Remove a face from this shell
    pub fn remove_face(&mut self, face: FaceRef) -> bool {
        if let Some(pos) = self.faces.iter().position(|&f| f == face) {
            self.faces.remove(pos);
            self.invalidate_cache();
            true
        } else {
            false
        }
    }
    
    /// Get all faces in this shell
    pub fn faces(&self) -> &[FaceRef] {
        &self.faces
    }
    
    /// Check if this shell contains a specific face
    pub fn contains_face(&self, face: FaceRef) -> bool {
        self.faces.contains(&face)
    }
    
    /// Flip the orientation of this shell
    pub fn flip_orientation(&mut self) {
        self.orientation = match self.orientation {
            ShellOrientation::Outward => ShellOrientation::Inward,
            ShellOrientation::Inward => ShellOrientation::Outward,
        };
    }
    
    /// Get the orientation of this shell
    pub fn orientation(&self) -> ShellOrientation {
        self.orientation
    }
    
    /// Compute topological properties (called after modifications)
    pub fn compute_properties(&mut self, edge_face_map: &HashMap<EdgeRef, Vec<FaceRef>>) {
        self.compute_closure(edge_face_map);
        self.compute_boundary_edges(edge_face_map);
    }
    
    /// Invalidate cached properties
    fn invalidate_cache(&mut self) {
        self.boundary_edges = None;
        // Mark as needing recomputation
    }
    
    /// Compute whether this shell is closed
    fn compute_closure(&mut self, edge_face_map: &HashMap<EdgeRef, Vec<FaceRef>>) {
        // A shell is closed if every edge is shared by exactly 2 faces within the shell
        for (_edge, face_refs) in edge_face_map {
            let faces_in_shell: Vec<_> = face_refs
                .iter()
                .filter(|&&face| self.faces.contains(&face))
                .collect();
            
            if faces_in_shell.len() == 1 {
                // Found a boundary edge - shell is open
                self.is_closed = false;
                return;
            }
        }
        self.is_closed = true;
    }
    
    /// Compute boundary edges for open shells
    fn compute_boundary_edges(&mut self, edge_face_map: &HashMap<EdgeRef, Vec<FaceRef>>) {
        if self.is_closed {
            self.boundary_edges = None;
            return;
        }
        
        let mut boundary = Vec::new();
        for (edge, face_refs) in edge_face_map {
            let faces_in_shell: Vec<_> = face_refs
                .iter()
                .filter(|&&face| self.faces.contains(&face))
                .collect();
            
            if faces_in_shell.len() == 1 {
                boundary.push(*edge);
            }
        }
        self.boundary_edges = Some(boundary);
    }
}

impl PartialEq for Shell {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Shell {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_creation() {
        let shell = Shell::new(
            ShellId::new(1),
            vec![FaceRef(FaceId::new(1)), FaceRef(FaceId::new(2))],
            ShellOrientation::Outward,
        );
        
        assert_eq!(shell.id, ShellId::new(1));
        assert_eq!(shell.faces.len(), 2);
        assert_eq!(shell.orientation, ShellOrientation::Outward);
    }
    
    #[test]
    fn test_orientation_flip() {
        let mut shell = Shell::new(
            ShellId::new(1),
            vec![],
            ShellOrientation::Outward,
        );
        
        shell.flip_orientation();
        assert_eq!(shell.orientation, ShellOrientation::Inward);
        
        shell.flip_orientation();
        assert_eq!(shell.orientation, ShellOrientation::Outward);
    }
}
