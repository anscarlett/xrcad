// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::opt::stitch

/// Stitch operation struct.
pub struct Stitch;

impl Stitch {
    pub fn new() -> Self {
        Stitch
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stitch_new() {
        let s = Stitch::new();
        let _ = s;
    }
}
