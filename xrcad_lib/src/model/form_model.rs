// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

//! Module: model::form

/// Form model struct.
pub struct FormModel;

impl FormModel {
    pub fn new() -> Self {
        FormModel
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_form_new() {
        let f = FormModel::new();
        let _ = f;
    }
}
