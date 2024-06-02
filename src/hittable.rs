use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};
 
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3, // Point of intersection
    pub normal: Vec3, // Normal at the point of intersection
    pub t: f64,// Distance from the ray origin to the intersection point
    pub front_face: bool, // True if the normal is facing the ray
}
 
impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
 
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool; // Return true if the ray hits the object
}