use std::io::{Error, Write};

use enc::var_int::VarIntSize;
use enc::Error::IntegerOverflow;
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen};

use crate::io::WireType::LengthPrefixed;
use crate::io::{FieldHeader, TagNumber};
use crate::Packet;

/// A `Packet` field.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PacketField<'a, P: Packet> {
    tag_number: TagNumber,
    value: &'a P,
}

impl<'a, P: Packet> PacketField<'a, P> {
    //! Construction

    /// Creates a new `PacketField`.
    #[inline(always)]
    pub const fn new(tag_number: TagNumber, value: &'a P) -> Self {
        Self { tag_number, value }
    }

    /// Creates a new `PacketField`.
    #[inline(always)]
    pub const fn from_packet(tag_number: TagNumber, value: &'a P) -> Self {
        Self::new(tag_number, value)
    }
}

impl<'a, P: Packet> PacketField<'a, P> {
    //! Field Header

    /// Gets the field header.
    #[inline(always)]
    pub fn field_header(&self) -> FieldHeader {
        FieldHeader::new(P::wire_type(), self.tag_number)
    }
}

impl<'a, P: Packet> EncodedLen for PacketField<'a, P> {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encoded_len()?;
        let packet_len: usize = self.value.encoded_len()?;
        let prefix_len: usize = if P::wire_type() == LengthPrefixed {
            VarIntSize::from(packet_len).encoded_len()?
        } else {
            0
        };
        (header_len + prefix_len)
            .checked_add(packet_len)
            .ok_or(IntegerOverflow)
    }
}

impl<'a, P: Packet> EncodeToSlice for PacketField<'a, P> {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let header_len: usize = self.field_header().encode_to_slice_unchecked(target)?;
        let prefix_len: usize = if P::wire_type() == LengthPrefixed {
            VarIntSize::from(self.value.encoded_len()?)
                .encode_to_slice_unchecked(&mut target[header_len..])?
        } else {
            0
        };
        let packet_len: usize = self
            .value
            .encode_to_slice_unchecked(&mut target[(header_len + prefix_len)..])?;
        (header_len + prefix_len)
            .checked_add(packet_len)
            .ok_or(IntegerOverflow)
    }
}

impl<'a, P: Packet> EncodeToWrite for PacketField<'a, P> {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let header_len: usize = self.field_header().encode_to_write(w)?;
        let prefix_len: usize = if P::wire_type() == LengthPrefixed {
            VarIntSize::from(self.value.encoded_len()?).encode_to_write(w)?
        } else {
            0
        };
        let packet_len: usize = self.value.encode_to_write(w)?;
        Ok((header_len + prefix_len)
            .checked_add(packet_len)
            .ok_or(IntegerOverflow)?)
    }
}
