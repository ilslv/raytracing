use crate::point3::Point3;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::angle::{Degrees, Radians};

pub(crate) struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        origin: Point3,
        destination: Point3,
        view_up: Vec3,
        vertical_fov: Degrees,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = Radians::from(vertical_fov);
        let h = (f32::from(theta) / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (Vec3::from(origin) - Vec3::from(destination)).unit_vec();
        let u = Vec3::cross(&view_up, &w).unit_vec();
        let v = Vec3::cross(&w, &u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            (self.lower_left_corner + u * self.horizontal + v * self.vertical - Vec3::from(self.origin) - offset).into(),
        )
    }
}