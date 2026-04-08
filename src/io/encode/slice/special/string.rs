use crate::io::{Encoder, ListHeader, WireType};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl Encoder<'_, Vec<String>> {
    //! Utilities

    /// Gets the total encoded length of all elements.
    fn elements_len(&self) -> Result<usize, Error> {
        let mut len: usize = 0;
        for element in self.value.iter() {
            let encoder: Encoder<'_, String> = Encoder::new(element, self.fixed);
            len += encoder.encoded_len()?;
        }
        Ok(len)
    }
}

impl EncodedLen for Encoder<'_, Vec<String>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let elements: usize = self.elements_len()?;
        let header: ListHeader = ListHeader::new(WireType::LengthPrefixed, elements);
        Ok(header.encoded_len()? + elements)
    }
}

impl EncodeToSlice for Encoder<'_, Vec<String>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let elements: usize = self.elements_len()?;
        let header: ListHeader = ListHeader::new(WireType::LengthPrefixed, elements);
        let mut offset: usize = unsafe { header.encode_to_slice_unchecked(target)? };
        for element in self.value.iter() {
            let encoder: Encoder<'_, String> = Encoder::new(element, self.fixed);
            offset +=
                unsafe { encoder.encode_to_slice_unchecked(target.get_unchecked_mut(offset..))? };
        }
        Ok(offset)
    }
}

impl EncodeToWrite for Encoder<'_, Vec<String>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let elements: usize = self.elements_len()?;
        let header: ListHeader = ListHeader::new(WireType::LengthPrefixed, elements);
        let mut written: usize = header.encode_to_write(w)?;
        for element in self.value.iter() {
            let encoder: Encoder<'_, String> = Encoder::new(element, self.fixed);
            written += encoder.encode_to_write(w)?;
        }
        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_string_slice() {
        let cases: &[(&[&str], &[u8])] = &[
            // empty: ListHeader{LengthPrefixed, 0} = 0xC0
            (&[], &[0xC0]),
            // ["ab", "c"]: each element is its own length-prefixed string.
            // "ab" -> [2, b'a', b'b'] (3 bytes), "c" -> [1, b'c'] (2 bytes), total 5 body bytes.
            // ListHeader{LengthPrefixed, 5} = 0xC0 | 5 = 0xC5
            (&["ab", "c"], &[0xC5, 2, b'a', b'b', 1, b'c']),
        ];

        for (value, expected) in cases {
            let value: Vec<String> = value.iter().map(|s| (*s).to_string()).collect();
            let encoder: Encoder<'_, Vec<String>> = Encoder::new(&value, false);
            test::test_encode(&encoder, expected);
        }
    }
}
