mod ray;
mod vec3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let nx = 200;
    let ny = 100;
    let max = 255;
    ppm_saver(nx, ny, max);
}

fn ppm_saver(nx: i32, ny: i32, max: i32) {
    let lower_left_corner = Vec3::vec(-2.0, -1.0, -1.0);
    let horizontal = Vec3::vec(4.0, 0.0, 0.0);
    let vertical = Vec3::vec(0.0, 2.0, 0.0);
    let origin = Vec3::vec_all(0.0);
    let mut file: File = File::create("foo.ppm").unwrap();
    let _line = format!("P3\n {}\n {}\n {}\n", nx, ny, max);
    file.write(_line.as_bytes());


    for i in 0..nx {
        for j in 0..ny {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;
            let ray = Ray::create_ray(origin, lower_left_corner + horizontal * r + vertical * g);
            let col = colour(ray);
            let ir: i32 = (255.99 * col.x) as i32;
            let ig: i32 = (255.99 * col.y) as i32;
            let ib: i32 = (255.99 * col.z) as i32;
            let line = format!("{} {} {}\n", ir, ig, ib);
            file.write(line.as_bytes());
        }
    }
}

fn colour(r: Ray) -> Vec3 {
    let vector: Vec3 = Vec3::vec(0.0, 0.0, -1.0);
    if hit_sphere(&vector, 0.5, &r) {
        return Vec3::vec(1.0, 0.0, 0.0)
    }
    let unit_direction = Vec3::normalised_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    Vec3::vec_all(1.0) * (1.0 - t) + Vec3::vec(0.5, 0.7, 1.0) * t
}

fn hit_sphere(centre: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - *centre;
    let a: f32 = Vec3::dot_product(r.direction(), r.direction());
    let b: f32 = 2.0 * Vec3::dot_product(oc, r.direction());
    let c: f32 = Vec3::dot_product(oc, oc) - radius * radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    discriminant > 0.0
}


