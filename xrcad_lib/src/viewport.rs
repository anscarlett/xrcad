//! Viewport system for XRCAD
//!
//! This module handles viewport and camera functionality:
//! - Camera management and positioning
//! - Camera control systems (orbit, pan, zoom)
//! - Viewport configuration and projection
//! - View manipulation

pub mod camera;

// Re-export commonly used types
pub use camera::*;
