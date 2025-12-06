use crate::io::WireType::*;
use crate::io::{ListHeader, WireType};
use enc::var_int::VarIntSize;
use enc::{read_single_byte, DecodeFromReadPrefix, EncodeToWrite};
use std::io::{Error, Read, Take, Write};

impl WireType {
    //! Transfer

    /// Transfers the wire type data from the `Read` to the `Write`.
    pub fn transfer<R, W>(&self, r: &mut R, w: &mut W) -> Result<(), Error>
    where
        R: Read,
        W: Write,
    {
        let first: u8 = read_single_byte(r)?;
        self.transfer_with_first_byte(r, w, first)
    }

    /// Transfers wire type from data from the `Read` to the `Write` with the `first` byte.
    pub fn transfer_with_first_byte<R, W>(
        &self,
        r: &mut R,
        w: &mut W,
        first: u8,
    ) -> Result<(), Error>
    where
        R: Read,
        W: Write,
    {
        match self {
            Fixed1Byte => w.write_all(&[first])?,
            Fixed2Byte => {
                let b: [u8; 2] = Self::decode_fixed_2_byte(r, first)?;
                w.write_all(&b)?;
            }
            Fixed4Byte => {
                let b: [u8; 4] = Self::decode_fixed_4_byte(r, first)?;
                w.write_all(&b)?;
            }
            Fixed8Byte => {
                let b: [u8; 8] = Self::decode_fixed_8_byte(r, first)?;
                w.write_all(&b)?;
            }
            Fixed16Byte => {
                let b: [u8; 16] = Self::decode_fixed_16_byte(r, first)?;
                w.write_all(&b)?;
            }
            VarInt => {
                let mut current: u8 = first;
                loop {
                    w.write_all(&[current])?;
                    if current & 0x10 == 0 {
                        break;
                    }
                    current = read_single_byte(r)?;
                }
            }
            LengthPrefixed => {
                let prefix: VarIntSize =
                    VarIntSize::decode_from_read_prefix_with_first_byte(r, first)?;
                prefix.encode_to_write(w)?;
                const _: () = assert!(usize::BITS <= 64);
                let mut r: Take<&mut R> = r.take(prefix.value() as u64);
                std::io::copy(&mut r, w)?;
            }
            List => {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)?;
                header.encode_to_write(w)?;
                const _: () = assert!(usize::BITS <= 64);
                let mut r: Take<&mut R> = r.take(header.size() as u64);
                std::io::copy(&mut r, w)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType;
    use crate::io::WireType::*;
    use std::io::{Cursor, Error};

    #[test]
    fn transfer() -> Result<(), Error> {
        let test_cases: &[(WireType, &[u8], &[u8])] = &[
            (Fixed1Byte, &[1, 2], &[1]),
            (Fixed2Byte, &[1, 2, 3], &[1, 2]),
            (Fixed4Byte, &[1, 2, 3, 4, 5], &[1, 2, 3, 4]),
            (
                Fixed8Byte,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9],
                &[1, 2, 3, 4, 5, 6, 7, 8],
            ),
            (
                Fixed16Byte,
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
                &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            ),
            (VarInt, &[1, 2, 3], &[1]),
            (VarInt, &[0xFF, 1, 2, 3], &[0xFF, 1]),
            (LengthPrefixed, &[3, 1, 2, 3, 4], &[3, 1, 2, 3]),
            (List, &[0x83, 1, 2, 3, 4], &[0x83, 1, 2, 3]),
        ];

        for (wire, input, expected) in test_cases {
            let mut source: Cursor<Vec<u8>> = Cursor::new(Vec::from(*input));
            let mut sink: Cursor<Vec<u8>> = Cursor::new(Vec::default());

            wire.transfer(&mut source, &mut sink)?;
            assert_eq!(sink.into_inner().as_slice(), *expected);
        }

        Ok(())
    }
}
