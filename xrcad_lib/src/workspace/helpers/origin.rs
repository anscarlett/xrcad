// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: workspace::helpers::origin

#[derive(Debug, Default, Clone)]
pub struct Origin;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_origin_default() {
        let origin = Origin::default();
        let _ = origin;
    }
}
