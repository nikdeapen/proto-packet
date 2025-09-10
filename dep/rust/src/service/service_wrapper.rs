use crate::service::service_write::ServiceWrite;
use crate::service::ServiceError;
use actix_web::web::Bytes;

pub struct ServiceWrapper<S, F>
where
    F: Fn(&S, &str, Bytes, &mut ServiceWrite) -> Result<usize, ServiceError>,
{
    service: S,
    service_fn: F,
}

impl<S, F> ServiceWrapper<S, F>
where
    F: Fn(&S, &str, Bytes, &mut ServiceWrite) -> Result<usize, ServiceError>,
{
    //! Construction

    /// Creates a new service wrapper.
    pub const fn new(service: S, service_fn: F) -> Self {
        Self {
            service,
            service_fn,
        }
    }
}

impl<S, F> ServiceWrapper<S, F>
where
    F: Fn(&S, &str, Bytes, &mut ServiceWrite) -> Result<usize, ServiceError>,
{
    //! Service

    /// Services the request.
    pub fn service(
        &self,
        service_call_name: &str,
        input: Bytes,
    ) -> Result<ServiceWrite, ServiceError> {
        let mut write: ServiceWrite = ServiceWrite::default();
        (self.service_fn)(&self.service, service_call_name, input, &mut write).map(|_| write)
    }
}
