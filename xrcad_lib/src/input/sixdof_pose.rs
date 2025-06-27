// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::sixdof_pose

/// Represents the absolute pose of a 6DoF device (position + orientation).
#[derive(Debug, Clone, PartialEq)]
pub struct SixDofPose {
    pub position: [f32; 3],
    pub orientation: [f32; 4], // Quaternion (x, y, z, w)
}

impl SixDofPose {
    /// Creates a new SixDofPose with the given position and orientation.
    pub fn new(position: [f32; 3], orientation: [f32; 4]) -> Self {
        Self { position, orientation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pose() {
        let pos = [1.0, 2.0, 3.0];
        let orient = [0.0, 0.0, 0.0, 1.0];
        let pose = SixDofPose::new(pos, orient);
        assert_eq!(pose.position, pos);
        assert_eq!(pose.orientation, orient);
    }
}
