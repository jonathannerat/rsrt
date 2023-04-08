use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{Material, Scatter};

#[derive(Debug)]
pub struct Lambertian(pub Color);

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut direction = rec.normal + Vec3::unit_sphere().normalized();

        if direction.is_near_zero() {
            direction = rec.normal;
        }

        Some(Scatter {
            attenuation: self.0,
            scattered: Ray {
                origin: rec.point,
                direction,
            },
        })
    }
}
