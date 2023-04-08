use crate::{
    ray::Ray,
    vec3::{Point, Vec3}, Config,
};

#[derive(Debug)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Point, config: &Config) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * config.aspect_ratio();
        let focal_length = 1.0;

        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);

        // Camera faces -z direction
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + x * self.horizontal + y * self.vertical
                - self.origin,
        }
    }
}
