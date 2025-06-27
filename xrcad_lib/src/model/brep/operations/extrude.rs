// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: brep::opt::extrude

/// Extrude operation struct.
pub struct Extrude;

impl Extrude {
    pub fn new() -> Self {
        Extrude
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extrude_new() {
        let e = Extrude::new();
        let _ = e;
    }
}
