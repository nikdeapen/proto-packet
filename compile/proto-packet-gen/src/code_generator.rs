use crate::GeneratedCode;
use clerr::Report;
use file_storage::FolderPath;
use proto_packet_tree::Project;

/// Responsible for generating code.
pub trait CodeGenerator {
    /// Generates the code for the `project`.
    fn generate(&self, project: &Project, target: &FolderPath) -> Result<GeneratedCode, Report>;
}
