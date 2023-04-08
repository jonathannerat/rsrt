use std::ops::Range;
use std::rc::Rc;

use super::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Real};

pub struct Sphere {
    pub center: Point,
    pub radius: Real,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_interval: &Range<Real>) -> Option<HitRecord> {
        let eq = self.calculate_quadratic_coefficients(ray);

        if let Some((root_min, root_max)) = eq.solutions() {
            let min_root_in_interval = if t_interval.contains(&root_min) {
                Some(root_min)
            } else {
                if t_interval.contains(&root_max) {
                    Some(root_max)
                } else {
                    None
                }
            };

            if let Some(root) = min_root_in_interval {
                let point = ray.at(root);

                Some(HitRecord::new(
                    &ray,
                    root,
                    (point - self.center) / self.radius,
                    Some(&self.material)
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

// Sphere equation: (R(t) - C) . (R(t) - C) = r^2
//                  (O + tD - C) . (O + tD - C) = r^2
//                  (D . D)t^2 + 2 t D . (O - C) + (O - C) . (O - C) - r^2 = 0
//                  a.t^2 + b.t + c = 0
struct FasterQuadEq {
    a: Real,
    half_b: Real,
    discriminant: Real,
}

impl FasterQuadEq {
    fn new(a: Real, half_b: Real, c: Real) -> Self {
        Self {
            a,
            half_b,
            discriminant: half_b * half_b - a * c,
        }
    }

    fn solutions(&self) -> Option<(Real, Real)> {
        if self.discriminant >= 0.0 {
            let sqrtd = self.discriminant.sqrt();

            Some((
                (-self.half_b - sqrtd) / self.a,
                (-self.half_b + sqrtd) / self.a,
            ))
        } else {
            None
        }
    }
}

impl Sphere {
    fn calculate_quadratic_coefficients(&self, ray: &Ray) -> FasterQuadEq {
        let o_minus_c = ray.origin - self.center;
        let a = ray.direction.length2();
        let half_b = ray.direction.dot(&o_minus_c);
        let c = o_minus_c.length2() - self.radius * self.radius;

        FasterQuadEq::new(a, half_b, c)
    }
}
