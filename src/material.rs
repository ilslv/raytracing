use crate::color::Color;
use crate::ray::Ray;
use crate::hit::HitRecord;
use crate::vec3::Vec3;
use std::rc::Rc;
use rand::Rng;

pub(crate) struct AttenuatedRay {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl AttenuatedRay {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        AttenuatedRay { attenuation, scattered }
    }
}

pub(crate) trait Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<AttenuatedRay>;
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<AttenuatedRay> {
        let scatter_direction = hit_rec.normal + Vec3::random_unit_vector();
        Some(AttenuatedRay::new(
            self.albedo,
            Ray::new(hit_rec.p, scatter_direction),
        ))
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Metal {
    albedo: Color,
    roughness: f32,
}

impl Metal {
    pub fn new(albedo: Color, roughness: f32) -> Self {
        Metal { albedo, roughness: f32::min(roughness, 1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<AttenuatedRay> {
        let reflected = Vec3::reflect(&ray.direction.unit_vec(), &hit_rec.normal);
        let scattered = Ray::new(hit_rec.p, reflected + self.roughness * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;

        if Vec3::dot(&scattered.direction, &hit_rec.normal) <= 0.0 {
            return None;
        }
        Some(AttenuatedRay::new(attenuation, scattered))
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Dielectric {
    refraction_idx: f32,
}

impl Dielectric {
    pub fn new(refraction_idx: f32) -> Self {
        Dielectric { refraction_idx }
    }

    fn schlick(cosine: f32, refraction_idx: f32) -> f32 {
        let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<AttenuatedRay> {
        let etai_over_etat = if hit_rec.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let mut rng = rand::thread_rng();
        let unit_direction = ray.direction.unit_vec();
        let cos_theta = f32::min(Vec3::dot(&-unit_direction, &hit_rec.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
        let reflect_probability = Dielectric::schlick(cos_theta, etai_over_etat);

        let direction = if etai_over_etat * sin_theta > 1.0 || rng.gen::<f32>() < reflect_probability {
            Vec3::reflect(&unit_direction, &hit_rec.normal)
        } else {
            Vec3::refract(unit_direction, hit_rec.normal, etai_over_etat)
        };

        Some(AttenuatedRay::new(
            Color::new(1.0, 1.0, 1.0),
            Ray::new(hit_rec.p, direction),
        ))
    }
}
