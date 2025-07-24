use std::collections::HashMap;
use super::shell::Shell;
use crate::model::brep::id::{BodyId, ShellId};

/// Reference to a shell in the B-rep structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShellRef(pub ShellId);

/// Type of body based on its shell configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    /// Solid body with a single closed shell
    Solid,
    /// Body with cavities (outer shell + inner shells)
    Hollow,
    /// Open body (contains open shells)
    Open,
    /// Complex body (multiple disconnected shells)
    Compound,
}

/// A body represents a complete 3D object composed of one or more shells
#[derive(Debug, Clone)]
pub struct Body {
    /// Unique identifier for this body
    pub id: BodyId,
    
    /// The outer boundary shell (required)
    pub outer_shell: ShellRef,
    
    /// Inner shells representing cavities/holes (optional)
    pub inner_shells: Vec<ShellRef>,
    
    /// Body type classification (computed when needed)
    body_type: Option<BodyType>,
}

impl Body {
    /// Create a new body with just an outer shell
    pub fn new(id: BodyId, outer_shell: ShellRef) -> Self {
        Self {
            id,
            outer_shell,
            inner_shells: Vec::new(),
            body_type: None,
        }
    }
    
    /// Create a new solid body (single closed shell)
    pub fn new_solid(id: BodyId, outer_shell: ShellRef) -> Self {
        Self {
            id,
            outer_shell,
            inner_shells: Vec::new(),
            body_type: Some(BodyType::Solid),
        }
    }
    
    /// Create a new hollow body (outer shell + cavities)
    pub fn new_hollow(id: BodyId, outer_shell: ShellRef, inner_shells: Vec<ShellRef>) -> Self {
        Self {
            id,
            outer_shell,
            inner_shells,
            body_type: Some(BodyType::Hollow),
        }
    }
    
    /// Get the outer shell reference
    pub fn outer_shell(&self) -> ShellRef {
        self.outer_shell
    }
    
    /// Get all inner shell references
    pub fn inner_shells(&self) -> &[ShellRef] {
        &self.inner_shells
    }
    
    /// Get all shell references (outer + inner)
    pub fn all_shells(&self) -> Vec<ShellRef> {
        let mut shells = vec![self.outer_shell];
        shells.extend(&self.inner_shells);
        shells
    }
    
    /// Add an inner shell (cavity)
    pub fn add_inner_shell(&mut self, shell: ShellRef) {
        self.inner_shells.push(shell);
        self.invalidate_cache();
    }
    
    /// Remove an inner shell
    pub fn remove_inner_shell(&mut self, shell: ShellRef) -> bool {
        if let Some(pos) = self.inner_shells.iter().position(|&s| s == shell) {
            self.inner_shells.remove(pos);
            self.invalidate_cache();
            true
        } else {
            false
        }
    }
    
    /// Check if this body has any inner shells (cavities)
    pub fn has_cavities(&self) -> bool {
        !self.inner_shells.is_empty()
    }
    
    /// Get the number of shells in this body
    pub fn shell_count(&self) -> usize {
        1 + self.inner_shells.len()
    }
    
    /// Check if this body contains a specific shell
    pub fn contains_shell(&self, shell: ShellRef) -> bool {
        self.outer_shell == shell || self.inner_shells.contains(&shell)
    }
    
    /// Get the body type (computed if not cached)
    pub fn body_type(&self, shells: &HashMap<ShellRef, Shell>) -> BodyType {
        if let Some(body_type) = self.body_type {
            return body_type;
        }
        
        // Compute body type based on shell properties
        let outer_shell = shells.get(&self.outer_shell);
        
        if self.inner_shells.is_empty() {
            // Single shell body
            if let Some(shell) = outer_shell {
                if shell.is_closed() {
                    BodyType::Solid
                } else {
                    BodyType::Open
                }
            } else {
                BodyType::Open // Default if shell not found
            }
        } else {
            // Multiple shells
            if let Some(shell) = outer_shell {
                if shell.is_closed() {
                    // Check if all inner shells are closed too
                    let all_inner_closed = self.inner_shells.iter()
                        .all(|&shell_ref| {
                            shells.get(&shell_ref)
                                .map(|s| s.is_closed())
                                .unwrap_or(false)
                        });
                    
                    if all_inner_closed {
                        BodyType::Hollow
                    } else {
                        BodyType::Compound
                    }
                } else {
                    BodyType::Compound
                }
            } else {
                BodyType::Compound
            }
        }
    }
    
    /// Check if this body is valid (has valid shells)
    pub fn is_valid(&self, shells: &HashMap<ShellRef, Shell>) -> bool {
        // Outer shell must exist
        if !shells.contains_key(&self.outer_shell) {
            return false;
        }
        
        // All inner shells must exist
        for &inner_shell in &self.inner_shells {
            if !shells.contains_key(&inner_shell) {
                return false;
            }
        }
        
        // Additional validation could go here:
        // - Check shell orientations
        // - Verify no intersecting shells
        // - Ensure inner shells are inside outer shell
        
        true
    }
    
    /// Compute and cache the body type
    pub fn compute_body_type(&mut self, shells: &HashMap<ShellRef, Shell>) {
        self.body_type = Some(self.body_type(shells));
    }
    
    /// Invalidate cached properties
    fn invalidate_cache(&mut self) {
        self.body_type = None;
    }
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Body {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::brep::topology::shell::{Shell, ShellOrientation};

    #[test]
    fn test_body_creation() {
        let body = Body::new(BodyId::new(1), ShellRef(ShellId::new(1)));
        
        assert_eq!(body.id, BodyId::new(1));
        assert_eq!(body.outer_shell, ShellRef(ShellId::new(1)));
        assert!(body.inner_shells.is_empty());
        assert!(!body.has_cavities());
    }
    
    #[test]
    fn test_hollow_body() {
        let body = Body::new_hollow(
            BodyId::new(1),
            ShellRef(ShellId::new(1)),
            vec![ShellRef(ShellId::new(2)), ShellRef(ShellId::new(3))]
        );
        
        assert_eq!(body.inner_shells.len(), 2);
        assert!(body.has_cavities());
        assert_eq!(body.shell_count(), 3);
    }
    
    #[test]
    fn test_body_type_computation() {
        let mut shells = HashMap::new();
        let closed_shell = Shell::new(ShellId::new(1), vec![], ShellOrientation::Outward);
        shells.insert(ShellRef(ShellId::new(1)), closed_shell);
        
        let body = Body::new_solid(BodyId::new(1), ShellRef(ShellId::new(1)));
        let body_type = body.body_type(&shells);
        assert_eq!(body_type, BodyType::Solid);
    }
}
