use crate::common;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn get_color_rgb(pixel_color: Color,samples_per_pixel: i32) -> (u8, u8, u8) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
 
    // Divide the color by the number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    // Write the translated [0, 255] value of each color component
    r = 256.0 * common::clamp(r, 0.0, 0.999);
    g = 256.0 * common::clamp(g, 0.0, 0.999);
    b = 256.0 * common::clamp(b, 0.0, 0.999);

    // Convert to u8
    (r as u8, g as u8, b as u8)
}