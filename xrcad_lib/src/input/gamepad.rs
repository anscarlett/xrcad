// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::keyboard

/// Represents a gamepad input device.
pub struct Gamepad;

impl Gamepad {
    pub fn new() -> Self {
        Gamepad
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gamepad_new() {
        let g = Gamepad::new();
        let _ = g;
    }
}
