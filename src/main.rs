mod vec3;
mod color;
mod point3;
mod ray;
mod hit;
mod sphere;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::point3::Point3;

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 384;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center.0;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(&oc.into(), &ray.direction);
    let c = oc.0.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray) -> Color {
    let mut t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = Vec3::unit_vec(ray.at(t).0 - Vec3::new(0.0, 0.0, -1.0));
        return (0.5 * Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)).into();
    }

    let unit_direction = ray.direction.unit_vec();
    t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)).into()
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

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Lines remained {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;

            let r = Ray::new(
                origin,
                (lower_left_corner + u * horizontal + v * vertical - origin.into()).into(),
            );

            let pixel_color = ray_color(&r);

            print!("{}\n", pixel_color);
        }
    }
    eprintln!("Done!");
}
