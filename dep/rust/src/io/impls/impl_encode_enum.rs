#[macro_export]
macro_rules! impl_encode_enum {
    ($name:ident) => {
        impl enc::EncodedLen for $name {
            fn encoded_len(&self) -> Result<usize, enc::Error> {
                use enc::var_int::VarInt32;
                use proto_packet::io::WithTagNumber;

                let tag_number: u32 = self.tag_number().value();
                let tag_number: VarInt32 = VarInt32::from(tag_number);
                tag_number.encoded_len()
            }
        }

        impl enc::EncodeToSlice for $name {
            unsafe fn encode_to_slice_unchecked(
                &self,
                target: &mut [u8],
            ) -> Result<usize, enc::Error> {
                use enc::var_int::VarInt32;
                use proto_packet::io::WithTagNumber;

                let tag_number: u32 = self.tag_number().value();
                let tag_number: VarInt32 = VarInt32::from(tag_number);
                tag_number.encode_to_slice_unchecked(target)
            }
        }

        enc::impl_encode_to_write_stack_buf!($name, enc::var_int::VarInt32::MAX_ENCODED_LEN);
    };
}
