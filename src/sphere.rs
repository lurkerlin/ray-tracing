use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};
 
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}
 
impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Ray -> A+tB
        // Sphere -> (P-C)*(P-C) = r^2
        // (A+tB-C)(A+tB-C) = r^2
        // B*B * t^2 + 2*B*(A-C) * t + ((A-C)*(A-C) - r^2) = 0
        // a= B*B, b= 2*B*(A-C), c = (A-C)*(A-C) - r^2
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction()); // half of b = B*(A-C)
        let c = oc.length_squared() - self.radius * self.radius;
        // we just need to check if it's positive or not 
        // so we use discriminant = (b/2)^2 - ac
        let discriminant = half_b * half_b - a * c; 
        if discriminant < 0.0 {
            return false;
        }
 
        let sqrt_d = f64::sqrt(discriminant);
 
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a; // t = (-half_b - sqrt(discriminant)) / a
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
        
        // constrecuting the hit record
        rec.t = root;
        rec.p = r.at(rec.t);
        
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.material.clone());
        true
    }
}