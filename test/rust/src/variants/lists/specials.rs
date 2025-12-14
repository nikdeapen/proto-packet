/// A variant with special type slices.
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum Specials {
    Unrecognized(SpecialsUnrecognized),

    Pne(Vec<uuid::Uuid>),

    Two(Vec<String>),
}

impl proto_packet::Packet for Specials {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Variant for Specials {}

impl proto_packet::io::WithTagNumber for Specials {
    fn tag(&self) -> proto_packet::io::TagNumber {
        let tag: u32 = match self {
            Self::Unrecognized(u) => {
                use enc::DecodeFromReadPrefix;
                use std::io::Cursor;
                let mut serial: Cursor<&[u8]> = Cursor::new(u.serial.as_slice());
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::decode_from_read_prefix(&mut serial)
                        .expect("'serial' must start with a valid field header");
                header.tag().value()
            }
            Self::Pne(_) => 6,
            Self::Two(_) => 7,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag) }
    }
}

impl enc::EncodedLen for Specials {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                encoded_len += u.serial.len();
            }
            Self::Pne(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(6) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
                encoded_len += header.encoded_len()?;
                let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encoded_len()?;
            }
            Self::Two(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(7) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
                encoded_len += header.encoded_len()?;
                let encoder: proto_packet::io::Encoder<Vec<String>> =
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
            Self::Pne(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(6) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
                encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
                let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            }
            Self::Two(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(7) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
                encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
                let encoder: proto_packet::io::Encoder<Vec<String>> =
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
            Self::Pne(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(6) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
                encoded_len += header.encode_to_write(w)?;
                let encoder: proto_packet::io::Encoder<Vec<uuid::Uuid>> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_write(w)?;
            }
            Self::Two(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(7) };
                let header: proto_packet::io::FieldHeader =
                    proto_packet::io::FieldHeader::new(proto_packet::io::WireType::List, tag);
                encoded_len += header.encode_to_write(w)?;
                let encoder: proto_packet::io::Encoder<Vec<String>> =
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
            6 => {
                let value: Vec<uuid::Uuid> = {
                    let decoder: Decoder = Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_uuid_list(proto_packet::io::WireType::List, r, first)?
                };
                Ok(Self::Pne(value))
            }
            7 => {
                let value: Vec<String> = {
                    let decoder: Decoder = Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_string_list(proto_packet::io::WireType::List, r, first)?
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
