/// A variant with special types.
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum Specials {
    Unrecognized(SpecialsUnrecognized),

    One(uuid::Uuid),

    Two(String),
}

impl proto_packet::Packet for Specials {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Variant for Specials {}

impl proto_packet::io::WithTagNumber for Specials {
    fn tag_number(&self) -> proto_packet::io::TagNumber {
        let tag_number: u32 = match self {
            Self::Unrecognized(u) => {
                use enc::DecodeFromReadPrefix;
                use std::io::Cursor;
                let mut serial: Cursor<&[u8]> = Cursor::new(u.serial.as_slice());
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::decode_from_read_prefix(&mut serial)
                        .expect("'serial' must start with a valid field header");
                header.tag().value()
            }
            Self::One(_) => 1,
            Self::Two(_) => 2,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag_number) }
    }
}

impl enc::EncodedLen for Specials {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                encoded_len += u.serial.len();
            }
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    proto_packet::io::WireType::Fixed16Byte,
                    tag_number,
                );
                encoded_len += header.encoded_len()?;
                let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encoded_len()?;
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    proto_packet::io::WireType::LengthPrefixed,
                    tag_number,
                );
                encoded_len += header.encoded_len()?;
                let encoder: proto_packet::io::Encoder<String> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encoded_len()?;
            }
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Specials {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                (&mut target[..u.serial.len()]).copy_from_slice(u.serial.as_slice());
                encoded_len += u.serial.len();
            }
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    proto_packet::io::WireType::Fixed16Byte,
                    tag_number,
                );
                encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
                let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    proto_packet::io::WireType::LengthPrefixed,
                    tag_number,
                );
                encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
                let encoder: proto_packet::io::Encoder<String> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            }
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Specials {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                w.write_all(u.serial.as_slice())?;
                encoded_len += u.serial.len();
            }
            Self::One(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    proto_packet::io::WireType::Fixed16Byte,
                    tag_number,
                );
                encoded_len += header.encode_to_write(w)?;
                let encoder: proto_packet::io::Encoder<uuid::Uuid> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_write(w)?;
            }
            Self::Two(value) => {
                let tag_number: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(2) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    proto_packet::io::WireType::LengthPrefixed,
                    tag_number,
                );
                encoded_len += header.encode_to_write(w)?;
                let encoder: proto_packet::io::Encoder<String> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_write(w)?;
            }
        }

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Specials {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
    where
        R: std::io::Read,
    {
        use enc::DecodeFromReadPrefix;
        use proto_packet::io::Decoder;

        let header: proto_packet::io::FieldHeader =
            proto_packet::io::FieldHeader::decode_from_read_prefix(r)?;
        match header.tag().value() {
            1 => {
                let value: uuid::Uuid = {
                    let decoder: Decoder = Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_uuid(proto_packet::io::WireType::Fixed16Byte, r, first)?
                };
                Ok(Self::One(value))
            }
            2 => {
                let value: String = {
                    let decoder: Decoder = Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_string(proto_packet::io::WireType::LengthPrefixed, r, first)?
                };
                Ok(Self::Two(value))
            }
            _ => {
                use enc::EncodeToWrite;
                use std::io::Cursor;

                let serial: Vec<u8> = Vec::default();
                let mut serial: Cursor<Vec<u8>> = Cursor::new(serial);
                header.encode_to_write(&mut serial)?;
                header.wire().transfer(r, &mut serial)?;
                let serial: Vec<u8> = serial.into_inner();
                Ok(Self::Unrecognized(SpecialsUnrecognized { serial }))
            }
        }
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Specials);

/// An unrecognized `Specials` case.
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct SpecialsUnrecognized {
    serial: Vec<u8>,
}
