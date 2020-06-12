mod vec3;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Lines remained {}", j);
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / IMAGE_WIDTH as f32;
            let g = j as f32 / IMAGE_HEIGHT as f32;
            let b = 0.25;

            let ir = (255.0 * r) as i32;
            let ig = (255.0 * g) as i32;
            let ib = (255.0 * b) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprintln!("Done!");
}
