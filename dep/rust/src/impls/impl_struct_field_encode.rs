#[macro_export]
macro_rules! impl_struct_field_encoded_len {
    ($field:expr, $fixed:literal, $encoded_len:ident) => {
        $encoded_len += Encoder::new($field, $fixed).encoded_len()?;
    };
}

#[macro_export]
macro_rules! impl_struct_field_encode_to_slice_unchecked {
    ($field:expr, $fixed:literal, $encoded_len:ident, $target:expr) => {
        $encoded_len += Encoder::new($field, $fixed).encode_to_slice_unchecked($target)?;
    };
}

#[macro_export]
macro_rules! impl_struct_field_encode_to_write {
    ($field:expr, $fixed:literal, $encoded_len:ident, $write:expr) => {
        $encoded_len += Encoder::new($field, $fixed).encode_to_write($write)?;
    };
}
