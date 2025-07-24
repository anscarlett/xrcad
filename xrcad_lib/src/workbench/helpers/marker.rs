// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: workspace::helpers::marker

#[derive(Debug, Default, Clone)]
pub struct Marker;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_marker_default() {
        let marker = Marker::default();
        let _ = marker;
    }    
}    
