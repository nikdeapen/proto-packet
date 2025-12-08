use proto_packet::io::TagNumber;

/// An element with a tag number.
pub trait WithTagNumber: Sized {
    /// Gets the tag number.
    fn tag(&self) -> TagNumber;

    /// Sets the `tag`.
    fn set_tag(&mut self, tag: TagNumber);

    /// Sets the `tag`.
    fn with_tag(mut self, tag: TagNumber) -> Self {
        self.set_tag(tag);
        self
    }
}
