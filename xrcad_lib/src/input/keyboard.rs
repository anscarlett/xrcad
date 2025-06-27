// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::keyboard

/// Represents a keyboard input device.
pub struct Keyboard;

impl Keyboard {
    pub fn new() -> Self {
        Keyboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_keyboard_new() {
        let k = Keyboard::new();
        let _ = k;
    }
}
