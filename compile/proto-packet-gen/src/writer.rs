use clerr::Report;
use code_gen::Source;

/// Responsible for writing generated code.
pub trait Writer {
    /// Writes the `source` code to the `file_name`.
    fn write(&self, source: &Source, file_name: &str) -> Result<(), Report>;
}
