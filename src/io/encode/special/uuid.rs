use crate::io::Encoder;
use enc::{EncodeToSlice, EncodedLen, Error, impl_encode_to_write_stack_buf};
use uuid::Uuid;

impl Encoder<'_, Uuid> {
    //! Constants

    /// The fixed encoded length. (16)
    const FIXED_ENCODED_LEN: usize = 16;
}

impl EncodedLen for Encoder<'_, Uuid> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(Self::FIXED_ENCODED_LEN)
    }
}

impl EncodeToSlice for Encoder<'_, Uuid> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let bytes: &[u8; 16] = self.value.as_bytes();
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                target.as_mut_ptr(),
                Self::FIXED_ENCODED_LEN,
            );
        }
        Ok(Self::FIXED_ENCODED_LEN)
    }
}

impl_encode_to_write_stack_buf!(Encoder<'_, Uuid>, Encoder::<'_, Uuid>::FIXED_ENCODED_LEN);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use uuid::Uuid;

    #[test]
    fn encode_uuid() {
        let cases: &[(Uuid, &[u8])] = &[
            // Nil UUID.
            (Uuid::nil(), &[0u8; 16]),
            // Sequential bytes.
            (
                Uuid::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
                &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            ),
            // Max UUID.
            (Uuid::from_bytes([0xFF; 16]), &[0xFF; 16]),
        ];

        for (value, expected) in cases {
            let encoder: Encoder<'_, Uuid> = Encoder::new(value, false);
            enc::test::test_encode(&encoder, expected);
        }
    }
}
