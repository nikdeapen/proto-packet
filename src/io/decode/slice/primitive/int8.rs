use crate::io::DecodingError;
use crate::io::DecodingError::InvalidWireType;
use crate::io::WireType::LengthPrefixed;
use crate::io::{Decoder, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarIntSize;
use std::io::Read;

impl Decoder {
    //! Decode: `Vec<u8>`

    /// Decodes a `Vec<u8>` from the `Read` prefix with the `first` byte.
    pub fn decode_u8_slice<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<u8>, DecodingError>
    where
        R: Read,
    {
        if wire != LengthPrefixed {
            return Err(InvalidWireType {
                semantic: "Vec<u8>",
                wire,
            });
        }

        let len: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(DecodingError::from_length_prefix_error)?
            .value();

        let mut bytes: Vec<u8> = vec![0u8; len];
        r.read_exact(&mut bytes)?;
        Ok(bytes)
    }
}

impl Decoder {
    //! Decode: `Vec<i8>`

    /// Decodes a `Vec<i8>` from the `Read` prefix with the `first` byte.
    pub fn decode_i8_slice<R>(
        &self,
        wire: WireType,
        r: &mut R,
        first: u8,
    ) -> Result<Vec<i8>, DecodingError>
    where
        R: Read,
    {
        if wire != LengthPrefixed {
            return Err(InvalidWireType {
                semantic: "Vec<i8>",
                wire,
            });
        }

        let len: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(DecodingError::from_length_prefix_error)?
            .value();

        let mut bytes: Vec<u8> = vec![0u8; len];
        r.read_exact(&mut bytes)?;

        // Reinterpret the `Vec<u8>` buffer as a `Vec<i8>` without copying. `u8` and `i8` have the
        // same size and alignment, so the existing allocation can be re-owned by a `Vec<i8>`.
        let cap: usize = bytes.capacity();
        let ptr: *mut i8 = bytes.as_mut_ptr() as *mut i8;
        std::mem::forget(bytes);
        Ok(unsafe { Vec::from_raw_parts(ptr, len, cap) })
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType::*;
    use crate::io::{Decoder, DecodingError};

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
        assert!(matches!(
            result,
            Err(DecodingError::InvalidWireType {
                semantic: "Vec<u8>",
                wire: VarInt
            })
        ));
    }
}
