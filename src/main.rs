mod color;
mod ray;
mod common;
mod vec3;
mod hittable;
mod hittable_list;
mod sphere;
use hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, Vec3};
use crate::color::Color;
use crate::ray::Ray;
use std::io;

// Ray -> A+tB
// Sphere -> (P-C)*(P-C) = r^2
// (A+tB-C)(A+tB-C) = r^2
// t^2B*B + 2tB*(A-C) + (A-C)*(A-C) - r^2 = 0
fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().length_squared();
    let half_b = vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

// fn ray_color(r: Ray) -> Color {
//     let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);
//     if t > 0.0 {
//         let n = vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
//         let angle = vec3::dot(n, Vec3::new(0.0, 0.0, 1.0)).acos().to_degrees();
//         if angle > 50.0 {
//             return Color::new(0.0, 0.0, 0.0); // 边缘颜色为黑色
//         }
//         return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
//     }
//     let unit_direction = vec3::unit_vector(r.direction());
//     let t = 0.5 * (unit_direction.y() + 1.0);
//     (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.0, 0.8, 1.0)
// }

fn ray_color(r:Ray, world: &dyn Hittable)-> Color{
    let mut rec = HitRecord::new();
    
    return Vec3::new(0.0, 0.0, 0.0);
}

fn main() {
    // viewport
    let aspect_ratio = 16.0 / 9.0; // width / height
    let image_width = 800;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64; // % of width
            let v = j as f64 / (image_height - 1) as f64; // % of height

            let pixel_color = ray_color(Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            ));
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
}
