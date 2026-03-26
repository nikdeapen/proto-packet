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
                target[prefix..prefix + len].copy_from_slice(bytes);
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
        let value: Vec<u8> = vec![1, 2, 3];
        let encoder: Encoder<'_, Vec<u8>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[3, 1, 2, 3]);
    }

    #[test]
    fn encode_u8_slice_empty() {
        let value: Vec<u8> = vec![];
        let encoder: Encoder<'_, Vec<u8>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[0]);
    }

    #[test]
    fn encode_i8_slice() {
        let value: Vec<i8> = vec![-1, 0, 1];
        let encoder: Encoder<'_, Vec<i8>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[3, 0xFF, 0, 1]);
    }
}
