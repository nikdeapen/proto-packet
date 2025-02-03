use std::io::Error;
use std::io::ErrorKind::InvalidData;

use crate::io::WireType;

/// Creates the invalid wire type error.
pub(in crate::io::decode) fn error_invalid_wire_type(
    semantic_type: &str,
    wire_type: WireType,
) -> Error {
    Error::new(
        InvalidData,
        format!("invalid wire type ({:?}) for {}", wire_type, semantic_type),
    )
}
