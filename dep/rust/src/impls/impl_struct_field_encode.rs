#[macro_export]
macro_rules! impl_struct_field_encoded_len {
    ($encoded_len:ident, $field:expr, $fixed:literal) => {
        $encoded_len += $crate::io::Encoder::new($field, $fixed).encoded_len()?;
    };
}

#[macro_export]
macro_rules! impl_struct_field_encode_to_slice_unchecked {
    ($encoded_len:ident, $field:expr, $fixed:literal, $target:expr) => {
        $encoded_len +=
            $crate::io::Encoder::new($field, $fixed).encode_to_slice_unchecked($target)?;
    };
}

#[macro_export]
macro_rules! impl_struct_field_encode_to_write {
    ($encoded_len:ident, $field:expr, $fixed:literal, $write:expr) => {
        $encoded_len += $crate::io::Encoder::new($field, $fixed).encode_to_write($write)?;
    };
}
