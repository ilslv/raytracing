use crate::vec3::Vec3;
use crate::point3::Point3;
use crate::ray::Ray;
use std::rc::Rc;
use crate::material::Material;

pub(crate) struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, material: Rc<dyn Material>) -> Self {
        HitRecord { p, normal, t, front_face: (true), material }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub(crate) trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}