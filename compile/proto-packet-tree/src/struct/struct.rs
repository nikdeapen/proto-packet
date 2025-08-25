use crate::{StructField, TypeName, TypeNameRef, WithComments, WithFieldName, WithTypeName};

/// A struct.
///
/// # Invariants
/// 1. No two fields can have the same name.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Struct {
    comments: Vec<String>,
    struct_name: TypeName,
    fields: Vec<StructField>,
}

impl<N: Into<TypeName>> From<N> for Struct {
    fn from(struct_name: N) -> Self {
        let struct_name: TypeName = struct_name.into();
        Self {
            comments: Vec::default(),
            struct_name,
            fields: Vec::default(),
        }
    }
}

impl WithComments for Struct {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithTypeName for Struct {
    fn type_name<'a>(&'a self) -> TypeNameRef<'a> {
        self.struct_name.to_ref()
    }
}

impl Struct {
    //! Fields

    /// Gets the fields.
    pub fn fields(&self) -> &[StructField] {
        self.fields.as_slice()
    }

    /// Gets the optional field with the given `field_name`.
    pub fn field_with_name<S>(&self, field_name: S) -> Option<&StructField>
    where
        S: AsRef<str>,
    {
        self.fields
            .iter()
            .filter(|f| f.field_name() == field_name)
            .next()
    }

    /// Checks if the `field` can be added.
    ///
    /// Returns `true` if:
    ///     1. The field name is not already present.
    ///     2. The tag number is not already present.
    ///     3. If the tag number is `None` all the current tag numbers are also `None`.
    pub fn can_add_field(&self, field: &StructField) -> bool {
        self.field_with_name(field.field_name()).is_none()
    }

    /// Adds the `field`.
    ///
    /// # Unsafe
    /// The `field` must be able to be added.
    pub unsafe fn add_field<F>(&mut self, field: F)
    where
        F: Into<StructField>,
    {
        let field: StructField = field.into();

        debug_assert!(self.can_add_field(&field));

        self.fields.push(field.into());
    }

    /// Adds the `field`.
    ///
    /// # Unsafe
    /// The `field` must be able to be added.
    pub unsafe fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<StructField>,
    {
        self.add_field(field);
        self
    }
}
