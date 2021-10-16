use crate::hittables::hittable::HitRecord;
use crate::rays::ray::Ray;
use crate::vectors::vec3::Vec3;

use std::option::Option;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}
