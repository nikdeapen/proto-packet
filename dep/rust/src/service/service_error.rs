use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use enc::Error;

/// A service error.
pub struct ServiceError {
    pub status: StatusCode,
    pub message: String,
}

impl ServiceError {
    //! Construction

    pub fn from_write_error(_error: enc::Error) -> Self {
        todo!()
    }

    pub fn from_read_error(_error: Error) -> Self {
        todo!()
    }
}

impl ServiceError {
    //! Responder

    /// Creates the responder.
    pub fn responder(self) -> HttpResponse {
        HttpResponse::build(self.status).body(self.message)
    }
}
