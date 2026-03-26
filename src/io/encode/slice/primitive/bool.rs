use crate::io::Encoder;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl EncodedLen for Encoder<'_, Vec<bool>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let len: usize = self.value.len();
        let prefix: usize = VarIntSize::from(len).encoded_len()?;
        Ok(prefix + len)
    }
}

impl EncodeToSlice for Encoder<'_, Vec<bool>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let len: usize = self.value.len();
        let prefix: usize =
            unsafe { VarIntSize::from(len).encode_to_slice_unchecked(target)? };
        for (i, b) in self.value.iter().enumerate() {
            target[prefix + i] = if *b { 1 } else { 0 };
        }
        Ok(prefix + len)
    }
}

impl EncodeToWrite for Encoder<'_, Vec<bool>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let len: usize = self.value.len();
        let prefix: usize = VarIntSize::from(len).encode_to_write(w)?;
        for b in self.value.iter() {
            w.write_all(&[if *b { 1 } else { 0 }])?;
        }
        Ok(prefix + len)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_bool_slice() {
        let value: Vec<bool> = vec![true, false, true];
        let encoder: Encoder<'_, Vec<bool>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[3, 1, 0, 1]);
    }

    #[test]
    fn encode_bool_slice_empty() {
        let value: Vec<bool> = vec![];
        let encoder: Encoder<'_, Vec<bool>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[0]);
    }
}
