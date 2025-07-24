// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: workspace::helpers::axes

use bevy::prelude::*;
use crate::color::{RED, GREEN, BLUE};

#[derive(Debug, Default, Clone)]
pub struct Axes;

impl Axes {
    pub fn render(&self, gizmos: &mut Gizmos) {
        let origin = Vec3::ZERO;
        let length = 100.0;
        gizmos.line(origin, origin + Vec3::X * length, RED);
        gizmos.line(origin, origin + Vec3::Y * length, GREEN);
        gizmos.line(origin, origin + Vec3::Z * length, BLUE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_axes_default() {
        let axes = Axes::default();
        let _ = axes;
    }
}
