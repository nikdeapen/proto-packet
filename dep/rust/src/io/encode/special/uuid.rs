use crate::io::Encoder;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;
use uuid::Uuid;

impl EncodedLen for Encoder<'_, Uuid> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(16)
    }
}

impl EncodeToSlice for Encoder<'_, Uuid> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        (&mut target[..16]).copy_from_slice(self.value.as_bytes());
        Ok(16)
    }
}

impl EncodeToWrite for Encoder<'_, Uuid> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        w.write_all(self.value.as_bytes())?;
        Ok(16)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use uuid::Uuid;

    #[test]
    fn encode_uuid() {
        let test_cases: &[(Uuid, bool, &[u8])] = &[(
            Uuid::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            true,
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        )];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, Uuid> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }
}
