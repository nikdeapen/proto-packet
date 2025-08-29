use crate::io::{Decoder, DecodingError, WireType};
use std::io::Read;
use uuid::Uuid;

macro_rules! impl_decode_slice {
    ($decode_fn_name:ident, $list_element_type:ty, $decode_element_fn_name:ident) => {

        impl Decoder {
            #[doc = concat!("//! Decode: `[]", stringify!($list_element_type), "`")]

            #[doc = concat!("/// Decodes a `[]", stringify!($list_element_type), "` value from the `Read` prefix with the `first` byte.")]
            pub fn $decode_fn_name<R>(
                &self,
                wire: WireType,
                r: &mut R,
                first: u8,
            ) -> Result<Vec<$list_element_type>, DecodingError>
            where
                R: Read,
            {
                self.decode_list(wire, r, first, |wire, r, first| {
                    self.$decode_element_fn_name(wire, r, first)
                })
            }
        }
    };
}

impl_decode_slice!(decode_u16_slice, u16, decode_u16);
impl_decode_slice!(decode_u32_slice, u32, decode_u32);
impl_decode_slice!(decode_u64_slice, u64, decode_u64);
impl_decode_slice!(decode_u128_slice, u128, decode_u128);
impl_decode_slice!(decode_uuid_slice, Uuid, decode_uuid);
impl_decode_slice!(decode_string_slice, String, decode_string);
