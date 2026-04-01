use crate::service::ServiceError;
use actix_web::web;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Mutex;

use super::handle_request;

/// A builder for creating an actix-web scope from a service.
pub struct ServiceScope<S> {
    scope: actix_web::Scope,
    service: web::Data<Mutex<S>>,
}

impl<S: 'static> ServiceScope<S> {
    //! Construction

    /// Creates a new service scope with the given `path` and `service`.
    pub fn new(path: &str, service: S) -> Self {
        Self {
            scope: web::scope(path),
            service: web::Data::new(Mutex::new(service)),
        }
    }
}

impl<S: 'static> ServiceScope<S> {
    //! Calls

    /// Registers a service call at the given `path`.
    pub fn call<I, O, F>(mut self, path: &str, f: F) -> Self
    where
        I: DeserializeOwned + 'static,
        O: Serialize + 'static,
        F: Fn(&mut S, I) -> Result<O, ServiceError> + 'static + Clone,
    {
        let service: web::Data<Mutex<S>> = self.service.clone();
        let handler = move |body: web::Bytes| {
            let service: web::Data<Mutex<S>> = service.clone();
            let f: F = f.clone();
            async move {
                let mut service = service.lock().unwrap();
                handle_request::<I, O, _>(&body, |req| f(&mut *service, req))
            }
        };
        self.scope = self.scope.route(path, web::post().to(handler));
        self
    }
}

impl<S: 'static> ServiceScope<S> {
    //! Build

    /// Builds the actix-web scope.
    pub fn build(self) -> actix_web::Scope {
        self.scope.app_data(self.service)
    }
}
