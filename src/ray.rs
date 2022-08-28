use crate::primitives::Vec3;

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Coordinates of origin + t*direction
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
