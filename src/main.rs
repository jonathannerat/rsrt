mod hittable;
mod ray;
mod vec3;

use std::{f32::INFINITY, ops::Range};

use hittable::list::List;
use hittable::sphere::Sphere;
use hittable::Hittable;
use ray::Ray;
use vec3::{Color, Real, Vec3};

struct Config {
    width: i32,
    height: i32,
    samples: i32,
    max_depth: i32,
}

impl Config {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            samples: 1,
            max_depth: 1,
        }
    }

    fn aspect_ratio(&self) -> Real {
        self.width as Real / self.height as Real
    }
}

fn main() {
    // Image
    let config = Config::new(640, 480);
    let aspect_ratio = config.aspect_ratio();

    // Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);

    // Camera faces -z direction
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    let width = config.width;
    let height = config.height;

    let mut world = List::new();

    world.add(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    });

    world.add(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    println!("P3\n{width} {height}\n255\n");

    for col in (0..height).rev() {
        for row in 0..width {
            let u = row as f32 / (width - 1) as f32;
            let v = col as f32 / (height - 1) as f32;
            let ray = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            let pixel = ray_color(&ray, &world);

            write_color(&pixel);
        }
    }
}

const VISIBLE_RANGE: Range<f32> = 0.0..INFINITY;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(rec) = world.hit(ray, &VISIBLE_RANGE) {
        return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction.normalized();
    let gradient = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - gradient) * Vec3(1.0, 1.0, 1.0) + gradient * Vec3(0.5, 0.7, 1.0)
}

fn write_color(color: &Color) {
    let red = (color.0 * 255.999) as i32;
    let green = (color.1 * 255.999) as i32;
    let blue = (color.2 * 255.999) as i32;

    println!("{red} {green} {blue}");
}
