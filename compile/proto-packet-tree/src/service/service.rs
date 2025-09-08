use crate::{ServiceCall, TypeName, TypeNameRef, WithComments, WithServiceCallName, WithTypeName};

/// A service.
///
/// # Invariants
/// 1. No two service calls can have the same name.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Service {
    comments: Vec<String>,
    service_name: TypeName,
    service_calls: Vec<ServiceCall>,
}

impl<N: Into<TypeName>> From<N> for Service {
    fn from(service_name: N) -> Self {
        let service_name: TypeName = service_name.into();
        Self {
            comments: Vec::default(),
            service_name,
            service_calls: Vec::default(),
        }
    }
}

impl WithComments for Service {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithTypeName for Service {
    fn type_name(&self) -> TypeNameRef<'_> {
        self.service_name.to_ref()
    }
}

impl Service {
    //! Calls

    /// Gets the service_calls.
    pub fn service_calls(&self) -> &[ServiceCall] {
        self.service_calls.as_slice()
    }

    /// Gets the optional service_call with the given `service_call_name`.
    pub fn service_call_with_name<S>(&self, service_call_name: S) -> Option<&ServiceCall>
    where
        S: AsRef<str>,
    {
        self.service_calls
            .iter()
            .filter(|f| f.service_call_name() == service_call_name)
            .next()
    }

    /// Checks if the `service_call` can be added.
    ///
    /// Returns `true` if:
    ///     1. The service_call name is not already present.
    pub fn can_add_service_call(&self, service_call: &ServiceCall) -> bool {
        self.service_call_with_name(service_call.service_call_name())
            .is_none()
    }

    /// Adds the `service_call`.
    ///
    /// # Unsafe
    /// The `service_call` must be able to be added.
    pub unsafe fn add_service_call<F>(&mut self, service_call: F)
    where
        F: Into<ServiceCall>,
    {
        let service_call: ServiceCall = service_call.into();

        debug_assert!(self.can_add_service_call(&service_call));

        self.service_calls.push(service_call.into());
    }

    /// Adds the `service_call`.
    ///
    /// # Unsafe
    /// The `service_call` must be able to be added.
    pub unsafe fn with_service_call<F>(mut self, service_call: F) -> Self
    where
        F: Into<ServiceCall>,
    {
        self.add_service_call(service_call);
        self
    }
}
