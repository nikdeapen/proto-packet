use crate::io::Encoder;
use chrono::NaiveDate;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl Encoder<'_, NaiveDate> {
    //! Encoded Value

    fn encoded_value(&self) -> i64 {
        let epoch: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        self.value.signed_duration_since(epoch).num_days()
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
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        Encoder::new(&self.encoded_value(), self.fixed).encode_to_write(w)
    }
}
