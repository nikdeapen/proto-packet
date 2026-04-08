use crate::io::{Encoder, ListHeader, WireType};
use enc::{EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::io::Write;

macro_rules! encode_int_slice {
    ($primitive:ident, $fixed_wire:expr) => {
        impl Encoder<'_, Vec<$primitive>> {
            //! Utilities

            /// Gets the wire type for encoding elements.
            fn wire_type(&self) -> WireType {
                if self.fixed {
                    $fixed_wire
                } else {
                    WireType::VarInt
                }
            }

            /// Gets the total encoded length of all elements.
            fn elements_len(&self) -> Result<usize, Error> {
                let mut len: usize = 0;
                for element in self.value.iter() {
                    let encoder: Encoder<'_, $primitive> = Encoder::new(element, self.fixed);
                    len += encoder.encoded_len()?;
                }
                Ok(len)
            }
        }

        impl EncodedLen for Encoder<'_, Vec<$primitive>> {
            fn encoded_len(&self) -> Result<usize, Error> {
                let elements: usize = self.elements_len()?;
                let header: ListHeader = ListHeader::new(self.wire_type(), elements);
                Ok(header.encoded_len()? + elements)
            }
        }

        impl EncodeToSlice for Encoder<'_, Vec<$primitive>> {
            unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
                let elements: usize = self.elements_len()?;
                let header: ListHeader = ListHeader::new(self.wire_type(), elements);
                let mut offset: usize = unsafe { header.encode_to_slice_unchecked(target)? };
                if self.fixed {
                    let element_size: usize = std::mem::size_of::<$primitive>();
                    let total: usize = element_size * self.value.len();
                    if cfg!(target_endian = "little") {
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                self.value.as_ptr() as *const u8,
                                target.as_mut_ptr().add(offset),
                                total,
                            );
                        }
                    } else {
                        let body: *mut u8 = unsafe { target.as_mut_ptr().add(offset) };
                        for (i, element) in self.value.iter().enumerate() {
                            unsafe {
                                body.add(i * element_size)
                                    .cast::<$primitive>()
                                    .write_unaligned(element.to_le());
                            }
                        }
                    }
                    offset += total;
                } else {
                    for element in self.value.iter() {
                        let encoder: Encoder<'_, $primitive> = Encoder::new(element, self.fixed);
                        offset += unsafe {
                            encoder.encode_to_slice_unchecked(target.get_unchecked_mut(offset..))?
                        };
                    }
                }
                Ok(offset)
            }
        }

        impl EncodeToWrite for Encoder<'_, Vec<$primitive>> {
            fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
            where
                W: Write,
            {
                let elements: usize = self.elements_len()?;
                let header: ListHeader = ListHeader::new(self.wire_type(), elements);
                let mut written: usize = header.encode_to_write(w)?;
                for element in self.value.iter() {
                    let encoder: Encoder<'_, $primitive> = Encoder::new(element, self.fixed);
                    written += encoder.encode_to_write(w)?;
                }
                Ok(written)
            }
        }
    };
}

encode_int_slice!(u16, WireType::Fixed2Byte);
encode_int_slice!(u32, WireType::Fixed4Byte);
encode_int_slice!(u64, WireType::Fixed8Byte);
encode_int_slice!(u128, WireType::Fixed16Byte);
encode_int_slice!(i16, WireType::Fixed2Byte);
encode_int_slice!(i32, WireType::Fixed4Byte);
encode_int_slice!(i64, WireType::Fixed8Byte);
encode_int_slice!(i128, WireType::Fixed16Byte);

#[cfg(test)]
mod tests {
    use crate::io::Encoder;
    use enc::test;

