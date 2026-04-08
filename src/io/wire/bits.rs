use crate::io::WireType;

impl WireType {
    //! Bits

    /// Gets the wire type from the low 3-bits of `b`.
    pub fn from_low_3_bits(b: u8) -> Self {
        // `WireType` is `#[repr(u8)]` with discriminants `0..=7`, so every value of `b & 0x7`
        // is a valid discriminant.
        unsafe { std::mem::transmute::<u8, WireType>(b & 0x7) }
    }

    /// Gets the wire type from the high 3-bits of `b`.
    pub fn from_high_3_bits(b: u8) -> Self {
        Self::from_low_3_bits(b >> 5)
    }

    /// Converts the wire type to the low 3-bits of a `u8`.
    pub fn to_low_3_bits(self) -> u8 {
        self as u8
    }

    /// Converts the wire type to the high 3-bits of a `u8`.
    pub fn to_high_3_bits(self) -> u8 {
        (self as u8) << 5
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType;
    use crate::io::WireType::*;

    #[test]
    fn from_low_3_bits() {
        let cases: &[(u8, WireType)] = &[
            (0b0000_0000, Fixed1Byte),
            (0b0000_0001, Fixed2Byte),
            (0b0000_0010, Fixed4Byte),
            (0b0000_0011, Fixed8Byte),
            (0b0000_0100, Fixed16Byte),
            (0b0000_0101, VarInt),
            (0b0000_0110, LengthPrefixed),
            (0b0000_0111, List),
            // High bits are masked off — these should still produce the same results.
            (0b1111_1000, Fixed1Byte),
            (0b1111_1101, VarInt),
        ];
        for (input, expected) in cases {
            assert_eq!(
                WireType::from_low_3_bits(*input),
                *expected,
                "input={input:#010b}"
            );
        }
    }

    #[test]
    fn from_high_3_bits() {
        let cases: &[(u8, WireType)] = &[
            (0b0000_0000, Fixed1Byte),
            (0b0010_0000, Fixed2Byte),
            (0b0100_0000, Fixed4Byte),
            (0b0110_0000, Fixed8Byte),
            (0b1000_0000, Fixed16Byte),
            (0b1010_0000, VarInt),
            (0b1100_0000, LengthPrefixed),
            (0b1110_0000, List),
            // Low bits are ignored — these should still produce the same results.
            (0b0001_1111, Fixed1Byte),
            (0b1011_1111, VarInt),
        ];
        for (input, expected) in cases {
            assert_eq!(
                WireType::from_high_3_bits(*input),
                *expected,
                "input={input:#010b}"
            );
        }
    }

    #[test]
    fn to_low_3_bits() {
        let cases: &[(WireType, u8)] = &[
            (Fixed1Byte, 0b0000_0000),
            (Fixed2Byte, 0b0000_0001),
            (Fixed4Byte, 0b0000_0010),
            (Fixed8Byte, 0b0000_0011),
            (Fixed16Byte, 0b0000_0100),
            (VarInt, 0b0000_0101),
            (LengthPrefixed, 0b0000_0110),
            (List, 0b0000_0111),
        ];
        for (wire, expected) in cases {
            assert_eq!(wire.to_low_3_bits(), *expected, "wire={wire:?}");
        }
    }

    #[test]
    fn to_high_3_bits() {
        let cases: &[(WireType, u8)] = &[
            (Fixed1Byte, 0b0000_0000),
            (Fixed2Byte, 0b0010_0000),
            (Fixed4Byte, 0b0100_0000),
            (Fixed8Byte, 0b0110_0000),
            (Fixed16Byte, 0b1000_0000),
            (VarInt, 0b1010_0000),
            (LengthPrefixed, 0b1100_0000),
            (List, 0b1110_0000),
        ];
        for (wire, expected) in cases {
            assert_eq!(wire.to_high_3_bits(), *expected, "wire={wire:?}");
        }
    }
}
