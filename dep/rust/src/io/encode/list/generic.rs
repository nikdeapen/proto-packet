use crate::io::WireType::*;
use crate::io::{Encoder, ListHeader};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;
use uuid::Uuid;

macro_rules! util {
    ($target_type:ty) => {
        impl Encoder<'_, Vec<$target_type>> {
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
            unsafe fn list_encode_to_slice_unchecked(
                &self,
                target: &mut [u8],
            ) -> Result<usize, Error> {
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
    };
}

macro_rules! encoded_len {
    ($target_type:ty, $list_wire:ident) => {
        impl EncodedLen for Encoder<'_, Vec<$target_type>> {
            fn encoded_len(&self) -> Result<usize, Error> {
                let size: usize = self.list_encoded_len()?;
                let header: usize = ListHeader::new($list_wire, size).encoded_len()?;
                Ok(header + size)
            }
        }
    };
}

macro_rules! encode_to_slice {
    ($target_type:ty, $list_wire:ident) => {
        impl EncodeToSlice for Encoder<'_, Vec<$target_type>> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                let size: usize = self.list_encoded_len()?;
                let header: usize =
                    ListHeader::new($list_wire, size).encode_to_slice_unchecked(target)?;
                let also_size: usize =
                    self.list_encode_to_slice_unchecked(&mut target[header..])?;
                debug_assert_eq!(size, also_size);
                Ok(header + size)
            }
        }
    };
}

macro_rules! encode_to_write {
    ($target_type:ty, $list_wire:ident) => {
        impl EncodeToWrite for Encoder<'_, Vec<$target_type>> {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
            where
                W: Write,
            {
                let size: usize = self.list_encoded_len()?;
                let header: usize = ListHeader::new($list_wire, size).encode_to_write(w)?;
                let also_size: usize = self.list_encode_to_write(w)?;
                debug_assert_eq!(size, also_size);
                Ok(header + size)
            }
        }
    };
}

macro_rules! encode {
    ($target_type:ty, $list_wire:ident) => {
        util!($target_type);
        encoded_len!($target_type, $list_wire);
        encode_to_slice!($target_type, $list_wire);
        encode_to_write!($target_type, $list_wire);
    };
}

encode!(u16, VarInt);
encode!(u32, VarInt);
encode!(u64, VarInt);
encode!(u128, VarInt);

encode!(Uuid, Fixed16Byte);
encode!(String, LengthPrefixed);
