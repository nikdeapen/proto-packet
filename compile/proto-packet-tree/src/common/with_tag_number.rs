use proto_packet::io::TagNumber;

/// An element with a tag number.
pub trait WithTagNumber: Sized {
    /// Gets the tag number.
    fn tag_number(&self) -> TagNumber;

    /// Sets the `tag_number`.
    fn set_tag_number(&mut self, tag_number: TagNumber);

    /// Sets the `tag_number`.
    fn with_tag_number(mut self, tag_number: TagNumber) -> Self {
        self.set_tag_number(tag_number);
        self
    }
}
