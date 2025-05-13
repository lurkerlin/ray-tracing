mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::Point3;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use image::{ImageBuffer, Rgb};
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use std::rc::Rc;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    // Background color
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.0, 0.3, 0.8);
}

fn main() {
    // Viewport
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1280;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 20;

    // World
    let mut world = HittableList::new();

    // Materials
    let material_ground = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.5)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.4, 0.1), 0.0));

    // Objects
    let r = f64::cos(common::PI / 4.0);
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5*r, -1.0),100.0*r,material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0),0.5*r,material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-r, 0.0, -1.0),0.5*r,material_left)));
    world.add(Box::new(Sphere::new(Point3::new(r, 0.0, -1.0),0.5*r,material_right)));

    // Camera
    let cam = Camera::new(90.0, ASPECT_RATIO);

    // image buffer
    let mut img = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            let (r, g, b) = color::get_color_rgb(pixel_color, SAMPLES_PER_PIXEL);
            img.put_pixel(i as u32, (IMAGE_HEIGHT - 1 - j) as u32, Rgb([r, g, b]));
        }
    }
    // save image
    img.save("output.png").unwrap();
    println!("\nDone.");
}
