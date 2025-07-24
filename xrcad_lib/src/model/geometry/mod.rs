//! Geometry module for XRCAD
//!
//! This module contains fundamental geometric primitives and operations.
//! It provides a unified foundation for both BREP geometric operations
//! and visual construction helpers.

pub mod plane;

// Re-export commonly used types
pub use plane::*;
