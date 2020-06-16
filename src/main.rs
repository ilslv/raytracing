mod vec3;
mod color;
mod point3;
mod ray;
mod hit;
mod sphere;
mod hittable_list;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::point3::Point3;
use crate::hit::{Hit, HitRecord};
use crate::hittable_list::HittableList;
use std::rc::Rc;
use crate::sphere::Sphere;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

fn ray_color(ray: &Ray, world: &impl Hit) -> Color {
    match world.hit(ray, 0.0, f32::INFINITY) {
        Some(hit_rec) => {
            (0.5 * (hit_rec.normal + Vec3::new(1.0, 1.0, 1.0))).into()
        }
        None => {
            let unit_direction = ray.direction.unit_vec();
            let t = 0.5 * (unit_direction.y() + 1.0);
            ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)).into()
        }
    }
}

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Lines remained {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray::new(
                origin,
                (lower_left_corner + u * horizontal + v * vertical - origin.into()).into(),
            );

            let pixel_color = ray_color(&r, &world);

            print!("{}\n", pixel_color);
        }
    }
    eprintln!("Done!");
}
