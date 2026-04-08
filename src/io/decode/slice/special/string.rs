use crate::io::DecodingError;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::List;
use crate::io::{Decoder, ListHeader, WireType};
use enc::{DecodeFromReadPrefix, read_single_byte};
use std::io::{Read, Take};

impl Decoder {
    //! Decode: `Vec<String>`

    /// Decodes a `Vec<String>` from the `Read` prefix with the `first` byte.
    pub fn decode_string_slice<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<String>, DecodingError>
    where
        R: Read,
    {
        if wire != List {
            return Err(InvalidWireType {
                semantic: "Vec<String>",
                wire,
            });
        }

        let header: ListHeader = ListHeader::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(DecodingError::from_length_prefix_error)?;

        if header.wire() != WireType::LengthPrefixed {
            return Err(InvalidWireType {
                semantic: "Vec<String>",
                wire: header.wire(),
            });
        }

        const _: () = assert!(usize::BITS <= 64);
        let mut r: Take<&mut R> = r.take(header.size() as u64);
        let mut result: Vec<String> = Vec::with_capacity(header.element_capacity_hint());

        while r.limit() > 0 {
            let first: u8 = read_single_byte(&mut r).map_err(DecodingError::from)?;
            let value: String = self.decode_string(header.wire(), &mut r, first)?;
            result.push(value);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

    #[test]
    fn decode_string_slice() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=LengthPrefixed(6), size=5 -> 0xC5
        // Element "ab": len_prefix=2, data=b"ab"
        // Element "c":  len_prefix=1, data=b"c"
        let data: &[u8] = &[2, b'a', b'b', 1, b'c'];
        let result: Vec<String> = decoder
            .decode_string_slice(List, &mut &data[..], 0xC5)
            .unwrap();
        assert_eq!(result, vec!["ab".to_string(), "c".to_string()]);
    }

    #[test]
    fn decode_string_slice_empty() {
        let decoder: Decoder = Decoder::default();
        // ListHeader: wire=LengthPrefixed(6), size=0 -> 0xC0
        let result: Vec<String> = decoder
            .decode_string_slice(List, &mut &[][..], 0xC0)
            .unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn decode_string_slice_invalid_wire() {
        let decoder: Decoder = Decoder::default();
        let result: Result<Vec<String>, DecodingError> =
            decoder.decode_string_slice(VarInt, &mut &[][..], 0);
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType {
                semantic: "Vec<String>",
                wire: VarInt
            })
        ));
    }
}
