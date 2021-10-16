use crate::vectors::vec3::Vec3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_origin() {
        let ray = Ray {
            orig: Vec3::new_with_values(0f64, 0f64, 0f64),
            dir: Vec3::new_with_values(1f64, 0f64, 0f64),
        };
        assert_eq!(ray.origin(), Vec3::new());
    }

    #[test]
    fn test_ray_direction() {
        let ray = Ray {
            orig: Vec3::new_with_values(0f64, 0f64, 0f64),
            dir: Vec3::new_with_values(1f64, 0f64, 0f64),
        };
        assert_eq!(ray.direction(), Vec3::new_with_values(1f64, 0f64, 0f64));
    }

    #[test]
    fn test_ray_at() {
        let ray = Ray {
            orig: Vec3::new_with_values(0f64, 0f64, 0f64),
            dir: Vec3::new_with_values(1f64, 0f64, 0f64),
        };
        assert_eq!(ray.at(2f64), Vec3::new_with_values(2f64, 0f64, 0f64));
    }
}
