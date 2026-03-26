use crate::service::{ServiceDispatchError, ServiceError};
use actix_web::HttpResponse;
use enc::{DecodeFromRead, EncodeToWrite};

/// Handles an actix-web request by decoding, calling the service, and encoding the response.
pub fn handle_request<I, O, F>(body: &[u8], service_call: F) -> HttpResponse
where
    I: DecodeFromRead,
    O: EncodeToWrite,
    F: FnOnce(I) -> Result<O, ServiceError>,
{
    let mut response_bytes: Vec<u8> = Vec::new();
    match crate::service::handle_call(&mut &*body, &mut response_bytes, service_call) {
        Ok(()) => HttpResponse::Ok()
            .content_type("application/octet-stream")
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
