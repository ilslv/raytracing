use crate::point3::Point3;
use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;
use crate::material::Material;

pub(crate) struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center.into();
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc.into(), &ray.direction);
        let c = Vec3::length_squared(&oc.into())- self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant <= 0.0 {
            return None;
        }

        let root = discriminant.sqrt();

        let temp: f32;
        if t_min < (-half_b - root) / a && (-half_b - root) / a < t_max {
            temp = (-half_b - root) / a;
        } else if t_min < (-half_b + root) / a && (-half_b + root) / a < t_max {
            temp = (-half_b + root) / a;
        } else {
            return None;
        }

        let t = temp;
        let p = ray.at(t);
        let normal = (Vec3::from(p) - Vec3::from(self.center)) / self.radius;
        let outward_normal = (Vec3::from(p) - Vec3::from(self.center)) / self.radius;

        let mut rec = HitRecord::new(p, normal, t, self.material.clone());
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}