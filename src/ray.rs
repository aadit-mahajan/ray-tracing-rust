use crate::vec3::{Point3, Vec3};
 
#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}
 
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
 
    pub fn origin(&self) -> Point3 {
        self.origin
    }
 
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
 
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}