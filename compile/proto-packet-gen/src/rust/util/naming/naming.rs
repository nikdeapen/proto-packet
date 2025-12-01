/// Responsible for naming things.
#[derive(Clone, Debug)]
pub struct Naming {
    pub(in crate::rust) dot_schema: String,
    pub(in crate::rust) dot_target: String,
}

impl Default for Naming {
    fn default() -> Self {
        Self {
            dot_schema: ".pps".to_owned(),
            dot_target: ".rs".to_owned(),
        }
    }
}
