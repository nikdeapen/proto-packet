/// An element with comment lines.
pub trait WithComments: Sized {
    /// Gets the comment lines.
    fn comments(&self) -> &[String];

    /// Adds the `comment` line.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;

    /// Adds the `comment` line.
    fn with_comment<S>(mut self, comment: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment);
        self
    }
}
