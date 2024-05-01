/// An element with a name.
pub trait WithName {
    /// Gets the name.
    fn name(&self) -> &str;
}
