use crate::io::Encoder;
use enc::var_int::VarIntSize;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl EncodedLen for Encoder<'_, String> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let prefix: usize = VarIntSize::from(self.value.len()).encoded_len()?;
        Ok(prefix + self.value.len())
    }
}

impl EncodeToSlice for Encoder<'_, String> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let prefix: usize =
            unsafe { VarIntSize::from(self.value.len()).encode_to_slice_unchecked(target)? };
        let bytes: &[u8] = self.value.as_bytes();
        unsafe {
            std::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                target.as_mut_ptr().add(prefix),
                bytes.len(),
            );
        }
        Ok(prefix + bytes.len())
    }
}

impl EncodeToWrite for Encoder<'_, String> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let prefix: usize = VarIntSize::from(self.value.len()).encode_to_write(w)?;
        w.write_all(self.value.as_bytes())?;
        Ok(prefix + self.value.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;

    #[test]
    fn encode_string() {
        // 200 'a's: varint(200) = [0xC8, 0x01], then 200 body bytes.
        let long_value: String = "a".repeat(200);
        let mut long_expected: Vec<u8> = Vec::with_capacity(2 + 200);
        long_expected.push(0xC8);
        long_expected.push(0x01);
        long_expected.extend(std::iter::repeat_n(b'a', 200));

        let cases: &[(&str, &[u8])] = &[
            // Empty string.
            ("", &[0]),
            // Single ASCII byte.
            ("a", &[1, b'a']),
            // ASCII string.
            (
                "Hello, World!",
                &[
                    13, b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd',
                    b'!',
                ],
            ),
            // Multi-byte UTF-8 (é = 0xC3 0xA9): 5 chars, 6 bytes.
            ("héllo", &[6, b'h', 0xC3, 0xA9, b'l', b'l', b'o']),
            // 4-byte UTF-8 (🦀 = 0xF0 0x9F 0xA6 0x80): 1 char, 4 bytes.
            ("🦀", &[4, 0xF0, 0x9F, 0xA6, 0x80]),
            // Long string forcing a 2-byte length prefix.
            (long_value.as_str(), long_expected.as_slice()),
        ];

        for (value, expected) in cases {
            let value: String = (*value).to_string();
            let encoder: Encoder<'_, String> = Encoder::new(&value, false);
            enc::test::test_encode(&encoder, expected);
        }
    }
}
