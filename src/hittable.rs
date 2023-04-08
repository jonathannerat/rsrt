pub mod list;
pub mod plane;
pub mod sphere;

use std::ops::Range;
use std::rc::Rc;

use crate::material::Material;
use crate::vec3::{Point, Real, Vec3};
use crate::Ray;

pub struct HitRecord {
    pub t: Real,
    pub point: Point,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>
}

impl HitRecord {
    fn new(ray: &Ray, t: Real, normal: Vec3, material: Option<&Rc<dyn Material>>) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;

        Self {
            t,
            point: ray.at(t),
            front_face,
            normal: if front_face { normal } else { -normal },
            material: material.map(| material_ref | { Rc::clone(material_ref) })
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_interval: &Range<Real>) -> Option<HitRecord>;
}
