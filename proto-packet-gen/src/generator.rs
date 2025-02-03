use clerr::Report;
use file_storage::FolderPath;

use proto_packet_tree::Project;

use crate::Generated;

/// Responsible for generating code.
pub trait Generator {
    /// Generates the code for the project.
    fn generate(&self, project: &Project, target: &FolderPath) -> Result<Generated, Report>;
}
