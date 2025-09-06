use crate::io::encode::list::util::*;
use crate::io::Encoder;
use crate::Packet;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl<P: Packet> EncodedLen for Encoder<'_, Vec<P>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        encoded_len(self.value, P::wire_type(), |value| value.encoded_len())
    }
}

impl<P: Packet> EncodeToSlice for Encoder<'_, Vec<P>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        encode_to_slice(
            self.value,
            P::wire_type(),
            target,
            |value, target| unsafe { value.encode_to_slice_unchecked(target) },
            |value| value.encoded_len(),
        )
    }
}

impl<P: Packet> EncodeToWrite for Encoder<'_, Vec<P>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        encode_to_write(
            self.value,
            P::wire_type(),
            w,
            |value, w| value.encode_to_write(w),
            |value| value.encoded_len(),
        )
    }
}
