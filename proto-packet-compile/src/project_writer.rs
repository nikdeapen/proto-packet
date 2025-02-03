use file_storage::FilePath;

use proto_packet_gen::Generated;

use crate::Error;
use crate::Error::Storage;

/// Responsible for writing generated code.
#[derive(Debug, Default)]
pub struct ProjectWriter {
    _nothing: (),
}

impl ProjectWriter {
    //! Write

    /// Writes the generated source code.
    pub fn write(&self, generated: &Generated) -> Result<(), Error> {
        if let Some(error) = generated
            .sources()
            .iter()
            .flat_map(|(file, code)| self.write_file(file, code).err())
            .next()
        {
            Err(error)
        } else {
            Ok(())
        }
    }
}

impl ProjectWriter {
    //! Write File

    /// Writes the source `code` to the `file`.
    pub fn write_file(&self, file: &FilePath, code: &str) -> Result<(), Error> {
        file.write_str(code).map_err(|error| Storage(error))
    }
}
