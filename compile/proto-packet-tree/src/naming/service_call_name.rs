use custom_string::custom_string;

use crate::naming::validate_name::validate_name;

custom_string!(
    #[doc = "The name of a service call."],
    ServiceCallName,
    ServiceCallNameRef,
    |s| validate_service_call_name(s)
);

/// Validates the `service_call_name`.
pub fn validate_service_call_name(service_call_name: &str) -> Result<(), &'static str> {
    validate_name(service_call_name)?;

    if !service_call_name.as_bytes()[0].is_ascii_lowercase() {
        Err("service call names must start with a lowercase letter")
    } else {
        Ok(())
    }
}

/// An element with a service_call name.
pub trait WithServiceCallName {
    /// Gets the service_call name.
    fn service_call_name(&self) -> ServiceCallNameRef<'_>;
}
