use crate::io::{DecodingError, WireType};
use enc::DecodeFromReadPrefix;
use enc::var_int::VarIntSize;
use std::io::{Error, Read};

macro_rules! decode_fixed {
    ($name:ident, $size:expr, $wire:literal) => {
        #[doc = concat!("Decodes a `", $wire, "` value from the `Read` prefix given the `first` byte.")]
        pub fn $name<R>(r: &mut R, first: u8) -> Result<[u8; $size], Error>
        where
            R: Read,
        {
            let mut buffer: [u8; $size] = [0u8; $size];
            buffer[0] = first;
            r.read_exact(&mut buffer[1..])?;
            Ok(buffer)
        }
    };
}

impl WireType {
    //! Decode

    decode_fixed!(decode_fixed_2_byte, 2, "Fixed2Byte");
    decode_fixed!(decode_fixed_4_byte, 4, "Fixed4Byte");
    decode_fixed!(decode_fixed_8_byte, 8, "Fixed8Byte");
    decode_fixed!(decode_fixed_16_byte, 16, "Fixed16Byte");

    /// Decodes a `LengthPrefixed` `[]u8` value from the `Read` prefix given the `first` byte.
    pub fn decode_length_prefixed_bytes<R>(r: &mut R, first: u8) -> Result<Vec<u8>, DecodingError>
    where
        R: Read,
    {
        let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(DecodingError::from_length_prefix_error)?
            .value();
        let mut result: Vec<u8> = Vec::with_capacity(prefix);
        let read: usize = r.take(prefix as u64).read_to_end(&mut result)?;
        if read != prefix {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "short read for length-prefixed bytes",
            )
            .into());
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType;
    use std::io::{Cursor, Error};

    #[test]
    fn decode_fixed_2_byte() -> Result<(), Error> {
        let mut source: Cursor<Vec<u8>> = Cursor::new(vec![1, 2]);

        let result: [u8; 2] = WireType::decode_fixed_2_byte(&mut source, 0)?;
        assert_eq!(result, [0, 1]);

        Ok(())
    }

    #[test]
    fn decode_fixed_4_byte() -> Result<(), Error> {
        let mut source: Cursor<Vec<u8>> = Cursor::new(vec![1, 2, 3, 4]);

        let result: [u8; 4] = WireType::decode_fixed_4_byte(&mut source, 0)?;
        assert_eq!(result, [0, 1, 2, 3]);

        Ok(())
    }

    #[test]
    fn decode_fixed_8_byte() -> Result<(), Error> {
        let mut source: Cursor<Vec<u8>> = Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);

        let result: [u8; 8] = WireType::decode_fixed_8_byte(&mut source, 0)?;
        assert_eq!(result, [0, 1, 2, 3, 4, 5, 6, 7]);

        Ok(())
    }

    #[test]
    fn decode_fixed_16_byte() -> Result<(), Error> {
        let mut source: Cursor<Vec<u8>> =
            Cursor::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

        let result: [u8; 16] = WireType::decode_fixed_16_byte(&mut source, 0)?;
        assert_eq!(
            result,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );

        Ok(())
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn decode_length_prefixed_bytes() {
        // 200-byte body with multi-byte varint prefix. After consuming first=0xC8, the
        // continuation byte 0x01 still in `rest` completes the varint to 200, then 200 body bytes.
        let mut large_rest: Vec<u8> = vec![0x01];
        large_rest.extend(std::iter::repeat_n(0xAB, 200));
        let large_expected: Vec<u8> = vec![0xAB; 200];

        // Each case: (first_byte, rest_bytes, Some(expected) for Ok, None for Err).
        let cases: &[(u8, &[u8], Option<&[u8]>)] = &[
            // Empty body: first=0 = varint(0), no body bytes.
            (0, &[], Some(&[])),
            // Small body: first=3 = varint(3), then 3 body bytes.
            (3, &[1, 2, 3], Some(&[1, 2, 3])),
            // Body with extra bytes — only consumes the declared 3.
            (3, &[1, 2, 3, 99], Some(&[1, 2, 3])),
            // Multi-byte varint prefix: first=0xC8 (continuation), rest starts with 0x01 to
            // complete varint(200), then 200 body bytes.
            (0xC8, large_rest.as_slice(), Some(large_expected.as_slice())),
            // Body shorter than declared length — should error with EOF.
            (5, &[1, 2, 3], None),
        ];

        for (first, rest, expected) in cases {
            let mut source: Cursor<&[u8]> = Cursor::new(*rest);
            let result = WireType::decode_length_prefixed_bytes(&mut source, *first);
            match (result, expected) {
                (Ok(bytes), Some(exp)) => {
                    assert_eq!(
                        bytes.as_slice(),
                        *exp,
                        "first={first:#x} rest_len={}",
                        rest.len()
                    );
                }
                (Err(_), None) => {}
                (Ok(bytes), None) => panic!(
                    "first={first:#x} rest_len={}: expected error, got Ok({} bytes)",
                    rest.len(),
                    bytes.len()
                ),
                (Err(e), Some(_)) => panic!(
                    "first={first:#x} rest_len={}: expected Ok, got error: {e:?}",
                    rest.len()
                ),
            }
        }
    }
}
