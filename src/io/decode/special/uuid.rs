use crate::io::WireType::Fixed16Byte;
use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;
use uuid::Uuid;

impl Decoder {
    //! Decode: `uuid`

    /// Decodes a `uuid` value from the `Read` prefix with the `first` byte.
    pub fn decode_uuid<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Uuid, DecodingError>
    where
        R: Read,
    {
        match wire {
            Fixed16Byte => Ok(Uuid::from_bytes(WireType::decode_fixed_16_byte(r, first)?)),
            _ => Err(DecodingError::InvalidWireType(wire)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};
    use uuid::Uuid;

    #[test]
    fn decode_uuid() {
        let decoder: Decoder = Decoder::default();
        let remaining: [u8; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let result: Uuid = decoder
            .decode_uuid(Fixed16Byte, &mut &remaining[..], 0)
            .unwrap();
        assert_eq!(
            result,
            Uuid::from_bytes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])
        );
    }

    #[test]
    fn decode_nil_uuid() {
        let decoder: Decoder = Decoder::default();
        let remaining: [u8; 15] = [0u8; 15];
        let result: Uuid = decoder
            .decode_uuid(Fixed16Byte, &mut &remaining[..], 0)
            .unwrap();
        assert_eq!(result, Uuid::nil());
    }

    #[test]
    fn decode_invalid_wire_type() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Uuid, DecodingError> =
            decoder.decode_uuid(VarInt, &mut &[][..], 0);
        assert!(matches!(result, Err(DecodingError::InvalidWireType(VarInt))));
    }
}
