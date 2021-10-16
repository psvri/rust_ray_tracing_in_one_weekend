use image::RgbImage;

pub fn write_image(image_buffer: RgbImage, output_path: &str) {
    image_buffer
        .save(output_path)
        .expect("Could not save image due to ");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;
    use std::path::Path;
    #[test]
    fn test_write_image() {
        let img: RgbImage = RgbImage::new(256 as u32, 256 as u32);
        let path_string = "test.png";
        let path = Path::new(path_string);
        write_image(img, path_string);
        assert_eq!(path.exists(), true);
        remove_file(path_string).expect(&format!("Could not delete test image at {}", path_string));
    }
}
