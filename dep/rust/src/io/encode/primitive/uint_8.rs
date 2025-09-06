use crate::io::Encoder;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl EncodedLen for Encoder<'_, u8> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(1)
    }
}

impl EncodeToSlice for Encoder<'_, u8> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        *target.get_unchecked_mut(0) = *self.value;
        Ok(1)
    }
}

impl EncodeToWrite for Encoder<'_, u8> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        w.write_all(&[*self.value])?;
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;

    #[test]
    fn encode_u8() {
        let test_cases: &[(u8, bool, &[u8])] = &[
            (0, true, &[0]),
            (0, false, &[0]),
            (0x7F, true, &[0x7F]),
            (0x7F, false, &[0x7F]),
            (u8::MAX, true, &[255]),
            (u8::MAX, false, &[255]),
        ];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, u8> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }
}
