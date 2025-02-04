use proto_packet::io::TagNumber;

/// An element with a tag number.
pub trait WithTagNumber: Sized {
    /// Gets the tag number.
    fn tag_number(&self) -> TagNumber;

    /// Sets the tag number.
    fn set_tag_number<N>(&mut self, tag_number: N)
    where
        N: Into<TagNumber>;

    /// Sets the optional tag number.
    fn with_tag_number<N>(mut self, tag_number: N) -> Self
    where
        N: Into<TagNumber>,
    {
        self.set_tag_number(tag_number);
        self
    }
}
