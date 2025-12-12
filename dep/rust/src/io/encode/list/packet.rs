use crate::io::WireType::List;
use crate::io::{Encoder, ListHeader};
use crate::Packet;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl<P: Packet> Encoder<'_, Vec<P>> {
    //! Utilities

    /// Gets the encoded length of the encoded `list`.
    fn list_encoded_len(&self) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        for value in self.value {
            encoded_len += Encoder::new(value, self.fixed).encoded_len()?;
        }
        Ok(encoded_len)
    }

    /// Encodes the list to the `target` slice.
    unsafe fn list_encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let mut encoded_len: usize = 0;
        for value in self.value {
            encoded_len += Encoder::new(value, self.fixed)
                .encode_to_slice_unchecked(&mut target[encoded_len..])?;
        }
        Ok(encoded_len)
    }

    /// Encodes the list to the `Write`.
    fn list_encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let mut encoded_len: usize = 0;
        for value in self.value {
            encoded_len += Encoder::new(value, self.fixed).encode_to_write(w)?;
        }
        Ok(encoded_len)
    }
}

impl<P: Packet> EncodedLen for Encoder<'_, Vec<P>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let size: usize = self.list_encoded_len()?;
        let header: usize = ListHeader::new(List, size).encoded_len()?;
        Ok(header + size)
    }
}

impl<P: Packet> EncodeToSlice for Encoder<'_, Vec<P>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let size: usize = self.list_encoded_len()?;
        let header: usize = ListHeader::new(List, size).encode_to_slice_unchecked(target)?;
        let also_size: usize = self.list_encode_to_slice_unchecked(&mut target[header..])?;
        debug_assert_eq!(size, also_size);
        Ok(header + size)
    }
}

impl<P: Packet> EncodeToWrite for Encoder<'_, Vec<P>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let size: usize = self.list_encoded_len()?;
        let header: usize = ListHeader::new(List, size).encode_to_write(w)?;
        let also_size: usize = self.list_encode_to_write(w)?;
        debug_assert_eq!(size, also_size);
        Ok(header + size)
    }
}
