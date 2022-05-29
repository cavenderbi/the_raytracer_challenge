use crate::{tuple::Tuple, matrix::Matrix4x4};

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }
    pub fn position_at(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray { origin: m * self.origin, direction: m * self.direction }
    }
}
