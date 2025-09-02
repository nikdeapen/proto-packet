use crate::io::Encoder;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, StreamError};
use std::io::Write;

impl EncodedLen for Encoder<'_, i8> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(1)
    }
}

impl EncodeToSlice for Encoder<'_, i8> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        *target.get_unchecked_mut(0) = (*self.value) as u8;
        Ok(1)
    }
}

impl EncodeToWrite for Encoder<'_, i8> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        w.write_all(&[(*self.value) as u8])?;
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;

    #[test]
    fn encode_i8() {
        let test_cases: &[(i8, bool, &[u8])] = &[
            (0, true, &[0]),
            (0, false, &[0]),
            (i8::MAX, true, &[127]),
            (i8::MAX, false, &[127]),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, i8> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }
}
