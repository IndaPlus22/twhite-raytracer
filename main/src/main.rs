mod ppm_viewer;
mod ray;
mod vec3;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let max = 255;
    print_ppm(nx, ny, max);
}

fn print_ppm(nx: i32, ny: i32, max: i32) {
    println!("P3\n{} {}\n{}", nx, ny, max);

    for i in 0..nx {
        for j in 0..ny {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;
            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn color(r: Ray) -> Vec3 {
    let unit_direction = Vec3::normalised_vector(&r.direction());
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    Vec3::vec_all_coordinates_same(1.0) * (1.0 - t) + Vec3::vec(0.5, 0.7, 1.0) * t
}
