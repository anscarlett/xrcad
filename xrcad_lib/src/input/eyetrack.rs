// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: input::eyetrack

/// Represents an eye tracking device.
pub struct EyeTracker;

impl EyeTracker {
    pub fn new() -> Self {
        EyeTracker
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eyetracker_new() {
        let tracker = EyeTracker::new();
        let _ = tracker;
    }
}
