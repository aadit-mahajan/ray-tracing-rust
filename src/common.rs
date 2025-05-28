use rand::Rng;

pub use std::f32::consts::PI;
pub use std::f32::INFINITY;
 
// Utility functions
 
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    rand::rng().random()
}

pub fn random_double_range(min: f32, max:f32) -> f32 {
    rand::rng().random_range(min..max)
}

pub fn clamp(x:f32, min:f32, max:f32) -> f32{
    if x < min{
        return min;
    }
    if x > max{
        return max;
    }
    x
}
