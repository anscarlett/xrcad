// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::mouse

/// Represents a mouse input device.
pub struct Mouse;

impl Mouse {
    pub fn new() -> Self {
        Mouse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mouse_new() {
        let m = Mouse::new();
        let _ = m;
    }
}
