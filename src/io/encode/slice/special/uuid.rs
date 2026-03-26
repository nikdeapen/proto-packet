use crate::io::{Encoder, ListHeader, WireType};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;
use uuid::Uuid;

impl EncodedLen for Encoder<'_, Vec<Uuid>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let elements: usize = self.value.len() * 16;
        let header: ListHeader = ListHeader::new(WireType::Fixed16Byte, elements);
        Ok(header.encoded_len()? + elements)
    }
}

impl EncodeToSlice for Encoder<'_, Vec<Uuid>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let elements: usize = self.value.len() * 16;
        let header: ListHeader = ListHeader::new(WireType::Fixed16Byte, elements);
        let mut offset: usize = unsafe { header.encode_to_slice_unchecked(target)? };
        for uuid in self.value.iter() {
            target[offset..offset + 16].copy_from_slice(uuid.as_bytes());
            offset += 16;
        }
        Ok(offset)
    }
}

impl EncodeToWrite for Encoder<'_, Vec<Uuid>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let elements: usize = self.value.len() * 16;
        let header: ListHeader = ListHeader::new(WireType::Fixed16Byte, elements);
        let mut written: usize = header.encode_to_write(w)?;
        for uuid in self.value.iter() {
            w.write_all(uuid.as_bytes())?;
            written += 16;
        }
        Ok(written)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;
    use uuid::Uuid;

    #[test]
    fn encode_uuid_slice() {
        let a: Uuid = Uuid::from_bytes([1; 16]);
        let b: Uuid = Uuid::from_bytes([2; 16]);
        let value: Vec<Uuid> = vec![a, b];
        let encoder: Encoder<'_, Vec<Uuid>> = Encoder::new(&value, false);

        // ListHeader: wire=Fixed16Byte(4), size=32 -> overflow (32 > 30)
        // 0x1F = overflow sentinel, high 3 bits = 4 -> 0b100_11111 = 0x9F
        // VarIntSize(32 - 30) = VarIntSize(2) = 0x02
        let mut expected: Vec<u8> = vec![0x9F, 0x02];
        expected.extend_from_slice(&[1; 16]);
        expected.extend_from_slice(&[2; 16]);
        test::test_encode(&encoder, &expected);
    }

    #[test]
    fn encode_uuid_slice_empty() {
        let value: Vec<Uuid> = vec![];
        let encoder: Encoder<'_, Vec<Uuid>> = Encoder::new(&value, false);
        // ListHeader: wire=Fixed16Byte(4), size=0 -> 0b100_00000 = 0x80
        test::test_encode(&encoder, &[0x80]);
    }
}
