// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: render::hilighting

/// Hilighting render struct.
pub struct Hilighting;

impl Hilighting {
    pub fn new() -> Self {
        Hilighting
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hilighting_new() {
        let h = Hilighting::new();
        let _ = h;
    }
}
