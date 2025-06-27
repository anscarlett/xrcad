// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: render::materials

/// Materials render struct.
pub struct Materials;

impl Materials {
    pub fn new() -> Self {
        Materials
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_materials_new() {
        let m = Materials::new();
        let _ = m;
    }
}
