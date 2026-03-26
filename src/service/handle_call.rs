use crate::service::{ServiceDispatchError, ServiceError};
use enc::{DecodeFromRead, EncodeToWrite};
use std::io::{Read, Write};

/// Handles the `service_call`.
pub fn handle_call<I, O, F, R, W>(
    request: &mut R,
    response: &mut W,
    service_call: F,
) -> Result<(), ServiceDispatchError>
where
    I: DecodeFromRead,
    O: EncodeToWrite,
    F: FnOnce(I) -> Result<O, ServiceError>,
    R: Read,
    W: Write,
{
    let request: I = I::decode_from_read(request).map_err(ServiceDispatchError::Decode)?;
    let result: O = service_call(request)?;
    result
        .encode_to_write(response)
        .map_err(ServiceDispatchError::Encode)?;
    Ok(())
}
