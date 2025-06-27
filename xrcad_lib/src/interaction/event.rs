// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: interaction::event

/// Represents a generic interaction event.
pub struct InteractionEvent;

impl InteractionEvent {
    pub fn new() -> Self {
        InteractionEvent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_event_new() {
        let e = InteractionEvent::new();
        let _ = e;
    }
}