    #[test]
    fn encode_u16_slice() {
        let cases: &[(&[u16], bool, &[u8])] = &[
            // empty, varint
            (&[], false, &[0xA0]),
            // empty, fixed
            (&[], true, &[0x20]),
            // [1, 2, 3], varint -> ListHeader{VarInt, 3} | varint(1), varint(2), varint(3)
            (&[1, 2, 3], false, &[0xA3, 0x01, 0x02, 0x03]),
            // [1, 2, 3], fixed -> ListHeader{Fixed2Byte, 6} | LE bytes
            (
                &[1, 2, 3],
                true,
                &[0x26, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<u16> = value.to_vec();
            let encoder: Encoder<'_, Vec<u16>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_u32_slice() {
        let cases: &[(&[u32], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x40]),
            (&[1, 2, 3], false, &[0xA3, 0x01, 0x02, 0x03]),
            // ListHeader{Fixed4Byte, 12} = 0x4C
            (
                &[1, 2, 3],
                true,
                &[0x4C, 0x01, 0, 0, 0, 0x02, 0, 0, 0, 0x03, 0, 0, 0],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<u32> = value.to_vec();
            let encoder: Encoder<'_, Vec<u32>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_u64_slice() {
        let cases: &[(&[u64], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x60]),
            (&[1, 2, 3], false, &[0xA3, 0x01, 0x02, 0x03]),
            // ListHeader{Fixed8Byte, 24} = 0x78
            (
                &[1, 2, 3],
                true,
                &[
                    0x78, 0x01, 0, 0, 0, 0, 0, 0, 0, 0x02, 0, 0, 0, 0, 0, 0, 0, 0x03, 0, 0, 0, 0,
                    0, 0, 0,
                ],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<u64> = value.to_vec();
            let encoder: Encoder<'_, Vec<u64>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_u128_slice() {
        let cases: &[(&[u128], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x80]),
            (&[1, 2, 3], false, &[0xA3, 0x01, 0x02, 0x03]),
            // 3 * 16 = 48 > 30, overflow header: 0x80 | 0x1F = 0x9F, then varint(48 - 30) = 0x12
            (
                &[1, 2, 3],
                true,
                &[
                    0x9F, 0x12, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x02, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x03, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0,
                ],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<u128> = value.to_vec();
            let encoder: Encoder<'_, Vec<u128>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i16_slice() {
        let cases: &[(&[i16], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x20]),
            // varint: zigzag(-1)=1, zigzag(0)=0, zigzag(1)=2
            (&[-1, 0, 1], false, &[0xA3, 0x01, 0x00, 0x02]),
            // fixed: -1 LE = [0xFF, 0xFF], 0 LE = [0, 0], 1 LE = [1, 0]
            (
                &[-1, 0, 1],
                true,
                &[0x26, 0xFF, 0xFF, 0x00, 0x00, 0x01, 0x00],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<i16> = value.to_vec();
            let encoder: Encoder<'_, Vec<i16>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i32_slice() {
        let cases: &[(&[i32], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x40]),
            (&[-1, 0, 1], false, &[0xA3, 0x01, 0x00, 0x02]),
            (
                &[-1, 0, 1],
                true,
                &[0x4C, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0x01, 0, 0, 0],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<i32> = value.to_vec();
            let encoder: Encoder<'_, Vec<i32>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i64_slice() {
        let cases: &[(&[i64], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x60]),
            (&[-1, 0, 1], false, &[0xA3, 0x01, 0x00, 0x02]),
            (
                &[-1, 0, 1],
                true,
                &[
                    0x78, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0,
                    0x01, 0, 0, 0, 0, 0, 0, 0,
                ],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<i64> = value.to_vec();
            let encoder: Encoder<'_, Vec<i64>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }

    #[test]
    fn encode_i128_slice() {
        let cases: &[(&[i128], bool, &[u8])] = &[
            (&[], false, &[0xA0]),
            (&[], true, &[0x80]),
            (&[-1, 0, 1], false, &[0xA3, 0x01, 0x00, 0x02]),
            (
                &[-1, 0, 1],
                true,
                &[
                    0x9F, 0x12, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            ),
        ];
        for (value, fixed, expected) in cases {
            let value: Vec<i128> = value.to_vec();
            let encoder: Encoder<'_, Vec<i128>> = Encoder::new(&value, *fixed);
            test::test_encode(&encoder, expected);
        }
    }
}
