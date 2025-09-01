use crate::io::Encoder;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, StreamError};
use std::io::Write;

impl Encoder<'_, bool> {
    //! Encoded Value

    /// Gets the encoded value.
    #[inline(always)]
    pub fn encoded_value(&self) -> u8 {
        if *self.value {
            1
        } else {
            0
        }
    }
}

impl EncodedLen for Encoder<'_, bool> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(1)
    }
}

impl EncodeToSlice for Encoder<'_, bool> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        *target.get_unchecked_mut(0) = self.encoded_value();
        Ok(1)
    }
}

impl EncodeToWrite for Encoder<'_, bool> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        w.write_all(&[self.encoded_value()])?;
        Ok(1)
    }
}
