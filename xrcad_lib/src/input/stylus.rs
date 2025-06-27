// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::stylus

/// Represents a stylus input device.
pub struct Stylus;

impl Stylus {
    pub fn new() -> Self {
        Stylus
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stylus_new() {
        let s = Stylus::new();
        let _ = s;
    }
}
