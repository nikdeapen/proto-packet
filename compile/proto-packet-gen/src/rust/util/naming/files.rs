use crate::rust::{Error, Naming};
use proto_packet_tree::ModPathRef;

impl Naming {
    //! File Properties

    /// Gets the `.{schema}` extension.
    pub fn dot_schema(&self) -> &str {
        self.dot_schema.as_ref()
    }

    /// Gets the `.{target}` extension.
    pub fn dot_target(&self) -> &str {
        self.dot_target.as_ref()
    }
}

impl Naming {
    //! File for ModPath

    /// Gets the file name for the `mod_path`.
    ///
    /// Assumes a `/` file separator.
    pub fn file_name_for_mod_path(&self, mod_path: ModPathRef) -> Result<String, Error> {
        let capacity: usize = mod_path.value().len() + self.dot_target.len();
        let mut s: String = String::with_capacity(capacity);
        mod_path
            .as_ref()
            .split('.')
            .enumerate()
            .for_each(|(i, part)| {
                if i != 0 {
                    s.push('/');
                }
                s.push_str(part);
            });
        s.push_str(self.dot_target.as_ref());
        Ok(s)
    }
}
