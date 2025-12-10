/// An enum with a single case.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum SingleCase {
    Unrecognized(SingleCaseUnrecognized),

    /// // The first case.
    /// One = 1;
    One,
}

impl From<proto_packet::io::TagNumber> for SingleCase {
    fn from(tag: proto_packet::io::TagNumber) -> Self {
        match tag.value() {
            1 => Self::One,
            _ => Self::Unrecognized(SingleCaseUnrecognized { tag }),
        }
    }
}
impl proto_packet::Packet for SingleCase {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::VarInt
    }
}

impl proto_packet::Enum for SingleCase {}

impl proto_packet::io::WithTagNumber for SingleCase {
    fn tag(&self) -> proto_packet::io::TagNumber {
        let tag: u32 = match self {
            Self::Unrecognized(u) => u.tag.value(),
            Self::One => 1,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag) }
    }
}

proto_packet::impl_encode_enum!(SingleCase);

proto_packet::impl_decode_enum!(SingleCase);

/// An unrecognized `SingleCase` case.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct SingleCaseUnrecognized {
    tag: proto_packet::io::TagNumber,
}
