/// Convert an array of unsigned 8-bit integers to bits.
/// As bits are 0's and 1's, this functions returns a boolean array. 1 = true, 0 = false.
/// This consumes the bytes, to prevent unwanted memory consumption.
pub fn bytes_to_bits(bytes: Vec<u8>) -> Vec<bool> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);

    for byte in bytes.into_iter() {
        for index in (0..8).rev() {
            bits.push((byte >> index) & 1 != 0);
        }
    }

    bits
}

#[cfg(test)]
mod tests {
    use super::bytes_to_bits;

    #[test]
    fn should_convert_bytes_to_bits_empty() {
        let bits = bytes_to_bits(vec![0]);

        assert_eq!(
            bits,
            &[false, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn should_convert_bytes_to_bits_full() {
        let bits = bytes_to_bits(vec![255]);

        assert_eq!(bits, &[true, true, true, true, true, true, true, true]);
    }

    #[test]
    fn should_convert_bytes_to_bits_one() {
        let bits = bytes_to_bits(vec![1]);

        assert_eq!(
            bits,
            &[false, false, false, false, false, false, false, true]
        );
    }

    #[test]
    fn should_convert_bytes_to_bits_ten() {
        let bits = bytes_to_bits(vec![10]);

        assert_eq!(
            bits,
            &[false, false, false, false, true, false, true, false]
        );
    }

    #[test]
    fn should_convert_multiple_bytes_to_bits() {
        let bits = bytes_to_bits(vec![255, 255]);

        assert_eq!(
            bits,
            &[
                true, true, true, true, true, true, true, true, true, true, true, true, true, true,
                true, true
            ]
        );
    }
}
