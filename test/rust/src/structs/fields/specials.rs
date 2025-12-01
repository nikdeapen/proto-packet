/// // A struct with special types.
/// struct Specials {
///    
///    // A 'uuid' field.
///    one: uuid;
///    
///    // A 'string' field.
///    two: string;
/// }
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Specials {
    one: uuid::Uuid,
    two: String,
}
