use crate::io::DecodingError::*;
use crate::io::WireType::*;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;

impl Decoder {
    //! Decode: `bool`

    /// Decodes the `bool` value from the `Read` prefix with the `first` byte.
    pub fn decode_bool<R>(
        &self,
        wire: WireType,
        _r: &mut R, // this parameter is present for code-generation consistency
        first: u8,
    ) -> Result<bool, DecodingError>
    where
        R: Read,
    {
        Ok(match wire {
            Fixed1Byte => match first {
                0 => false,
                1 => true,
                _ => return Err(InvalidBool(first)),
            },
            _ => return Err(InvalidWireType(wire)),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_false() {
        let decoder: Decoder = Decoder::default();
        let result: bool = decoder.decode_bool(Fixed1Byte, &mut &[][..], 0).unwrap();
        assert!(!result);
    }

    #[test]
    fn decode_true() {
        let decoder: Decoder = Decoder::default();
        let result: bool = decoder.decode_bool(Fixed1Byte, &mut &[][..], 1).unwrap();
        assert!(result);
    }

    #[test]
    fn decode_invalid_value() {
        let decoder: Decoder = Decoder::default();
        let result: Result<bool, DecodingError> = decoder.decode_bool(Fixed1Byte, &mut &[][..], 2);
        assert!(matches!(result, Err(DecodingError::InvalidBool(2))));
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<bool, DecodingError> = decoder.decode_bool(VarInt, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType(VarInt))
        ));
    }
}
