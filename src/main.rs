mod progress_bar;
mod angle;
mod vec3;
mod color;
mod point3;
mod ray;
mod hit;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::point3::Point3;
use crate::hit::Hit;
use crate::hittable_list::HittableList;
use std::rc::Rc;
use crate::sphere::Sphere;
use crate::camera::Camera;
use rand::Rng;
use crate::material::{Lambertian, Metal, Dielectric, Material};
use crate::angle::Degrees;
use crate::progress_bar::ProgressBar;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 1600;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: u32 = 50;

fn ray_color(ray: &Ray, world: &impl Hit, depth: u32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
        Some(hit_rec) => {
            hit_rec.material
                .scatter(ray, &hit_rec)
                .map_or_else(
                    || Color::new(0.0, 0.0, 0.0),
                    |r| (Vec3::from(r.attenuation) * Vec3::from(ray_color(&r.scattered, world, depth - 1))).into(),
                )
        }
        None => {
            let unit_direction = ray.direction.unit_vec();
            let t = 0.5 * (unit_direction.y() + 1.0);
            ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)).into()
        }
    }
}

fn scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::default();

    let ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (Vec3::from(center) - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = match rng.gen::<f32>() {
                    x if (0.0..0.8).contains(&x) => {
                        let albedo = Color::random();
                        Rc::new(Lambertian::new(albedo))
                    },
                    x if (0.8..0.9).contains(&x) => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let roughness = rng.gen_range(0.0, 0.5);
                        Rc::new(Metal::new(albedo, roughness))
                    },
                    _ => {
                        Rc::new(Dielectric::new(1.5))
                    },
                };

                world.add(Rc::new(Sphere::new(
                    center,
                    0.2,
                    sphere_material,
                )));
            }
        }
    }

    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    world
}

fn main() {
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();

    let origin = Point3::new(13.0, 2.0, 3.0);
    let destination = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        origin,
        destination,
        view_up,
        Degrees(20.0),
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let world = scene();
    let progress_bar = ProgressBar::new(50);

    for j in (0..IMAGE_HEIGHT).rev() {
        progress_bar.update(1.0 - j as f32 / IMAGE_HEIGHT as f32);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;
                let ray = cam.get_ray(u, v);
                pixel_color += Vec3::from(ray_color(&ray, &world, MAX_DEPTH)) / SAMPLES_PER_PIXEL as f32;
            }

            print!("{}\n", Color::new(
                pixel_color.x().sqrt(),
                pixel_color.y().sqrt(),
                pixel_color.z().sqrt(),
            ));
        }
    }
    eprint!("\nDone!");
}
