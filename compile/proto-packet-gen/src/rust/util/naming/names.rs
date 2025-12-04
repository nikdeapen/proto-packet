use crate::rust::{Error, Naming};
use proto_packet_tree::{ModName, TypeNameRef, WithCaseName, WithFieldName, WithTypeName};

impl Naming {
    //! Field Names

    /// Gets the rust field name for the `element`.
    pub fn field_name<N>(&self, element: &N) -> String
    where
        N: WithFieldName,
    {
        element.field_name().to_string()
    }
}

impl Naming {
    //! Case Names

    /// Gets the rust case name for the `element`.
    pub fn case_name<N>(&self, element: &N) -> String
    where
        N: WithCaseName,
    {
        element.case_name().to_string()
    }
}

impl Naming {
    //! Type Names

    /// Gets the rust type name for the `element`.
    pub fn type_name<E>(&self, element: &E) -> String
    where
        E: WithTypeName,
    {
        element.type_name().to_string()
    }
}

impl Naming {
    //! Mod Names

    /// Gets the mod name for the `element`.
    pub fn mod_name_for_type_name<N>(&self, element: N) -> Result<ModName, Error>
    where
        N: WithTypeName,
    {
        let type_name: TypeNameRef = element.type_name();
        let snake_case: String = self.pascal_to_snake_case(type_name.as_ref());
        match ModName::new(snake_case) {
            Ok(mod_name) => Ok(mod_name),
            Err(error) => Err(Error::TypeNameToModName {
                type_name: type_name.to_owned(),
                error: error.to_string(),
            }),
        }
    }
}
