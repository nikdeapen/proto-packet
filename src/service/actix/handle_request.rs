use crate::service::{ServiceDispatchError, ServiceError};
use actix_web::HttpResponse;
use enc::{DecodeFromRead, EncodeToSlice};
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Handles an actix-web request by dispatching based on the `fmt` parameter.
///
/// - `"json"` uses JSON encoding.
/// - Default uses binary proto-packet encoding.
pub fn handle_request<I, O, F>(body: &[u8], fmt: &str, service_call: F) -> HttpResponse
where
    I: DeserializeOwned + DecodeFromRead,
    O: Serialize + EncodeToSlice,
    F: FnOnce(I) -> Result<O, ServiceError>,
{
    if fmt == "json" {
        match crate::service::handle_call_json(body, service_call) {
            Ok(response) => HttpResponse::Ok()
                .content_type("application/json")
                .body(response),
            Err(error) => dispatch_error_to_response(error),
        }
    } else {
        match crate::service::handle_call_binary(body, service_call) {
            Ok(response) => HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(response),
            Err(error) => dispatch_error_to_response(error),
        }
    }
}

/// Maps a [ServiceDispatchError] to an [HttpResponse].
fn dispatch_error_to_response(error: ServiceDispatchError) -> HttpResponse {
    match error {
        ServiceDispatchError::Decode(error) => {
            HttpResponse::BadRequest().body(format!("{}", error))
        }
        ServiceDispatchError::Service(error) => {
            HttpResponse::InternalServerError().body(format!("{}", error))
        }
        ServiceDispatchError::Encode(error) => {
            HttpResponse::InternalServerError().body(format!("{}", error))
        }
    }
}
