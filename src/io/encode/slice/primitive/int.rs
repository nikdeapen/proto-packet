use crate::io::{Encoder, ListHeader, WireType};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

macro_rules! encode_int_slice {
    ($primitive:ident, $var_int:ident, $convert:ident, $fixed_wire:expr) => {
        impl Encoder<'_, Vec<$primitive>> {
            //! Utilities

            /// Gets the wire type for encoding elements.
            fn wire_type(&self) -> WireType {
                if self.fixed {
                    $fixed_wire
                } else {
                    WireType::VarInt
                }
            }

            /// Gets the total encoded length of all elements.
            fn elements_len(&self) -> Result<usize, Error> {
                let mut len: usize = 0;
                for element in self.value.iter() {
                    let encoder: Encoder<'_, $primitive> = Encoder::new(element, self.fixed);
                    len += encoder.encoded_len()?;
                }
                Ok(len)
            }
        }

        impl EncodedLen for Encoder<'_, Vec<$primitive>> {
            fn encoded_len(&self) -> Result<usize, Error> {
                let elements: usize = self.elements_len()?;
                let header: ListHeader = ListHeader::new(self.wire_type(), elements);
                Ok(header.encoded_len()? + elements)
            }
        }

        impl EncodeToSlice for Encoder<'_, Vec<$primitive>> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                let elements: usize = self.elements_len()?;
                let header: ListHeader = ListHeader::new(self.wire_type(), elements);
                let mut offset: usize = unsafe { header.encode_to_slice_unchecked(target)? };
                for element in self.value.iter() {
                    let encoder: Encoder<'_, $primitive> = Encoder::new(element, self.fixed);
                    offset += unsafe { encoder.encode_to_slice_unchecked(&mut target[offset..])? };
                }
                Ok(offset)
            }
        }

        impl EncodeToWrite for Encoder<'_, Vec<$primitive>> {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
            where
                W: Write,
            {
                let elements: usize = self.elements_len()?;
                let header: ListHeader = ListHeader::new(self.wire_type(), elements);
                let mut written: usize = header.encode_to_write(w)?;
                for element in self.value.iter() {
                    let encoder: Encoder<'_, $primitive> = Encoder::new(element, self.fixed);
                    written += encoder.encode_to_write(w)?;
                }
                Ok(written)
            }
        }
    };
}

encode_int_slice!(u16, VarInt16, from, WireType::Fixed2Byte);
encode_int_slice!(u32, VarInt32, from, WireType::Fixed4Byte);
encode_int_slice!(u64, VarInt64, from, WireType::Fixed8Byte);
encode_int_slice!(u128, VarInt128, from, WireType::Fixed16Byte);
encode_int_slice!(i16, VarInt16, from_zigzag, WireType::Fixed2Byte);
encode_int_slice!(i32, VarInt32, from_zigzag, WireType::Fixed4Byte);
encode_int_slice!(i64, VarInt64, from_zigzag, WireType::Fixed8Byte);
encode_int_slice!(i128, VarInt128, from_zigzag, WireType::Fixed16Byte);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_u32_slice_varint() {
        // ListHeader: wire=VarInt(5), size=3 -> high 3 bits = 0b101, low 5 bits = 3 -> 0xA3
        // Elements: varint(1)=0x01, varint(2)=0x02, varint(3)=0x03
        let value: Vec<u32> = vec![1, 2, 3];
        let encoder: Encoder<'_, Vec<u32>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[0xA3, 1, 2, 3]);
    }

    #[test]
    fn encode_u32_slice_empty() {
        // ListHeader: wire=VarInt(5), size=0 -> 0xA0
        let value: Vec<u32> = vec![];
        let encoder: Encoder<'_, Vec<u32>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[0xA0]);
    }

    #[test]
    fn encode_i32_slice_varint() {
        // ListHeader: wire=VarInt(5), size=3 -> 0xA3
        // Elements: zigzag(-1)=1, zigzag(0)=0, zigzag(1)=2
        let value: Vec<i32> = vec![-1, 0, 1];
        let encoder: Encoder<'_, Vec<i32>> = Encoder::new(&value, false);
        test::test_encode(&encoder, &[0xA3, 1, 0, 2]);
    }
}
