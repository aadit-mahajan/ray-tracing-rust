use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;

pub trait Material: {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian{albedo:a}
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray, 
        rec: &HitRecord, 
        attenuation: &mut Color, 
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            // If the scatter direction is near zero, use the normal as the scatter direction
            scatter_dir = rec.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_dir);
        true
    }

}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal{albedo:a}
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
 
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected);
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}