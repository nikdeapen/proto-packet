use crate::io::DecodingError;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::List;
use crate::io::{Decoder, ListHeader, WireType};
use enc::{DecodeFromReadPrefix, read_single_byte};
use std::io::{Read, Take};
use uuid::Uuid;

impl Decoder {
    //! Decode: `Vec<Uuid>`

    /// Decodes a `Vec<Uuid>` from the `Read` prefix with the `first` byte.
    pub fn decode_uuid_slice<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<Uuid>, DecodingError>
    where
        R: Read,
    {
        if wire != List {
            return Err(InvalidWireType(wire));
        }

        let header: ListHeader =
            ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                .map_err(DecodingError::from_length_prefix_error)?;

        if header.wire() != WireType::Fixed16Byte {
            return Err(InvalidWireType(header.wire()));
        }

        const _: () = assert!(usize::BITS <= 64);
        let mut r: Take<&mut R> = r.take(header.size() as u64);
        let mut result: Vec<Uuid> = Vec::new();

        while r.limit() > 0 {
            let first: u8 = read_single_byte(&mut r).map_err(DecodingError::from)?;
            let uuid: Uuid = self.decode_uuid(header.wire(), &mut r, first)?;
            result.push(uuid);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::{Decoder, DecodingError};
    use crate::io::WireType::*;
    use uuid::Uuid;

    #[test]
    fn decode_uuid_slice() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=Fixed16Byte(4), size=32 -> overflow: 0x9F, VarIntSize(2) = 0x02
        let mut data: Vec<u8> = vec![0x02];
        data.extend_from_slice(&[1; 16]);
        data.extend_from_slice(&[2; 16]);

        let result: Vec<Uuid> = decoder
            .decode_uuid_slice(List, &mut data.as_slice(), 0x9F)
            .unwrap();
        assert_eq!(result, vec![Uuid::from_bytes([1; 16]), Uuid::from_bytes([2; 16])]);
    }

    #[test]
    fn decode_uuid_slice_empty() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=Fixed16Byte(4), size=0 -> 0x80
        let result: Vec<Uuid> = decoder
            .decode_uuid_slice(List, &mut &[][..], 0x80)
            .unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn decode_uuid_slice_invalid_wire() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Vec<Uuid>, DecodingError> =
            decoder.decode_uuid_slice(VarInt, &mut &[][..], 0);
        assert!(matches!(result, Err(DecodingError::InvalidWireType(VarInt))));
    }
}
