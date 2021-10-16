use crate::vectors::vec3::Vec3;

use super::random_number_utils::random_f64_range;

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        vector3: [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ],
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * dot(v, n) * (*n)
}

pub fn min(x: f64, y: f64) -> f64 {
    match x < y {
        true => x,
        false => y,
    }
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = min(dot(&(-*uv), n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * (*n));
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * (*n);
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            vector3: [
                random_f64_range(-1.0, 1.0),
                random_f64_range(-1.0, 1.0),
                0.0,
            ],
        };
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot() {
        let test_vector = Vec3 {
            vector3: [1f64, 1f64, 1f64],
        };
        assert_eq!(dot(&test_vector, &test_vector), 3f64);
        let test_vector2 = Vec3 {
            vector3: [1f64, 2f64, 3f64],
        };
        assert_eq!(dot(&test_vector, &test_vector2), 6f64);
    }

    #[test]
    fn test_cross_itself() {
        let test_vector = Vec3 {
            vector3: [1f64, 1f64, 1f64],
        };
        let test_vector2 = Vec3 {
            vector3: [1f64, 1f64, 1f64],
        };
        let zero_vector = Vec3 {
            vector3: [0f64, 0f64, 0f64],
        };
        assert_eq!(cross(&test_vector, &test_vector2), zero_vector);
    }

    #[test]
    fn test_cross_orthogonal() {
        let x_vector = Vec3 {
            vector3: [1f64, 0f64, 0f64],
        };
        let y_vector2 = Vec3 {
            vector3: [0f64, 1f64, 0f64],
        };
        let z_vector = Vec3 {
            vector3: [0f64, 0f64, 1f64],
        };
        assert_eq!(cross(&x_vector, &y_vector2), z_vector);
    }

    #[test]
    fn test_unit_vector() {
        let test_vector = Vec3 {
            vector3: [1f64, 1f64, 1f64],
        };
        debug_assert_eq!(unit_vector(test_vector), test_vector / 3.0f64.sqrt());
    }
}
