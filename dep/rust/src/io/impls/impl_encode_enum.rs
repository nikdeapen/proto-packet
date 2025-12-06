#[macro_export]
macro_rules! impl_encode_enum {
    ($target:ident) => {
        impl enc::EncodedLen for $target {
            fn encoded_len(&self) -> Result<usize, enc::Error> {
                use enc::var_int::VarInt32;
                use proto_packet::io::WithTagNumber;

                let tag: u32 = self.tag().value();
                let tag: VarInt32 = VarInt32::from(tag);
                tag.encoded_len()
            }
        }

        impl enc::EncodeToSlice for $target {
            unsafe fn encode_to_slice_unchecked(
                &self,
                target: &mut [u8],
            ) -> Result<usize, enc::Error> {
                use enc::var_int::VarInt32;
                use proto_packet::io::WithTagNumber;

                let tag: u32 = self.tag().value();
                let tag: VarInt32 = VarInt32::from(tag);
                tag.encode_to_slice_unchecked(target)
            }
        }

        enc::impl_encode_to_write_stack_buf!($target, enc::var_int::VarInt32::MAX_ENCODED_LEN);
    };
}
