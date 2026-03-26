use crate::io::DecodingError;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::LengthPrefixed;
use crate::io::{Decoder, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarIntSize;
use std::io::Read;

macro_rules! decode_int8_slice {
    ($fn_name:ident, $primitive:ty, $convert:expr) => {
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
                if wire != LengthPrefixed {
                    return Err(InvalidWireType(wire));
                }

                let len: usize =
                    VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(DecodingError::from_length_prefix_error)?
                        .value();

                let mut bytes: Vec<u8> = vec![0u8; len];
                r.read_exact(&mut bytes)?;

                Ok($convert(bytes))
            }
        }
    };
}

fn bytes_to_u8(bytes: Vec<u8>) -> Vec<u8> {
    bytes
}

fn bytes_to_i8(bytes: Vec<u8>) -> Vec<i8> {
    unsafe { std::mem::transmute::<Vec<u8>, Vec<i8>>(bytes) }
}

decode_int8_slice!(decode_u8_slice, u8, bytes_to_u8);
decode_int8_slice!(decode_i8_slice, i8, bytes_to_i8);

#[cfg(test)]
mod tests {
    use crate::io::{Decoder, DecodingError};
    use crate::io::WireType::*;

    #[test]
    fn decode_u8_slice() {
        let decoder: Decoder = Decoder::default();
        let result: Vec<u8> = decoder
            .decode_u8_slice(LengthPrefixed, &mut &[1u8, 2, 3][..], 3)
            .unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn decode_u8_slice_empty() {
        let decoder: Decoder = Decoder::default();
        let result: Vec<u8> = decoder
            .decode_u8_slice(LengthPrefixed, &mut &[][..], 0)
            .unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn decode_i8_slice() {
        let decoder: Decoder = Decoder::default();
        let result: Vec<i8> = decoder
            .decode_i8_slice(LengthPrefixed, &mut &[0xFFu8, 0, 1][..], 3)
            .unwrap();
        assert_eq!(result, vec![-1, 0, 1]);
    }

    #[test]
    fn decode_u8_slice_invalid_wire() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Vec<u8>, DecodingError> =
            decoder.decode_u8_slice(VarInt, &mut &[][..], 0);
        assert!(matches!(result, Err(DecodingError::InvalidWireType(VarInt))));
    }
}
