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

/// Convert an array of unsigned bits to an array of unsigned 8-bit integers.
/// This consumes the bits, to prevent unwanted memory consumption.
pub fn bits_to_bytes(bits: Vec<bool>) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(bits.len() / 8);

    for chunk in bits.chunks(8) {
        let mut byte = 0u8; // start at zero

        for (index, &bit) in chunk.into_iter().enumerate() {
            if bit {
                let lonely_bit_in_byte: u8 = 1 << (7 - index);

                byte |= lonely_bit_in_byte;
            }
        }

        bytes.push(byte);
    }

    bytes
}

#[cfg(test)]
mod test_bytes_to_bits {
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

#[cfg(test)]
mod test_bits_to_bytes {
    use super::bits_to_bytes;

    #[test]
    fn should_convert_bits_to_bytes_empty() {
        let bits = bits_to_bytes(vec![false, false, false, false, false, false, false, false]);

        assert_eq!(bits, &[0]);
    }

    #[test]
    fn should_convert_bits_to_bytes_full() {
        let bits = bits_to_bytes(vec![true, true, true, true, true, true, true, true]);

        assert_eq!(bits, &[255]);
    }

    #[test]
    fn should_convert_bits_to_bytes_one() {
        let bits = bits_to_bytes(vec![false, false, false, false, false, false, false, true]);

        assert_eq!(bits, &[1]);
    }

    #[test]
    fn should_convert_bits_to_bytes_ten() {
        let bits = bits_to_bytes(vec![false, false, false, false, true, false, true, false]);

        assert_eq!(bits, &[10]);
    }

    #[test]
    fn should_convert_multiple_bits_to_bytes() {
        let bits = bits_to_bytes(vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true,
        ]);

        assert_eq!(bits, &[255, 255]);
    }
}
