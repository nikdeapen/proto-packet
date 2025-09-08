use crate::{ServiceCallName, ServiceCallNameRef, TypeTag, WithComments, WithServiceCallName};

/// A service call.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ServiceCall {
    comments: Vec<String>,
    service_call_name: ServiceCallName,
    input_type: TypeTag,
    output_type: TypeTag,
}

impl ServiceCall {
    //! Construction

    /// Creates a new service call.
    pub fn new<N, T0, T1>(service_call_name: N, input_type: T0, output_type: T1) -> Self
    where
        N: Into<ServiceCallName>,
        T0: Into<TypeTag>,
        T1: Into<TypeTag>,
    {
        let service_call_name: ServiceCallName = service_call_name.into();
        let input_type: TypeTag = input_type.into();
        let output_type: TypeTag = output_type.into();
        Self {
            comments: Vec::default(),
            service_call_name,
            input_type,
            output_type,
        }
    }
}

impl WithComments for ServiceCall {
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

impl WithServiceCallName for ServiceCall {
    fn service_call_name(&self) -> ServiceCallNameRef<'_> {
        self.service_call_name.to_ref()
    }
}

impl ServiceCall {
    //! Type Tags

    /// Gets the input type.
    pub fn input_type(&self) -> &TypeTag {
        &self.input_type
    }

    /// Gets the output type.
    pub fn output_type(&self) -> &TypeTag {
        &self.output_type
    }
}
