mod color;
mod vec3;

use std::io;
use color::Color;
use vec3::Vec3;



fn main() {
    
    //image
    const IMAGE_WIDTH:u16=256;
    const IMAGE_HEIGHT:u16=256;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("lines remaining: {}\n", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let pixel_color = Color::new(r, g, b);
            color::write_color(&mut io::stdout(), pixel_color);

            // let ir = (255.999 * r) as i32;
            // let ig = (255.999 * g) as i32;
            // let ib = (255.999 * b) as i32;
            // print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprint!("\nDone\n");
}
