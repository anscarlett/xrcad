// SPDX-License-Identifier: MIT OR Apache-2.0

use nalgebra::Point3;
use nalgebra::Vector3;


#[derive(Debug, Clone, PartialEq)]
pub struct CoordinateSystem {
    pub origin: Point3<f64>,
    pub x_axis: Vector3<f64>,
    pub y_axis: Vector3<f64>,
    pub z_axis: Vector3<f64>,
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        CoordinateSystem {
            origin: Point3::new(0.0, 0.0, 0.0),
            x_axis: Vector3::new(1.0, 0.0, 0.0),
            y_axis: Vector3::new(0.0, 1.0, 0.0),
            z_axis: Vector3::new(0.0, 0.0, 1.0),
        }
    }
}
