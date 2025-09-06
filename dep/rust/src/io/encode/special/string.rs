use crate::io::Encoder;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl EncodedLen for Encoder<'_, String> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let prefix: usize = VarIntSize::from(self.value.len()).encoded_len()?;
        Ok(prefix + self.value.len())
    }
}

impl EncodeToSlice for Encoder<'_, String> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let prefix: usize = VarIntSize::from(self.value.len()).encode_to_slice_unchecked(target)?;
        (&mut target[prefix..(prefix + self.value.len())]).copy_from_slice(self.value.as_bytes());
        Ok(prefix + self.value.len())
    }
}

impl EncodeToWrite for Encoder<'_, String> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let prefix: usize = VarIntSize::from(self.value.len()).encode_to_write(w)?;
        w.write_all(self.value.as_bytes())?;
        Ok(prefix + self.value.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;

    #[test]
    fn encode_string() {
        let test_cases: &[(String, bool, &[u8])] = &[(
            "Hello, World!".to_string(),
            true,
            &[
                13, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!',
            ],
        )];

        for (value, fixed, expected) in test_cases {
            let encoder: Encoder<'_, String> = Encoder::new(value, *fixed);
            enc::test::test_encode(&encoder, *expected);
        }
    }
}
