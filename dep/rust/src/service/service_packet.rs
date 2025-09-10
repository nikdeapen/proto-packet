use crate::service::ServiceError;
use actix_web::web::Bytes;
use enc::{DecodeFromRead, EncodeToWrite};
use std::io::Cursor;

/// Services a single packet.
pub fn service_packet<I, O, W, F>(
    input: actix_web::web::Bytes,
    output: &mut W,
    service_fn: F,
) -> Result<usize, ServiceError>
where
    I: DecodeFromRead,
    O: EncodeToWrite,
    W: std::io::Write,
    F: Fn(I) -> Result<O, ServiceError>,
{
    let mut input: Cursor<Bytes> = Cursor::new(input);
    match I::decode_from_read(&mut input) {
        Ok(input) => service_fn(input)?
            .encode_to_write(output)
            .map_err(|e| ServiceError::from_write_error(e)),
        Err(e) => Err(ServiceError::from_read_error(e)),
    }
}
