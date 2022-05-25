use crate::vectors::vec3::Vec3;

pub fn gamma_correct(vec3: &Vec3, scale: f64) -> Vec3 {
    Vec3::new_with_values(
        vec3[0].powf(scale),
        vec3[1].powf(scale),
        vec3[2].powf(scale),
    )
}

pub fn convert_vec3_to_color(vec3: Vec3) -> [u8; 3] {
    [
        (vec3.x() * 255.0) as u8,
        (vec3.y() * 255.0) as u8,
        (vec3.z() * 255.0) as u8,
    ]
}
