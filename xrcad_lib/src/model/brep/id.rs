//! ID types for B-rep entities
//!
//! This module provides a consistent ID system for all B-rep entities.
//! IDs are strongly typed to prevent mixing different entity types.

use std::fmt;

/// A trait for all B-rep entity IDs
pub trait BrepId: Copy + Clone + PartialEq + Eq + fmt::Debug + fmt::Display {
    /// The underlying ID value
    fn value(&self) -> u64;
    
    /// Create a new ID from a value
    fn from_value(value: u64) -> Self;
    
    /// Generate the next ID in sequence
    fn next(&self) -> Self {
        Self::from_value(self.value() + 1)
    }
}

/// Macro to generate ID types with consistent implementation
macro_rules! define_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(u64);
        
        impl $name {
            pub const fn new(value: u64) -> Self {
                Self(value)
            }
            
            pub const fn value(&self) -> u64 {
                self.0
            }
        }
        
        impl BrepId for $name {
            fn value(&self) -> u64 {
                self.0
            }
            
            fn from_value(value: u64) -> Self {
                Self(value)
            }
        }
        
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", $prefix, self.0)
            }
        }
        
        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }
    };
}

// Topology IDs
define_id!(BodyId, "B");
define_id!(ShellId, "S");
define_id!(FaceId, "F");
define_id!(EdgeId, "E");
define_id!(VertexId, "V");

// Geometry IDs
define_id!(PointId, "P");
define_id!(LineId, "L");
define_id!(CircleId, "C");
define_id!(SurfaceId, "SF");
define_id!(CurveId, "CR");

/// ID generator for creating unique IDs
#[derive(Debug, Clone)]
pub struct IdGenerator<T: BrepId> {
    next_id: u64,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: BrepId> IdGenerator<T> {
    pub fn new() -> Self {
        Self {
            next_id: 1, // Start from 1, reserve 0 for invalid/default
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn new_id(&mut self) -> T {
        let id = T::from_value(self.next_id);
        self.next_id += 1;
        id
    }
    
    pub fn peek_next(&self) -> T {
        T::from_value(self.next_id)
    }
}

impl<T: BrepId> Default for IdGenerator<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_id_creation() {
        let body_id = BodyId::new(42);
        assert_eq!(body_id.value(), 42);
        assert_eq!(body_id.to_string(), "B42");
    }
    
    #[test]
    fn test_id_generator() {
        let mut generator = IdGenerator::<BodyId>::new();
        let id1 = generator.new_id();
        let id2 = generator.new_id();
        
        assert_eq!(id1.value(), 1);
        assert_eq!(id2.value(), 2);
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_id_next() {
        let id = BodyId::new(5);
        let next = id.next();
        assert_eq!(next.value(), 6);
    }
}
