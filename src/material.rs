pub mod lambertian;

use crate::{ray::Ray, hittable::HitRecord, vec3::Color};

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<Scatter>;
}
