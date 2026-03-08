use crate::io::Encoder;
use enc::{EncodeToSlice, EncodedLen, Error, impl_encode_to_write_stack_buf};

impl Encoder<'_, bool> {
    //! Utilities

    /// The encoded length of a boolean value.
    pub const ENCODED_LEN: usize = 1;

    /// Gets the encoded value.
    pub fn encoded_value(&self) -> u8 {
        if *self.value { 1 } else { 0 }
    }
}

impl EncodedLen for Encoder<'_, bool> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(Self::ENCODED_LEN)
    }
}

impl EncodeToSlice for Encoder<'_, bool> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        unsafe {
            *target.get_unchecked_mut(0) = self.encoded_value();
        }
        Ok(Self::ENCODED_LEN)
    }
}

impl_encode_to_write_stack_buf!(Encoder<'_, bool>, Encoder::<'_, bool>::ENCODED_LEN);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_true() {
        let encoder: Encoder<'_, bool> = Encoder::new(&true, false);
        test::test_encode(&encoder, &[1]);
    }

    #[test]
    fn encode_false() {
        let encoder: Encoder<'_, bool> = Encoder::new(&false, false);
        test::test_encode(&encoder, &[0]);
    }
}
