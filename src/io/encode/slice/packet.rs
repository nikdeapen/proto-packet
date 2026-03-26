use crate::Packet;
use crate::io::{Encoder, ListHeader};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

impl<P: Packet> Encoder<'_, Vec<P>> {
    //! Utilities

    /// Gets the total encoded length of all elements.
    fn elements_len(&self) -> Result<usize, Error> {
        let mut len: usize = 0;
        for element in self.value.iter() {
            let encoder: Encoder<'_, P> = Encoder::new(element, self.fixed);
            len += encoder.encoded_len()?;
        }
        Ok(len)
    }

    /// Creates the list header for the encoded elements.
    fn list_header(&self) -> Result<ListHeader, Error> {
        let elements: usize = self.elements_len()?;
        Ok(ListHeader::new(P::wire(), elements))
    }
}

impl<P: Packet> EncodedLen for Encoder<'_, Vec<P>> {
    fn encoded_len(&self) -> Result<usize, Error> {
        let elements: usize = self.elements_len()?;
        let header: ListHeader = ListHeader::new(P::wire(), elements);
        Ok(header.encoded_len()? + elements)
    }
}

impl<P: Packet> EncodeToSlice for Encoder<'_, Vec<P>> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let header: ListHeader = self.list_header()?;
        let mut offset: usize = unsafe { header.encode_to_slice_unchecked(target)? };
        for element in self.value.iter() {
            let encoder: Encoder<'_, P> = Encoder::new(element, self.fixed);
            offset += unsafe { encoder.encode_to_slice_unchecked(&mut target[offset..])? };
        }
        Ok(offset)
    }
}

impl<P: Packet> EncodeToWrite for Encoder<'_, Vec<P>> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let header: ListHeader = self.list_header()?;
        let mut written: usize = header.encode_to_write(w)?;
        for element in self.value.iter() {
            let encoder: Encoder<'_, P> = Encoder::new(element, self.fixed);
            written += encoder.encode_to_write(w)?;
        }
        Ok(written)
    }
}
