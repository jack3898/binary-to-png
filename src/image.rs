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

/// Using the bit length, get the optimal image dimensions
/// If a height is provided, then the width calculated to satisfy the height
/// If a width is provided, then the height is calculated to satisfy the width
/// If both the width and height are provided, the bit length is discarded, and they are re-returned.
pub fn bits_to_image_dimensions(
    bit_len: usize,
    width: Option<u32>,
    height: Option<u32>,
) -> (u32, u32) {
    let bit_len = bit_len as u32;

    let (final_width, final_height) = match (width, height) {
        (Some(width), Some(height)) => (width, height),
        (Some(width), None) => {
            let auto_height = (bit_len as f64 / width as f64).ceil() as u32;

            (width, auto_height)
        }
        (None, Some(height)) => {
            let auto_width = (bit_len as f64 / height as f64).ceil() as u32;

            (auto_width, height)
        }
        (None, None) => {
            let bit_len_sqrt = (bit_len as f64).sqrt().ceil() as u32;

            (bit_len_sqrt, bit_len_sqrt)
        }
    };

    (final_width, final_height)
}

#[cfg(test)]
mod tests {
    use crate::image::bits_to_image_dimensions;

    #[test]
    fn should_get_best_image_size_for_byte_length_moderate_fill() {
        assert_eq!(bits_to_image_dimensions(26, None, None), (6, 6));
    }

    #[test]
    fn should_get_best_image_size_for_byte_length_fill() {
        assert_eq!(bits_to_image_dimensions(36, None, None), (6, 6));
    }

    #[test]
    fn should_get_best_image_size_for_byte_length_1_too_many() {
        assert_eq!(bits_to_image_dimensions(37, None, None), (7, 7));
    }

    #[test]
    fn should_override_height_and_calculate_width() {
        assert_eq!(bits_to_image_dimensions(8, Some(1), None), (1, 8));
    }

    #[test]
    fn should_override_width_and_calculate_height() {
        assert_eq!(bits_to_image_dimensions(8, None, Some(1)), (8, 1));
    }

    #[test]
    fn should_override_width_and_height() {
        assert_eq!(bits_to_image_dimensions(8, Some(10), Some(10)), (10, 10));
    }
}
