/// // A struct with primitive types.
/// struct Primitives {
///    
///    // A 'u8' field.
///    one: u8;
///    
///    // A 'u16' field.
///    two: u16;
///    
///    // A 'u32' field.
///    three: u32;
///    
///    // A 'u64' field.
///    four: u64;
///    
///    // A 'u128' field.
///    five: u128;
/// }
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct Primitives {
    one: u8,
    two: u16,
    three: u32,
    four: u64,
    five: u128,
}
