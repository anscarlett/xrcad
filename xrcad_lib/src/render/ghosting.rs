// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: render::ghosting

/// Ghosting render struct.
pub struct Ghosting;

impl Ghosting {
    pub fn new() -> Self {
        Ghosting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ghosting_new() {
        let g = Ghosting::new();
        let _ = g;
    }
}
