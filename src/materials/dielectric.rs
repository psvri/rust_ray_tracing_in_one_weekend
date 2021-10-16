use crate::{
    hittables::hittable::HitRecord,
    rays::ray::Ray,
    utils::{
        random_number_utils::random_f64,
        vec3_utils::{dot, min, reflect, refract, unit_vector},
    },
    vectors::vec3::Vec3,
};

use super::material::Material;

use std::option::Option;

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = Vec3::new_with_values(1.0, 1.0, 1.0);
        let refraction_ratio = match rec.front_face {
            true => 1.0 / self.ir,
            false => self.ir,
        };
        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio)
        }

        let scattered = Ray {
            orig: rec.p,
            dir: direction,
        };
        Some((scattered, attenuation))
    }
}
