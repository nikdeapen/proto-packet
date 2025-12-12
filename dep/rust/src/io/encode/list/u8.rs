use crate::io::Encoder;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl EncodedLen for Encoder<'_, Vec<u8>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let prefix: usize = VarIntSize::from(self.value.len()).encoded_len()?;
        Ok(prefix + self.value.len())
    }
}

impl EncodeToSlice for Encoder<'_, Vec<u8>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let prefix: usize = VarIntSize::from(self.value.len()).encode_to_slice_unchecked(target)?;
        target[prefix..(prefix + self.value.len())].copy_from_slice(self.value);
        Ok(prefix + self.value.len())
    }
}

impl EncodeToWrite for Encoder<'_, Vec<u8>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let prefix: usize = VarIntSize::from(self.value.len()).encode_to_write(w)?;
        w.write_all(self.value)?;
        Ok(prefix + self.value.len())
    }
}
