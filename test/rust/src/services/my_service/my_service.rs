use proto_packet::service::ServiceError;

/// My service.
pub trait MyService {
    //// The first service call.
    fn one(
        &self,
        input: crate::services::my_service::MyRequest,
    ) -> Result<crate::services::my_service::MyResponse, ServiceError>;
}

fn service_packet<S, W>(
    service: &S,
    service_call_name: &str,
    input: actix_web::web::Bytes,
    output: &mut W,
) -> Result<usize, ServiceError>
where
    S: MyService,
    W: std::io::Write,
{
    match service_call_name {
        "one" => proto_packet::service::service_packet(input, output, |input| service.one(input)),
        _ => {
            todo!()
        }
    }
}

pub fn service<S>(service: S) -> actix_web::Scope
where
    S: 'static + MyService + Sync + Send,
{
    proto_packet::service::service(
        service,
        "crate::services::my_service::MyService",
        |service, call, input, output| service_packet(service, call, input, output),
    )
}
