use crate::rust::Naming;
use proto_packet_tree::{CaseNameRef, FieldNameRef, ServiceCallNameRef, TypeNameRef, WithTypeName};

impl Naming {
    //! Field Names

    /// Gets the rust field name for the `field_name`.
    pub fn field_name(&self, field_name: FieldNameRef) -> String {
        field_name.to_string()
    }
}

impl Naming {
    //! Case Names

    /// Gets the rust case name for the `case_name`.
    pub fn case_name(&self, case_name: CaseNameRef) -> String {
        case_name.to_string()
    }
}

impl Naming {
    //! Service Call Names

    /// Gets the rust function name name for the `service_call_name`.
    pub fn service_call_name(&self, service_call_name: ServiceCallNameRef) -> String {
        service_call_name.to_string()
    }
}

impl Naming {
    //! Type Names

    /// Gets the rust type name for the `type_name`.
    pub fn type_name<E>(&self, element: &E) -> String
    where
        E: WithTypeName,
    {
        element.type_name().to_string()
    }
}

impl Naming {
    //! Mod Names

    /// Gets the mod name for the declared `type_name`.
    pub fn mod_name_for_type_name(&self, type_name: TypeNameRef) -> String {
        self.pascal_to_snake_case(type_name.as_ref())
    }
}
