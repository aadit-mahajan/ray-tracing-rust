mod color;
mod vec3;
mod ray;
mod hittable;
mod sphere;
mod common;
mod hittable_list;
mod camera;

use std::io;
use camera::Camera;
use hittable::{HitRecord,Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use ray::Ray;
use color::Color;
use vec3::{Point3, Vec3};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i16) -> Color {
    let mut rec = HitRecord::new();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0); // Return black if depth is zero
    }

    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        let direction = rec.normal + vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray::new(rec.p, direction), world, depth - 1);
    }


    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    
    //image
    const ASPECT_RATIO : f32 = 16.0/9.0;
    const IMAGE_WIDTH : i16 = 400;
    const IMAGE_HEIGHT : i16 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i16;
    const SAMPLES_PER_PIXEL: i16 = 100;
    const MAX_DEPTH: i16 = 50;

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //camera
    let camera = Camera::new();

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("lines remaining: {}\n", j);
           for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + common::random_double()) / (IMAGE_WIDTH-1) as f32;
                let v = (j as f32 + common::random_double()) / (IMAGE_HEIGHT-1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nDone\n");
}
