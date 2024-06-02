mod color;
mod common;
mod hittable;
mod camera;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Point3;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use std::io;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let direction = rec.normal + vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);
        // return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0)); // Normal to RGB
    }
    // if not hit, render background
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0); // -1.0 < y < 1.0 -> 0.0 < t < 1.0
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.0, 0.5, 0.8); // lerp
    // based on height of y
}

fn main() {
    // viewport
    const ASPECT_RATIO: f64 = 16.0 / 9.0; // width / height
    const IMAGE_WIDTH: i32 = 640;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::new();
    // Render
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev(){
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone.\n");
}
