use std::error::Error;
use std::io::{Cursor, Write};

use enc::hex::HexEncoder;
use enc::var_int::VarIntSize;
use enc::EncodeToWrite;

use proto_packet::io::WireType::LengthPrefixed;
use proto_packet::Packet;

pub fn test_packet<P>(packet: &P, encoded: &[u8]) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    test_encode(packet, encoded)?;
    test_decode(encoded, packet)?;
    Ok(())
}

pub fn display_hex(bytes: &[u8]) -> String {
    let mut s: String = String::default();
    bytes
        .iter()
        .map(|b| HexEncoder::UPPER.encode_chars(*b))
        .enumerate()
        .for_each(|(i, (a, b))| {
            if i != 0 {
                s.push(':')
            }
            s.push(a);
            s.push(b)
        });
    s
}

pub fn test_encode<P>(packet: &P, expected: &[u8]) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    test_encoded_len(packet, expected)?;
    test_encode_to_slice(packet, expected)?;
    test_encode_to_write(packet, expected)?;
    Ok(())
}

pub fn test_encoded_len<P>(packet: &P, expected: &[u8]) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    let result: usize = packet.encoded_len()?;
    assert_eq!(result, expected.len());
    Ok(())
}

pub fn test_encode_to_slice<P>(packet: &P, expected: &[u8]) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    let mut result: Vec<u8> = vec![0u8; packet.encoded_len()?];
    let encoded_len: usize = packet.encode_to_slice(result.as_mut_slice())?;
    assert_eq!(encoded_len, result.len());
    assert_eq!(
        result.as_slice(),
        expected,
        "result={}, expected={}",
        display_hex(result.as_slice()),
        display_hex(expected)
    );
    Ok(())
}

pub fn test_encode_to_write<P>(packet: &P, expected: &[u8]) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    let result: Vec<u8> = vec![0u8; packet.encoded_len()?];
    let mut result: Cursor<Vec<u8>> = Cursor::new(result);
    let encoded_len: usize = packet.encode_to_write(&mut result)?;
    let result: Vec<u8> = result.into_inner();
    let result: &[u8] = result.as_slice();
    assert_eq!(encoded_len, result.len());
    assert_eq!(
        result,
        expected,
        "result={}, expected={}",
        display_hex(result),
        display_hex(expected)
    );
    Ok(())
}

pub fn test_decode<P>(encoded: &[u8], expected: &P) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    test_decode_from_read(encoded, expected)?;
    test_decode_from_read_prefix(encoded, expected)?;
    Ok(())
}

pub fn test_decode_from_read<P>(encoded: &[u8], expected: &P) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    let read: Vec<u8> = encoded.to_vec();
    let mut read: Cursor<Vec<u8>> = Cursor::new(read);
    let result: P = P::decode_from_read(&mut read)?;
    assert_eq!(&result, expected);
    Ok(())
}

pub fn test_decode_from_read_prefix<P>(encoded: &[u8], expected: &P) -> Result<(), Box<dyn Error>>
where
    P: Packet,
{
    let read: Vec<u8> = Vec::with_capacity(VarIntSize::MAX_ENCODED_LEN + encoded.len());
    let mut write: Cursor<Vec<u8>> = Cursor::new(read);

    if P::wire_type() == LengthPrefixed {
        VarIntSize::from(encoded.len()).encode_to_write(&mut write)?;
    }

    write.write_all(encoded)?;
    let read: Vec<u8> = write.into_inner();
    let mut read: Cursor<Vec<u8>> = Cursor::new(read);

    let result: P = P::decode_from_read_prefix(&mut read)?;
    assert_eq!(&result, expected);

    Ok(())
}
