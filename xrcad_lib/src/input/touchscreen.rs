// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::touchscreen

/// Represents a touchscreen input device.
pub struct Touchscreen;

impl Touchscreen {
    pub fn new() -> Self {
        Touchscreen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_touchscreen_new() {
        let t = Touchscreen::new();
        let _ = t;
    }
}
