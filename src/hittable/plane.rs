use std::{ops, rc::Rc};

use crate::{
    constants::EPSILON,
    ray::Ray,
    vec3::{Point, Real, Vec3}, material::Material,
};

use super::{HitRecord, Hittable};

pub struct Plane {
    pub origin: Point,
    pub normal: Vec3,
    pub material: Rc<dyn Material>
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_interval: &ops::Range<Real>) -> Option<super::HitRecord> {
        if ray.direction.dot(&self.normal) < EPSILON {
            None
        } else {
            let t = (self.origin - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);

            if !t_interval.contains(&t) {
                None
            } else {
                Some(HitRecord::new(&ray, t, self.normal, Some(&self.material)))
            }
        }
    }
}
