use crate::service::{ServiceError, ServiceWrapper, ServiceWrite};
use actix_web::{web, HttpResponse, Scope};

/// Creates an actix-web service.
pub fn service<S, F>(service: S, service_name: &str, service_fn: F) -> Scope
where
    S: 'static + Sync + Send,
    F: 'static
        + Fn(&S, &str, web::Bytes, &mut ServiceWrite) -> Result<usize, ServiceError>
        + Sync
        + Send,
{
    web::scope(format!("/{}", service_name).as_str())
        .app_data(web::Data::new(ServiceWrapper::new(service, service_fn)))
        .route(
            "/{service_call_name}",
            web::post().to(service_handler::<S, F>),
        )
}

async fn service_handler<S, F>(
    service: web::Data<ServiceWrapper<S, F>>,
    service_call_name: web::Path<String>,
    input: web::Bytes,
) -> HttpResponse
where
    S: 'static + Sync + Send,
    F: 'static
        + Fn(&S, &str, web::Bytes, &mut ServiceWrite) -> Result<usize, ServiceError>
        + Sync
        + Send,
{
    match web::block(move || service.service(service_call_name.as_str(), input)).await {
        Ok(result) => match result {
            Ok(write) => HttpResponse::Ok().body(write.body()),
            Err(error) => error.responder(),
        },
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}
