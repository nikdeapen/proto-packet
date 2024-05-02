use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

use crate::compiler_error::CompilerError;
use crate::CompilerError::{DirectoryReadError, PathNameUTF8Error};

/// Responsible for resolving schema files.
#[derive(Clone, Debug, Default)]
pub struct SchemaFileResolver {
    _nothing: (),
}

impl SchemaFileResolver {
    //! Constants

    /// The schema file extension with a dot prefix.
    pub const DOT_SCHEMA_EXTENSION: &'static str = ".pps";
}

impl SchemaFileResolver {
    //! Resolve Schema Files

    /// Recursively resolves the schema files in the directory.
    pub fn resolve_schema_files(&self, directory: &str) -> Result<Vec<String>, CompilerError> {
        let mut result: Vec<String> = Vec::default();
        self.resolve_schema_files_to_vec(directory, &mut result)?;
        Ok(result)
    }

    /// Recursively resolves the schema files in the directory and adds them to the `result` vec.
    pub fn resolve_schema_files_to_vec(
        &self,
        directory: &str,
        result: &mut Vec<String>,
    ) -> Result<(), CompilerError> {
        let read_dir: ReadDir =
            fs::read_dir(directory).map_err(|e| DirectoryReadError(directory.to_string(), e))?;
        for result_dir_entry in read_dir.into_iter() {
            match result_dir_entry {
                Err(e) => return Err(DirectoryReadError(directory.to_string(), e)),
                Ok(dir_entry) => {
                    let path: PathBuf = dir_entry.path();
                    if let Some(path_str) = path.to_str() {
                        if path.is_file() {
                            if path_str.ends_with(Self::DOT_SCHEMA_EXTENSION) {
                                result.push(path_str.to_string());
                            }
                        } else if path.is_dir() {
                            self.resolve_schema_files_to_vec(path_str, result)?;
                        }
                    } else {
                        return Err(PathNameUTF8Error(path.into_os_string()));
                    }
                }
            }
        }
        Ok(())
    }
}
