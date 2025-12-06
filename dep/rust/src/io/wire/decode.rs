use crate::io::{DecodingError, WireType};
use enc::var_int::VarIntSize;
use enc::{read_single_byte, DecodeFromReadPrefix};
use std::io::{Error, Read};

impl WireType {
    //! Decode

    /// Decodes a `Fixed2Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_2_byte<R>(r: &mut R, first: u8) -> Result<[u8; 2], Error>
    where
        R: Read,
    {
        Ok([first, read_single_byte(r)?])
    }

    /// Decodes a `Fixed4Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_4_byte<R>(r: &mut R, first: u8) -> Result<[u8; 4], Error>
    where
        R: Read,
    {
        let mut buffer: [u8; 4] = [0u8; 4];
        buffer[0] = first;
        r.read_exact(&mut buffer[1..])?;
        Ok(buffer)
    }

    /// Decodes a `Fixed8Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_8_byte<R>(r: &mut R, first: u8) -> Result<[u8; 8], Error>
    where
        R: Read,
    {
        let mut buffer: [u8; 8] = [0u8; 8];
        buffer[0] = first;
        r.read_exact(&mut buffer[1..])?;
        Ok(buffer)
    }

    /// Decodes a `Fixed16Byte` value from the `Read` prefix given the `first` byte.
    pub fn decode_fixed_16_byte<R>(r: &mut R, first: u8) -> Result<[u8; 16], Error>
    where
        R: Read,
    {
        let mut buffer: [u8; 16] = [0u8; 16];
        buffer[0] = first;
        r.read_exact(&mut buffer[1..])?;
        Ok(buffer)
    }

    /// Decodes a `LengthPrefixed` `[]u8` value from the `Read` prefix given the `first` byte.
    pub fn decode_length_prefixed_bytes<R>(r: &mut R, first: u8) -> Result<Vec<u8>, DecodingError>
    where
        R: Read,
    {
        let prefix: usize = VarIntSize::decode_from_read_prefix_with_first_byte(r, first)
            .map_err(|e| DecodingError::from_length_prefix_error(e))?
            .value();
        // todo -- may improve performance with `unsafe`
        let mut result: Vec<u8> = vec![0; prefix];
        r.read_exact(&mut result)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::io::{DecodingError, WireType};
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
    fn decode_length_prefixed_bytes() -> Result<(), DecodingError> {
        let mut source: Cursor<Vec<u8>> = Cursor::new(vec![1, 2, 3, 4]);

        let result: Vec<u8> = WireType::decode_length_prefixed_bytes(&mut source, 3)?;
        assert_eq!(result.as_slice(), &[1, 2, 3]);

        Ok(())
    }
}
