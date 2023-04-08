mod camera;
mod config;
mod constants;
mod hittable;
mod material;
mod ray;
mod vec3;

use std::{
    f32::INFINITY,
    io::{self, Write},
    ops::Range, rc::Rc,
};

use camera::Camera;
use config::Config;
use hittable::list::List;
use hittable::sphere::Sphere;
use hittable::Hittable;
use ray::Ray;
use vec3::{Color, Real, Vec3};

use crate::material::lambertian::Lambertian;

fn main() {
    // Image
    let config = Config::new(360, 240, 100, 50);

    let origin = Vec3::zero();
    let camera = Camera::new(origin, &config);

    let width = config.width;
    let height = config.height;

    let mut world = List::new();

    world.add(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian(Vec3(0.2, 0.1, 0.5)))
    });

    world.add(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian(Vec3(0.8, 0.3, 0.1)))
    });

    println!("P3\n{width} {height}\n255\n");

    for col in (0..height).rev() {
        eprint!("\r Scanlines remaining: {col}");
        io::stderr().flush().unwrap();

        for row in 0..width {
            let mut pixel = Vec3::zero();

            for _ in 0..config.samples {
                let u = random_in_step(row, width);
                let v = random_in_step(col, height);
                let ray = camera.get_ray(u, v);

                pixel += ray_color(&ray, &world, config.max_depth);
            }

            write_color(&pixel, config.samples);
        }
    }
}

fn random_in_step(step: i32, total: i32) -> Real {
    (step as Real + rand::random::<Real>()) / (total - 1) as Real
}

const VISIBLE_RANGE: Range<f32> = 0.001..INFINITY;

fn ray_color(ray: &Ray, world: &List, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::zero()
    } else {
        if let Some(rec) = world.hit(ray, &VISIBLE_RANGE) {
            let material = rec.material.as_ref().unwrap();

            if let Some(scatter) = material.scatter(&ray, &rec) {
                let attenuation = scatter.attenuation;
                let scattered = scatter.scattered;

                attenuation * ray_color(&scattered, world, depth - 1)
            } else {
                unreachable!("Objects should have material")
            }
        } else {
            let unit_direction = ray.direction.normalized();
            let gradient = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - gradient) * Vec3(1.0, 1.0, 1.0) + gradient * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn write_color(color: &Color, samples: i32) {
    let red = scale_color_by_samples(color.0, samples);
    let green = scale_color_by_samples(color.1, samples);
    let blue = scale_color_by_samples(color.2, samples);

    println!("{red} {green} {blue}");
}

fn scale_color_by_samples(color: f32, samples: i32) -> i32 {
    let color = (color / samples as f32).sqrt();
    (color.clamp(0.0, 1.0) * 256.0) as i32
}
