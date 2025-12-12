use crate::io::decode::list::util::decode_generic_list;
use crate::io::{Decoder, DecodingError, ListHeader};
use enc::DecodeFromReadPrefix;
use std::io::Read;
use uuid::Uuid;

macro_rules! decode {
    ($target_type:ty, $fn_name:ident, $base_fn_name:ident) => {
        impl Decoder {
            //! todo -- docs

            // todo -- docs`
            pub fn $fn_name<R>(
                &self,
                r: &mut R,
                first: u8,
            ) -> Result<Vec<$target_type>, DecodingError>
            where
                R: Read,
            {
                let header: ListHeader =
                    ListHeader::decode_from_read_prefix_with_first_byte(r, first)
                        .map_err(DecodingError::from_list_header_error)?;
                decode_generic_list(r, header, |wire, r, first| {
                    self.$base_fn_name(wire, r, first)
                })
            }
        }
    };
}

decode!(u16, decode_u16_list, decode_u16);
decode!(u32, decode_u32_list, decode_u32);
decode!(u64, decode_u64_list, decode_u64);
decode!(u128, decode_u128_list, decode_u128);

decode!(Uuid, decode_uuid_list, decode_uuid);
decode!(String, decode_string_list, decode_string);
