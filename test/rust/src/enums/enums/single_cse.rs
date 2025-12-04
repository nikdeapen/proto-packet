/// An enum with a single case.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum SingleCse {
    Unrecognized(SingleCseUnrecognized),

    /// // The first case.
    /// One = 1;
    One,
}

impl From<proto_packet::io::TagNumber> for SingleCse {
    fn from(tag: proto_packet::io::TagNumber) -> Self {
        match tag.value() {
            1 => Self::One,
            _ => Self::Unrecognized(SingleCseUnrecognized { tag }),
        }
    }
}
impl proto_packet::Packet for SingleCse {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::VarInt
    }
}

impl proto_packet::Enum for SingleCse {}

impl proto_packet::io::WithTagNumber for SingleCse {
    fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::Unrecognized(u) => u.tag.value(),
            Self::One => 1,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

proto_packet::impl_encode_enum!(SingleCse);

proto_packet::impl_decode_enum!(SingleCse);

/// An unrecognized `SingleCse` case.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct SingleCseUnrecognized {
    tag: proto_packet::io::TagNumber,
}
