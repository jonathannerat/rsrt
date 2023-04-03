use crate::vec3::{Real, Vec3, Point};

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: Real) -> Point {
        self.origin + t * self.direction
    }
}
