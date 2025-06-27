// Re-exports for ergonomic use in xrcad_app
pub use model::brep_model::{BrepModel, na_vec3_to_bevy};
pub use model::brep::topology::{vertex::Vertex, edge::Edge, face::Face, edge_loop::EdgeLoop};
pub use workspace::workspace::Workspace;
pub mod color;
pub use color::*;
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Adrian Scarlett

/// xrcad core library


pub mod input{
    pub mod mouse;
    pub mod keyboard;
    pub mod touchscreen;
    pub mod eyetrack;
    pub mod stylus;
    pub mod gamepad;
    pub mod sixdof_delta;
    pub mod sixdof_pose;
}

pub mod interaction{
    pub mod event;
    pub mod state;
    // pub mod gestures;
    // pub mod haptics;
    // pub mod voice;
}

pub mod model {
    pub mod brep {
        pub mod topology {
            pub mod vertex;
            pub mod edge;
            pub mod edge_loop;
            pub mod face;
        }
        pub mod geometry {
            pub mod circle;
            pub mod rectangle;
            pub mod polygon;
            pub mod line;
            pub mod point;
        }
        pub mod operations {
            pub mod extrude;
            pub mod split;
            pub mod stitch;
            // pub mod boolean;
            // pub mod revolve;
            // pub mod loft;
            // pub mod sweep;
            // pub mod fillet;
            // pub mod chamfer;
            // pub mod taper;
            // pub mod twist;
            // pub mod offset;
            // pub mod shell;
            // pub mod solid;
            // pub mod trim;
        }
        pub mod constraints {
            pub mod length;
            // pub mod angle;
            // pub mod tangent;
            // pub mod normal;
            // pub mod binormal;
            // pub mod perpendicular;
            // pub mod diameter;
            // pub mod radius;
            // pub mod parallel;
            // pub mod equal;
            // pub mod horizontal;
            // pub mod vertical;
            // pub mod coincident;
        }
    }
    pub mod brep_model;
    pub mod composite_model;
    pub mod form_model;
}

pub mod render{
    pub mod ghosting;
    pub mod hilighting;
    pub mod materials;
    // pub mod lighting;
    // pub mod shadows;
    // pub mod textures;
    // pub mod shaders;
}

pub mod viewport{
    pub mod camera;
    // pub mod frustum;
    // pub mod projection;
    // pub mod view;
}

pub mod workspace {
    pub mod helpers {
        pub mod axes;
        pub mod coordinate_system;
        pub mod grid;
        pub mod marker;
        pub mod origin;
        pub mod plane;
    }
    pub mod workspace;
}


