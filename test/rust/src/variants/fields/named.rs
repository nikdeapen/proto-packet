/// A variant with named types.
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub enum Named {
    Unrecognized(NamedUnrecognized),

    One(crate::variants::fields::Primitives),
}

impl proto_packet::Packet for Named {
    fn wire_type() -> proto_packet::io::WireType {
        proto_packet::io::WireType::LengthPrefixed
    }
}

impl proto_packet::Variant for Named {}

impl proto_packet::io::WithTagNumber for Named {
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
            Self::One(_) => 1,
        };
        unsafe { proto_packet::io::TagNumber::new_unchecked(tag) }
    }
}

impl enc::EncodedLen for Named {
    fn encoded_len(&self) -> Result<usize, enc::Error> {
        use proto_packet::Packet;

        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                encoded_len += u.serial.len();
            }
            Self::One(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    crate::variants::fields::Primitives::wire_type(),
                    tag,
                );
                encoded_len += header.encoded_len()?;
                let encoder: proto_packet::io::Encoder<crate::variants::fields::Primitives> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encoded_len()?;
            }
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToSlice for Named {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, enc::Error> {
        use proto_packet::Packet;

        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                (&mut target[..u.serial.len()]).copy_from_slice(u.serial.as_slice());
                encoded_len += u.serial.len();
            }
            Self::One(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    crate::variants::fields::Primitives::wire_type(),
                    tag,
                );
                encoded_len += header.encode_to_slice_unchecked(&mut target[encoded_len..])?;
                let encoder: proto_packet::io::Encoder<crate::variants::fields::Primitives> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_slice_unchecked(&mut target[encoded_len..])?;
            }
        }

        Ok(encoded_len)
    }
}

impl enc::EncodeToWrite for Named {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, enc::Error>
    where
        W: std::io::Write,
    {
        use proto_packet::Packet;

        let mut encoded_len: usize = 0;

        match self {
            Self::Unrecognized(u) => {
                w.write_all(u.serial.as_slice())?;
                encoded_len += u.serial.len();
            }
            Self::One(value) => {
                let tag: proto_packet::io::TagNumber =
                    unsafe { proto_packet::io::TagNumber::new_unchecked(1) };
                let header: proto_packet::io::FieldHeader = proto_packet::io::FieldHeader::new(
                    crate::variants::fields::Primitives::wire_type(),
                    tag,
                );
                encoded_len += header.encode_to_write(w)?;
                let encoder: proto_packet::io::Encoder<crate::variants::fields::Primitives> =
                    proto_packet::io::Encoder::new(value, false);
                encoded_len += encoder.encode_to_write(w)?;
            }
        }

        Ok(encoded_len)
    }
}

impl enc::DecodeFromRead for Named {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, enc::Error>
    where
        R: std::io::Read,
    {
        use proto_packet::Packet;
        use enc::DecodeFromReadPrefix;
        use proto_packet::io::Decoder;

        let header: proto_packet::io::FieldHeader =
            proto_packet::io::FieldHeader::decode_from_read_prefix(r)?;
        match header.tag().value() {
            1 => {
                let value: crate::variants::fields::Primitives = {
                    let decoder: Decoder = Decoder::default();
                    let first: u8 = enc::read_single_byte(r)?;
                    decoder.decode_packet(
                        crate::variants::fields::Primitives::wire_type(),
                        r,
                        first,
                    )?
                };
                Ok(Self::One(value))
            }
            _ => {
                use enc::EncodeToWrite;
                use std::io::Cursor;

                let serial: Vec<u8> = Vec::default();
                let mut serial: Cursor<Vec<u8>> = Cursor::new(serial);
                header.encode_to_write(&mut serial)?;
                header.wire().transfer(r, &mut serial)?;
                let serial: Vec<u8> = serial.into_inner();
                Ok(Self::Unrecognized(NamedUnrecognized { serial }))
            }
        }
    }
}

enc::impl_decode_from_read_prefix_length_prefixed!(Named);

/// An unrecognized `Named` case.
#[derive(
    Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct NamedUnrecognized {
    serial: Vec<u8>,
}
