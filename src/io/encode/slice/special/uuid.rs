use crate::io::{Encoder, ListHeader, WireType};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;
use uuid::Uuid;

impl Encoder<'_, Vec<Uuid>> {
    //! Utilities

    /// Gets the total encoded length of all elements.
    fn elements_len(&self) -> usize {
        self.value.len() * 16
    }
}

impl EncodedLen for Encoder<'_, Vec<Uuid>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let elements: usize = self.elements_len();
        let header: ListHeader = ListHeader::new(WireType::Fixed16Byte, elements);
        Ok(header.encoded_len()? + elements)
    }
}

impl EncodeToSlice for Encoder<'_, Vec<Uuid>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let elements: usize = self.elements_len();
        let header: ListHeader = ListHeader::new(WireType::Fixed16Byte, elements);
        let mut offset: usize = unsafe { header.encode_to_slice_unchecked(target)? };
        // `Uuid` is `#[repr(transparent)]` over `[u8; 16]`, so `Vec<Uuid>` is laid out as a
        // contiguous run of `len * 16` bytes — copy the whole thing in one shot.
        unsafe {
            std::ptr::copy_nonoverlapping(
                self.value.as_ptr() as *const u8,
                target.as_mut_ptr().add(offset),
                elements,
            );
        }
        offset += elements;
        Ok(offset)
    }
}

impl EncodeToWrite for Encoder<'_, Vec<Uuid>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let elements: usize = self.elements_len();
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
        // Single-element non-overflow: ListHeader{Fixed16Byte, 16} = 0x80 | 16 = 0x90
        let mut nil_single: Vec<u8> = vec![0x90];
        nil_single.extend_from_slice(&[0; 16]);

        let mut max_single: Vec<u8> = vec![0x90];
        max_single.extend_from_slice(&[0xFF; 16]);

        // Two-element overflow: ListHeader{Fixed16Byte, 32} -> 0x9F, varint(32 - 30) = 0x02
        let mut pair: Vec<u8> = vec![0x9F, 0x02];
        pair.extend_from_slice(&[1; 16]);
        pair.extend_from_slice(&[2; 16]);

        let cases: &[(Vec<Uuid>, &[u8])] = &[
            // empty
            (vec![], &[0x80]),
            // single nil
            (vec![Uuid::nil()], nil_single.as_slice()),
            // single max
            (vec![Uuid::from_bytes([0xFF; 16])], max_single.as_slice()),
            // pair (triggers overflow header)
            (
                vec![Uuid::from_bytes([1; 16]), Uuid::from_bytes([2; 16])],
                pair.as_slice(),
            ),
        ];

        for (value, expected) in cases {
            let encoder: Encoder<'_, Vec<Uuid>> = Encoder::new(value, false);
            test::test_encode(&encoder, expected);
        }
    }
}
