use crate::rust::util::Typing;
use proto_packet_tree::QualifiedNameRef;

impl Typing {
    //! Rust Name

    /// Maps the `qualified_name` to a qualified name in rust.
    pub fn rust_name(&self, qualified_name: QualifiedNameRef) -> String {
        let mut s: String = String::with_capacity(16 + qualified_name.len());
        s.push_str("crate::");
        if let Some(mod_path) = qualified_name.mod_path() {
            for mod_name in mod_path.mod_names() {
                s.push_str(mod_name.as_ref());
                s.push_str("::");
            }
        }
        s.push_str(qualified_name.type_name().as_ref());
        s
    }
}
