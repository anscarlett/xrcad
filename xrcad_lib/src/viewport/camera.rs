// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: viewport::camera

/// Camera viewport struct.
pub struct Camera;

impl Camera {
    pub fn new() -> Self {
        Camera
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_camera_new() {
        let c = Camera::new();
        let _ = c;
    }
}
