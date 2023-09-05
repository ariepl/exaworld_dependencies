use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rotation {
    deg: f32,
    rad: f32,
}

impl Rotation {
    pub fn new(rad: f32) -> Self {
        Self {
            deg: rad * 180. / PI,
            rad,
        }
    }

    #[allow(dead_code)]
    pub fn set_deg(&mut self, mut deg: f32) {
        let full_circle = 360.;

        while deg >= full_circle {
            deg -= full_circle;
        }
        while deg < 0.0 {
            deg += full_circle;
        }

        self.deg = deg;
        self.rad = deg * PI / 180.;
    }

    #[allow(dead_code)]
    pub fn set_rad(&mut self, mut rad: f32) {
        let full_circle = 2. * PI;

        while rad >= full_circle {
            rad -= full_circle;
        }
        while rad < 0.0 {
            rad += full_circle;
        }

        self.rad = rad;
        self.deg = rad * 180. / PI;
    }

    #[allow(dead_code)]
    pub fn get_deg(&self) -> f32 {
        self.deg
    }

    #[allow(dead_code)]
    pub fn get_rad(&self) -> f32 {
        self.rad
    }
}
