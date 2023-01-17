mod ppm_viewer;
mod vec3; 
use crate::vec3::Vec3;


fn main() {

    let nx = 200;
    let ny = 100;
    let max = 255;
    print_ppm(nx, ny, max);

    let v = Vec3::vec(2.0, 10.0, 18.0);
    let u = Vec3::vec(2.0, 5.0, 6.0);
    let w = v / u;
    print!("{:?}\n", w);
    let q = w.normalise();
    let len = q.length();
    print!("{}\n", len);


    println!("{} {} {}", w.x, w.y, w.z);
    println!("{} {} {}", q.x, q.y, q.z);
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