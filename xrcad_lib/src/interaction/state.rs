// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: interaction::state

/// Represents the state of an interaction.
pub struct InteractionState;

impl InteractionState {
    pub fn new() -> Self {
        InteractionState
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_state_new() {
        let s = InteractionState::new();
        let _ = s;
    }
}
