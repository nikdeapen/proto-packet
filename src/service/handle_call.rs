use crate::service::{ServiceDispatchError, ServiceError};
use enc::{DecodeFromRead, EncodeToSlice};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::io::Cursor;

/// Handles the `service_call` using JSON encoding.
pub fn handle_call_json<I, O, F>(
    request: &[u8],
    service_call: F,
) -> Result<Vec<u8>, ServiceDispatchError>
where
    I: DeserializeOwned,
    O: Serialize,
    F: FnOnce(I) -> Result<O, ServiceError>,
{
    let request: I = serde_json::from_slice(request).map_err(ServiceDispatchError::decode)?;
    let result: O = service_call(request)?;
    let response: Vec<u8> = serde_json::to_vec(&result).map_err(ServiceDispatchError::encode)?;
    Ok(response)
}

/// Handles the `service_call` using binary proto-packet encoding.
pub fn handle_call_binary<I, O, F>(
    request: &[u8],
    service_call: F,
) -> Result<Vec<u8>, ServiceDispatchError>
where
    I: DecodeFromRead,
    O: EncodeToSlice,
    F: FnOnce(I) -> Result<O, ServiceError>,
{
    let mut reader: Cursor<&[u8]> = Cursor::new(request);
    let request: I = I::decode_from_read(&mut reader).map_err(ServiceDispatchError::decode)?;
    let result: O = service_call(request)?;
    let response: Vec<u8> = result.encode_as_vec().map_err(ServiceDispatchError::encode)?;
    Ok(response)
}
