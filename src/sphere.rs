use crate::ray::Ray;
use crate::vec3::{self, Point3};
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center:center, radius:radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec3::dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrt_d = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || root >= t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        true
    }
}