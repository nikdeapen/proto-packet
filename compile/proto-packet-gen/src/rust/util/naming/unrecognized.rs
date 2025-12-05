use crate::rust::Naming;
use proto_packet_tree::WithTypeName;

impl Naming {
    //! Unrecognized

    /// Gets the type name for the unrecognized value of the element `e`.
    pub fn unrecognized_case_type_name<T>(&self, e: &T) -> String
    where
        T: WithTypeName,
    {
        format!("{}{}", self.type_name(e), Self::UNRECOGNIZED_CASE_NAME)
    }
}
