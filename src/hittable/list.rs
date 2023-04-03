use std::ops::Range;

use crate::{ray::Ray, vec3::Real};

use super::{HitRecord, Hittable};

pub struct List<'a> {
    objects: Vec<Box<dyn 'a + Hittable>>,
}

impl Hittable for List<'_> {
    fn hit(&self, ray: &Ray, t_interval: &Range<Real>) -> Option<super::HitRecord> {
        let mut t_interval = t_interval.start .. t_interval.end;
        let mut hit_record: Option<super::HitRecord> = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, &t_interval) {
                t_interval = t_interval.start..rec.t;
                hit_record = Some(HitRecord::new(ray, rec.t, rec.normal));
            }
        }

        hit_record
    }
}

impl<'a> List<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl 'a + Hittable) {
        self.objects.push(Box::new(object))
    }
}
