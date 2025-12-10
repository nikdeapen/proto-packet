/// An empty enum.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum EmptyEnum {
    Unrecognized(EmptyEnumUnrecognized),
}

impl From<proto_packet::io::TagNumber> for EmptyEnum {
    fn from(tag: proto_packet::io::TagNumber) -> Self {
        match tag.value() {
            _ => Self::Unrecognized(EmptyEnumUnrecognized { tag }),
        }
    }
}
impl proto_packet::Packet for EmptyEnum {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::VarInt
    }
}

impl proto_packet::Enum for EmptyEnum {}

impl proto_packet::io::WithTagNumber for EmptyEnum {
    fn tag(&self) -> proto_packet::io::TagNumber {
        let tag: u32 = match self {
            Self::Unrecognized(u) => u.tag.value(),
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag) }
    }
}

proto_packet::impl_encode_enum!(EmptyEnum);

proto_packet::impl_decode_enum!(EmptyEnum);

/// An unrecognized `EmptyEnum` case.
#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct EmptyEnumUnrecognized {
    tag: proto_packet::io::TagNumber,
}
