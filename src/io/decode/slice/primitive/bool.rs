use crate::io::DecodingError;
use crate::io::DecodingError::{InvalidBool, InvalidWireType};
use crate::io::WireType::LengthPrefixed;
use crate::io::{Decoder, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarIntSize;
use std::io::Read;

impl Decoder {
    //! Decode: `Vec<bool>`

    /// Decodes a `Vec<bool>` from the `Read` prefix with the `first` byte.
    pub fn decode_bool_slice<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<bool>, DecodingError>
    where
        R: Read,
    {
        if wire != LengthPrefixed {
            return Err(InvalidWireType(wire));
        }

        let len: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(DecodingError::from_length_prefix_error)?
            .value();

        let mut bytes: Vec<u8> = vec![0u8; len];
        r.read_exact(&mut bytes)?;

        let mut result: Vec<bool> = Vec::with_capacity(len);
        for b in bytes {
            match b {
                0 => result.push(false),
                1 => result.push(true),
                _ => return Err(InvalidBool(b)),
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_bool_slice() {
        let decoder: Decoder = Decoder::default();
        let result: Vec<bool> = decoder
            .decode_bool_slice(LengthPrefixed, &mut &[1u8, 0, 1][..], 3)
            .unwrap();
        assert_eq!(result, vec![true, false, true]);
    }

    #[test]
    fn decode_bool_slice_empty() {
        let decoder: Decoder = Decoder::default();
        let result: Vec<bool> = decoder
            .decode_bool_slice(LengthPrefixed, &mut &[][..], 0)
            .unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn decode_bool_slice_invalid_value() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Vec<bool>, DecodingError> =
            decoder.decode_bool_slice(LengthPrefixed, &mut &[2u8][..], 1);
        assert!(matches!(result, Err(DecodingError::InvalidBool(2))));
    }

    #[test]
    fn decode_bool_slice_invalid_wire() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Vec<bool>, DecodingError> =
            decoder.decode_bool_slice(VarInt, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(VarInt))
        ));
    }
}
