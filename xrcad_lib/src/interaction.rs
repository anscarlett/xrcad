//! User interaction system for XRCAD
//!
//! This module handles user interactions and state management:
//! - Interaction events (selection, manipulation, etc.)
//! - Interaction state tracking
//! - Gesture recognition
//! - Command processing

pub mod event;
pub mod state;

// Re-export commonly used types
pub use event::*;
pub use state::*;