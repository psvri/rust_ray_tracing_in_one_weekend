use crate::materials::material::Material;
use crate::rays::ray::Ray;
use crate::utils::vec3_utils::*;
use crate::vectors::vec3::Vec3;

use std::option::Option;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal,
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMaterial;

    impl Material for TestMaterial {
        fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
            None
        }
    }

    #[test]
    fn test_set_face_normal() {
        let material = Arc::new(TestMaterial);
        let mut hr = HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 1.0,
            front_face: false,
            mat_ptr: material.clone(),
        };
        let ray = Ray {
            orig: Vec3::new_with_values(0f64, 0f64, 0f64),
            dir: Vec3::new_with_values(0f64, 0f64, 1f64),
        };
        let outward_normal = Vec3::new_with_values(1.0, 0.0, 0.0);
        hr.set_face_normal(&ray, &outward_normal);
        assert_eq!(hr.front_face, false);
        assert_eq!(hr.normal, -outward_normal);
    }
}
