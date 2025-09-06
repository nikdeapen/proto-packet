use crate::io::encode::list::util::{encode_to_slice, encode_to_write, encoded_len};
use crate::io::{Encoder, WireType};
use enc::var_int::{VarInt128, VarInt16, VarInt32, VarInt64};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

macro_rules! impl_encode_list_uint {
    ($uint_type:ty, $fixed_wire:expr,  $var_int_type:ident) => {
        impl EncodedLen for Encoder<'_, Vec<$uint_type>> {
            fn encoded_len(&self) -> Result<usize, Error> {
                let wire: WireType = if self.fixed {
                    $fixed_wire
                } else {
                    WireType::VarInt
                };
                encoded_len(self.value, wire, |value| {
                    $var_int_type::from(value).encoded_len()
                })
            }
        }

        impl EncodeToSlice for Encoder<'_, Vec<$uint_type>> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                let wire: WireType = if self.fixed {
                    $fixed_wire
                } else {
                    WireType::VarInt
                };
                encode_to_slice(
                    self.value,
                    wire,
                    target,
                    |value, target| $var_int_type::from(value).encode_to_slice(target),
                    |value| $var_int_type::from(value).encoded_len(),
                )
            }
        }

        impl EncodeToWrite for Encoder<'_, Vec<$uint_type>> {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
            where
                W: Write,
            {
                let wire: WireType = if self.fixed {
                    $fixed_wire
                } else {
                    WireType::VarInt
                };
                encode_to_write(
                    self.value,
                    wire,
                    w,
                    |value, w| $var_int_type::from(value).encode_to_write(w),
                    |value| $var_int_type::from(value).encoded_len(),
                )
            }
        }
    };
}

impl_encode_list_uint!(u16, WireType::Fixed2Byte, VarInt16);
impl_encode_list_uint!(u32, WireType::Fixed4Byte, VarInt32);
impl_encode_list_uint!(u64, WireType::Fixed8Byte, VarInt64);
impl_encode_list_uint!(u128, WireType::Fixed16Byte, VarInt128);
