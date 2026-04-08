//! # Wire Format
//!
//! `bool` elements use the [`Fixed1Byte`](crate::io::WireType::Fixed1Byte) wire type, so the
//! element count equals the byte count. This skips the [`ListHeader`](crate::io::ListHeader)
//! (whose high 3 bits would just re-encode the already-known element wire type) and writes a
//! raw [`VarIntSize`] length prefix followed by the element bytes. The result saves one byte
//! per encoding for element counts in `31..=127` compared to a [`ListHeader`]-based encoding,
//! and matches the byte length for all other sizes.

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
        let prefix: usize = unsafe { VarIntSize::from(len).encode_to_slice_unchecked(target)? };
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.value.as_ptr() as *const u8,
                target.as_mut_ptr().add(prefix),
                len,
            );
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
        let bytes: &[u8] =
            unsafe { std::slice::from_raw_parts(self.value.as_ptr() as *const u8, len) };
        w.write_all(bytes)?;
        Ok(prefix + len)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_bool_slice() {
        let cases: &[(&[bool], &[u8])] = &[
            // empty
            (&[], &[0]),
            // [true, false, true]: varint(3) length prefix, then 3 body bytes
            (&[true, false, true], &[3, 1, 0, 1]),
        ];

        for (value, expected) in cases {
            let value: Vec<bool> = value.to_vec();
            let encoder: Encoder<'_, Vec<bool>> = Encoder::new(&value, false);
            test::test_encode(&encoder, expected);
        }
    }
}
