use crate::service::{ServiceDispatchError, ServiceError};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Handles the `service_call`.
pub fn handle_call<I, O, F>(
    request: &[u8],
    service_call: F,
) -> Result<Vec<u8>, ServiceDispatchError>
where
    I: DeserializeOwned,
    O: Serialize,
    F: FnOnce(I) -> Result<O, ServiceError>,
{
    let request: I = serde_json::from_slice(request).map_err(ServiceDispatchError::Decode)?;
    let result: O = service_call(request)?;
    let response: Vec<u8> = serde_json::to_vec(&result).map_err(ServiceDispatchError::Encode)?;
    Ok(response)
}
