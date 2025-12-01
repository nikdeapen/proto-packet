/// Responsible for encoding values.
#[derive(Copy, Clone, Debug)]
pub struct Encoder<'a, T> {
    pub(in crate::io::encode) value: &'a T,
    pub(in crate::io::encode) fixed: bool,
}

impl<'a, T> Encoder<'a, T> {
    //! Construction

    /// Creates a new encoder.
    pub const fn new(value: &'a T, fixed: bool) -> Self {
        Self { value, fixed }
    }
}
