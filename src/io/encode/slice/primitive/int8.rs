//! # Wire Format
//!
//! `u8` and `i8` elements use the [`Fixed1Byte`](crate::io::WireType::Fixed1Byte) wire type,
//! so the element count equals the byte count. This skips the
//! [`ListHeader`](crate::io::ListHeader) (whose high 3 bits would just re-encode the
//! already-known element wire type) and writes a raw [`VarIntSize`] length prefix followed by
//! the element bytes. The result saves one byte per encoding for element counts in `31..=127`
//! compared to a [`ListHeader`]-based encoding, and matches the byte length for all other
//! sizes.

use crate::io::Encoder;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

macro_rules! encode_single_byte_slice {
    ($primitive:ty, $to_bytes:expr) => {
        impl EncodedLen for Encoder<'_, Vec<$primitive>> {
            fn encoded_len(&self) -> Result<usize, Error> {
                let len: usize = self.value.len();
                let prefix: usize = VarIntSize::from(len).encoded_len()?;
                Ok(prefix + len)
            }
        }

        impl EncodeToSlice for Encoder<'_, Vec<$primitive>> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                let len: usize = self.value.len();
                let prefix: usize =
                    unsafe { VarIntSize::from(len).encode_to_slice_unchecked(target)? };
                let bytes: &[u8] = $to_bytes(self.value.as_slice());
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        target.as_mut_ptr().add(prefix),
                        len,
                    );
                }
                Ok(prefix + len)
            }
        }

        impl EncodeToWrite for Encoder<'_, Vec<$primitive>> {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
            where
                W: Write,
            {
                let len: usize = self.value.len();
                let prefix: usize = VarIntSize::from(len).encode_to_write(w)?;
                let bytes: &[u8] = $to_bytes(self.value.as_slice());
                w.write_all(bytes)?;
                Ok(prefix + len)
            }
        }
    };
}

fn u8_as_bytes(s: &[u8]) -> &[u8] {
    s
}

fn i8_as_bytes(s: &[i8]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len()) }
}

encode_single_byte_slice!(u8, u8_as_bytes);
encode_single_byte_slice!(i8, i8_as_bytes);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_u8_slice() {
        let cases: &[(&[u8], &[u8])] = &[(&[], &[0]), (&[1, 2, 3], &[3, 1, 2, 3])];

        for (value, expected) in cases {
            let value: Vec<u8> = value.to_vec();
            let encoder: Encoder<'_, Vec<u8>> = Encoder::new(&value, false);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i8_slice() {
        let cases: &[(&[i8], &[u8])] = &[(&[], &[0]), (&[-1, 0, 1], &[3, 0xFF, 0, 1])];

        for (value, expected) in cases {
            let value: Vec<i8> = value.to_vec();
            let encoder: Encoder<'_, Vec<i8>> = Encoder::new(&value, false);
            test::test_encode(&encoder, expected);
        }
    }
}
