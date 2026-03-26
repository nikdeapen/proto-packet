use crate::io::DecodingError;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::List;
use crate::io::{Decoder, ListHeader, WireType};
use enc::{DecodeFromReadPrefix, read_single_byte};
use std::io::{Read, Take};

macro_rules! decode_int_slice {
    ($fn_name:ident, $primitive:ty, $decode_fn:ident) => {
        impl Decoder {
            /// Decodes a `Vec<$primitive>` from the `Read` prefix with the `first` byte.
            pub fn $fn_name<R>(
                &self,
                wire: WireType,
                r: &mut R,
                first: u8,
            ) -> Result<Vec<$primitive>, DecodingError>
            where
                R: Read,
            {
                if wire != List {
                    return Err(InvalidWireType(wire));
                }

                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(DecodingError::from_length_prefix_error)?;

                const _: () = assert!(usize::BITS <= 64);
                let mut r: Take<&mut R> = r.take(header.size() as u64);
                let mut result: Vec<$primitive> = Vec::new();

                while r.limit() > 0 {
                    let first: u8 = read_single_byte(&mut r).map_err(DecodingError::from)?;
                    let value: $primitive = self.$decode_fn(header.wire(), &mut r, first)?;
                    result.push(value);
                }

                Ok(result)
            }
        }
    };
}

decode_int_slice!(decode_u16_slice, u16, decode_u16);
decode_int_slice!(decode_u32_slice, u32, decode_u32);
decode_int_slice!(decode_u64_slice, u64, decode_u64);
decode_int_slice!(decode_u128_slice, u128, decode_u128);
decode_int_slice!(decode_i16_slice, i16, decode_i16);
decode_int_slice!(decode_i32_slice, i32, decode_i32);
decode_int_slice!(decode_i64_slice, i64, decode_i64);
decode_int_slice!(decode_i128_slice, i128, decode_i128);

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_u32_slice_varint() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=VarInt(5), size=3 -> 0xA3
        // Elements: varint(1)=0x01, varint(2)=0x02, varint(3)=0x03
        let result: Vec<u32> = decoder
            .decode_u32_slice(List, &mut &[1u8, 2, 3][..], 0xA3)
            .unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn decode_u32_slice_empty() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=VarInt(5), size=0 -> 0xA0
        let result: Vec<u32> = decoder.decode_u32_slice(List, &mut &[][..], 0xA0).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn decode_i32_slice_varint() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=VarInt(5), size=3 -> 0xA3
        // Elements: zigzag(-1)=1, zigzag(0)=0, zigzag(1)=2
        let result: Vec<i32> = decoder
            .decode_i32_slice(List, &mut &[1u8, 0, 2][..], 0xA3)
            .unwrap();
        assert_eq!(result, vec![-1, 0, 1]);
    }

    #[test]
    fn decode_u32_slice_invalid_wire() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Vec<u32>, DecodingError> =
            decoder.decode_u32_slice(VarInt, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(VarInt))
        ));
    }
}
