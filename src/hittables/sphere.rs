use crate::rays::ray::Ray;
use crate::vectors::vec3::Vec3;

use crate::hittables::hittable::*;
use crate::materials::material::Material;
use crate::utils::vec3_utils::*;

use std::option::Option;
use std::sync::Arc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let hit_point = r.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let mut hit_record = HitRecord {
            t: root,
            p: hit_point,
            normal: Vec3::new(),
            front_face: false,
            mat_ptr: self.mat_ptr.clone(),
        };
        hit_record.set_face_normal(r, &outward_normal);

        Some(hit_record)
    }
}
