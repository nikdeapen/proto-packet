use crate::io::Encoder;
use enc::{EncodeToSlice, EncodedLen, Error, impl_encode_to_write_stack_buf};

impl Encoder<'_, bool> {
    //! Constants

    /// The encoded length of a boolean value.
    const ENCODED_LEN: usize = 1;

    /// Gets the encoded value.
    #[inline(always)]
    pub fn encoded_value(self) -> u8 {
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
    fn encode_bool() {
        let cases: &[(bool, &[u8])] = &[(false, &[0]), (true, &[1])];

        for (value, expected) in cases {
            let encoder: Encoder<'_, bool> = Encoder::new(value, false);
            test::test_encode(&encoder, expected);
        }
    }
}
