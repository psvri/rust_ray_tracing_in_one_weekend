use crate::{
    hittables::hittable::HitRecord,
    rays::ray::Ray,
    utils::vec3_utils::{dot, reflect, unit_vector},
    vectors::vec3::Vec3,
};

use super::material::Material;

use std::option::Option;

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        let scattered = Ray {
            orig: rec.p,
            dir: (reflected + self.fuzz * Vec3::random_in_unit_sphere()),
        };
        let attenuation = self.albedo;
        match dot(&scattered.direction(), &rec.normal) > 0.0 {
            true => Some((scattered, attenuation)),
            false => None,
        }
    }
}
