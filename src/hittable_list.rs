use std::rc::Rc;
use crate::hit::{Hit, HitRecord};
use crate::ray::Ray;

#[derive(Default)]
pub(crate) struct HittableList {
    objects: Vec<Rc<dyn Hit>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hit>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hit for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        self.objects
            .iter()
            .filter_map(|object| {
                object
                    .hit(ray, t_min, closest_so_far)
                    .and_then(|hit_rec| {
                        closest_so_far = hit_rec.t;
                        Some(hit_rec)
                    })
            })
            .last()
    }
}
