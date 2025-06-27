// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::sixdof_delta

/// Represents a delta (change) in 6DoF pose.
pub struct SixDofDelta {
    pub translation: [f32; 3],
    pub rotation: [f32; 4], // Quaternion (x, y, z, w)
}

impl SixDofDelta {
    pub fn new(translation: [f32; 3], rotation: [f32; 4]) -> Self {
        Self { translation, rotation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_delta() {
        let t = [0.1, 0.2, 0.3];
        let r = [0.0, 0.0, 0.0, 1.0];
        let delta = SixDofDelta::new(t, r);
        assert_eq!(delta.translation, t);
        assert_eq!(delta.rotation, r);
    }
}
