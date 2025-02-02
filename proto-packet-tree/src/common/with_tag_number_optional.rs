use proto_packet::io::TagNumber;

/// An element with an optional tag number.
pub trait WithTagNumberOptional: Sized {
    /// Gets the optional tag number.
    fn tag_number(&self) -> Option<TagNumber>;

    /// Sets the optional tag number.
    fn set_tag_number<N>(&mut self, tag_number: N)
    where
        N: Into<Option<TagNumber>>;

    /// Sets the optional tag number.
    fn with_tag_number<N>(mut self, tag_number: N) -> Self
    where
        N: Into<Option<TagNumber>>,
    {
        self.set_tag_number(tag_number);
        self
    }
}
