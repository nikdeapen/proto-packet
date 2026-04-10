use crate::service::ServiceError;
use actix_web::web;
use enc::{DecodeFromRead, EncodeToSlice};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
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
        I: DeserializeOwned + DecodeFromRead + 'static,
        O: Serialize + EncodeToSlice + 'static,
        F: Fn(&mut S, I) -> Result<O, ServiceError> + 'static + Clone,
    {
        let service: web::Data<Mutex<S>> = self.service.clone();
        let handler = move |body: web::Bytes, query: web::Query<HashMap<String, String>>| {
            let service: web::Data<Mutex<S>> = service.clone();
            let f: F = f.clone();
            let fmt: String = query.get("fmt").cloned().unwrap_or_default();
            async move {
                let mut service = service.lock().unwrap_or_else(|e| e.into_inner());
                handle_request::<I, O, _>(&body, &fmt, |req| f(&mut *service, req))
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
