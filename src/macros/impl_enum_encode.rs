/// Implements `EncodedLen`, `EncodeToSlice`, and `EncodeToWrite` for an enum type.
///
/// The enum must implement `proto_packet::io::WithTagNumber`.
#[macro_export]
macro_rules! impl_enum_encode {
    ($enum_type:ty) => {
        impl enc::EncodedLen for $enum_type {
            fn encoded_len(&self) -> Result<usize, enc::Error> {
                use $crate::io::WithTagNumber;
                enc::var_int::VarInt32::from(self.tag().value()).encoded_len()
            }
        }

        impl enc::EncodeToSlice for $enum_type {
            unsafe fn encode_to_slice_unchecked(
                &self,
                target: &mut [u8],
            ) -> Result<usize, enc::Error> {
                use $crate::io::WithTagNumber;
                unsafe {
                    enc::var_int::VarInt32::from(self.tag().value())
                        .encode_to_slice_unchecked(target)
                }
            }
        }

        enc::impl_encode_to_write_stack_buf!($enum_type, enc::var_int::VarInt32::MAX_ENCODED_LEN);
    };
}
