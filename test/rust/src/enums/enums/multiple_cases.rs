/// An enum with multiple cases.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum MultipleCases {
    Unrecognized(MultipleCasesUnrecognized),

    /// // The first case.
    /// One = 1;
    One,

    /// // The second case.
    /// Two = 2;
    Two,

    /// // The third case.
    /// Three = 3;
    Three,
}

impl From<proto_packet::io::TagNumber> for MultipleCases {
    fn from(tag: proto_packet::io::TagNumber) -> Self {
        match tag.value() {
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            _ => Self::Unrecognized(MultipleCasesUnrecognized { tag }),
        }
    }
}
impl proto_packet::Packet for MultipleCases {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::VarInt
    }
}

impl proto_packet::Enum for MultipleCases {}

impl proto_packet::io::WithTagNumber for MultipleCases {
    fn tag(&self) -> proto_packet::io::TagNumber {
        let tag: u32 = match self {
            Self::Unrecognized(u) => u.tag.value(),
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag) }
    }
}

proto_packet::impl_encode_enum!(MultipleCases);

proto_packet::impl_decode_enum!(MultipleCases);

/// An unrecognized `MultipleCases` case.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct MultipleCasesUnrecognized {
    tag: proto_packet::io::TagNumber,
}
