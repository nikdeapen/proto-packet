use crate::io::Encoder;
use chrono::NaiveDate;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error, StreamError};
use std::io::Write;

impl Encoder<'_, NaiveDate> {
    //!

    fn encoded_value(&self) -> u32 {
        let epoch: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        let days: i64 = self.value.signed_duration_since(epoch).num_days();
        days as u32 // todo -- lol
    }
}

impl EncodedLen for Encoder<'_, NaiveDate> {
    fn encoded_len(&self) -> Result<usize, Error> {
        Encoder::new(&self.encoded_value(), self.fixed).encoded_len()
    }
}

impl EncodeToSlice for Encoder<'_, NaiveDate> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        Encoder::new(&self.encoded_value(), self.fixed).encode_to_slice_unchecked(target)
    }
}

impl EncodeToWrite for Encoder<'_, NaiveDate> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, StreamError>
    where
        W: Write,
    {
        Encoder::new(&self.encoded_value(), self.fixed).encode_to_write(w)
    }
}
