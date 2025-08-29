use crate::io::{Decoder, DecodingError, WireType};
use chrono::{Days, NaiveDate};
use std::io::Read;

impl Decoder {
    //! Decode: `date`

    /// Decodes a `date` value from the `Read` prefix with the `first` byte.
    pub fn decode_date<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<NaiveDate, DecodingError>
    where
        R: Read,
    {
        let value: u32 = self.decode_u32(wire, r, first)?;
        Ok(NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .checked_add_days(Days::new(value as u64))
            .unwrap())
    }
}
