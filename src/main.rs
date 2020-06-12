use crate::color::Color;

mod vec3;
mod color;
mod point3;
mod ray;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Lines remained {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel = Color::new(
                i as f32 / IMAGE_WIDTH as f32,
                j as f32 / IMAGE_HEIGHT as f32,
                0.25,
            );

            print!("{}\n", pixel);
        }
    }
    eprintln!("Done!");
}
