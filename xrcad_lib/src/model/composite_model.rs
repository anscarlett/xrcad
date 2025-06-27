// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: model::composite

/// Composite model struct.
pub struct CompositeModel;

impl CompositeModel {
    pub fn new() -> Self {
        CompositeModel
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_composite_new() {
        let c = CompositeModel::new();
        let _ = c;
    }
}
