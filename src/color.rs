use std::io::Write;
use crate::vec3::Vec3;
use crate::common;
// type alias for Vec3 to represent a color
pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color:Color, samples_per_pixel:i16) {

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0/samples_per_pixel as f32;
    r*=scale;
    g*=scale;
    b*=scale;

    writeln!(
        out, 
        "{} {} {}", 
        (256.0 * common::clamp(r, 0.000, 0.999)) as i16,
        (256.0 * common::clamp(g, 0.000, 0.999)) as i16,
        (256.0 * common::clamp(b, 0.000, 0.999)) as i16,
    ).expect("writing color failed");
}


