use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let ASPECT_RATIO = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO*viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
            

        Camera {
            origin, 
            lower_left_corner,
            horizontal, 
            vertical
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin, 
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        )
    }
}