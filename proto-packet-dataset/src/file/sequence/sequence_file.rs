use file_storage::FilePath;

/// A sequence file.
///
/// # Layout
/// A sequence file is simply one packet after another with no metadata.
///
/// If the packets are `WireType::LengthPrefixed` each packet is length-prefixed.
pub struct SequenceFile {
    pub(in crate::file::sequence) file: FilePath,
}

impl<P: Into<FilePath>> From<P> for SequenceFile {
    fn from(file: P) -> Self {
        Self { file: file.into() }
    }
}
