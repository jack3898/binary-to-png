use image::{ColorType, DynamicImage, GenericImage, Rgba};

pub fn bits_to_image_buf(width: u32, height: u32, bits: Vec<bool>) -> DynamicImage {
    let mut image = DynamicImage::new(width, height, ColorType::Rgba8);
    let black_pixel = Rgba([0, 0, 0, 255]);
    let white_pixel = Rgba([255, 255, 255, 255]);

    for (index, bit) in bits.into_iter().enumerate() {
        let x = (index as u32) % width;
        let y = (index as u32) / width;
        let white_or_black_pixel = if bit { white_pixel } else { black_pixel };

        image.put_pixel(x, y, white_or_black_pixel);
    }

    return DynamicImage::from(image);
}

pub fn bits_to_image_dimensions(bit_len: usize) -> (u32, u32) {
    let sqrt_of_byte_length = (bit_len as f64).sqrt().ceil() as u32;

    (sqrt_of_byte_length, sqrt_of_byte_length)
}

#[cfg(test)]
mod tests {
    use crate::image::bits_to_image_dimensions;

    #[test]
    fn should_get_best_image_size_for_byte_length_moderate_fill() {
        assert_eq!(bits_to_image_dimensions(26), (6, 6));
    }

    #[test]
    fn should_get_best_image_size_for_byte_length_fill() {
        assert_eq!(bits_to_image_dimensions(36), (6, 6));
    }

    #[test]
    fn should_get_best_image_size_for_byte_length_1_too_many() {
        assert_eq!(bits_to_image_dimensions(37), (7, 7));
    }
}
