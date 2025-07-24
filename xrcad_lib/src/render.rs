//! Rendering system for XRCAD
//!
//! This module handles visual rendering functionality:
//! - Ghosting effects for transparent display
//! - Highlighting for selection and emphasis
//! - Material rendering and shading
//! - Visual feedback systems

pub mod ghosting;
pub mod hilighting;
pub mod materials;

// Re-export commonly used types
pub use ghosting::*;
pub use hilighting::*;
pub use materials::*;