//! Input handling for XRCAD
//!
//! This module provides input handling for various input devices:
//! - Eye tracking
//! - Gamepad/controller input
//! - Keyboard input
//! - Mouse and touchpad input
//! - SixDOF spatial input devices
//! - Stylus/pen input
//! - Touchscreen gestures

pub mod eyetrack;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;
pub mod sixdof_delta;
pub mod sixdof_pose;
pub mod stylus;
pub mod touchscreen;

// Re-export commonly used types
pub use mouse::*;
pub use keyboard::*;
pub use touchscreen::*;