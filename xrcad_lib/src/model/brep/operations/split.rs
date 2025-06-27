// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::opt::split

/// Split operation struct.
pub struct Split;

impl Split {
    pub fn new() -> Self {
        Split
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_new() {
        let s = Split::new();
        let _ = s;
    }
}
