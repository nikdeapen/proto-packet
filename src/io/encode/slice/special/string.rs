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
            offset += unsafe { encoder.encode_to_slice_unchecked(&mut target[offset..])? };
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
        let value: Vec<String> = vec!["ab".to_string(), "c".to_string()];
        let encoder: Encoder<'_, Vec<String>> = Encoder::new(&value, false);
        // ListHeader: wire=LengthPrefixed(6), size=5 -> 0b110_00101 = 0xC5
        // Element "ab": len_prefix=2, data=b"ab" -> [2, b'a', b'b']
        // Element "c":  len_prefix=1, data=b"c"  -> [1, b'c']
        test::test_encode(&encoder, &[0xC5, 2, b'a', b'b', 1, b'c']);
    }

    #[test]
    fn encode_string_slice_empty() {
        let value: Vec<String> = vec![];
        let encoder: Encoder<'_, Vec<String>> = Encoder::new(&value, false);
        // ListHeader: wire=LengthPrefixed(6), size=0 -> 0b110_00000 = 0xC0
        test::test_encode(&encoder, &[0xC0]);
    }
}
