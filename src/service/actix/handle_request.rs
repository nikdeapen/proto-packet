use crate::service::{ServiceDispatchError, ServiceError};
use actix_web::HttpResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Handles an actix-web request by decoding JSON, calling the service, and encoding the response.
pub fn handle_request<I, O, F>(body: &[u8], service_call: F) -> HttpResponse
where
    I: DeserializeOwned,
    O: Serialize,
    F: FnOnce(I) -> Result<O, ServiceError>,
{
    match crate::service::handle_call(body, service_call) {
        Ok(response_bytes) => HttpResponse::Ok()
            .content_type("application/json")
            .body(response_bytes),
        Err(error) => dispatch_error_to_response(error),
    }
}

/// Maps a `ServiceDispatchError` to an `HttpResponse`.
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
