use crate::io::WireType;
use crate::io::WireType::*;

impl WireType {
    //! Bits

    /// Gets the wire type from the low 3-bits of `b`.
    pub fn from_low_3_bits(b: u8) -> Self {
        match b & 0x7 {
            0 => Fixed1Byte,
            1 => Fixed2Byte,
            2 => Fixed4Byte,
            3 => Fixed8Byte,
            4 => Fixed16Byte,
            5 => VarInt,
            6 => LengthPrefixed,
            7 => List,
            _ => unreachable!(),
        }
    }

    /// Gets the wire type from the high 3-bits of `b`.
    pub fn from_high_3_bits(b: u8) -> Self {
        Self::from_low_3_bits(b >> 5)
    }

    /// Converts the wire type to the low 3-bits of a `u8`.
    pub fn to_low_3_bits(&self) -> u8 {
        match self {
            Fixed1Byte => 0,
            Fixed2Byte => 1,
            Fixed4Byte => 2,
            Fixed8Byte => 3,
            Fixed16Byte => 4,
            VarInt => 5,
            LengthPrefixed => 6,
            List => 7,
        }
    }

    /// Converts the wire type to the high 3-bits of a `u8`.
    pub fn to_high_3_bits(&self) -> u8 {
        self.to_low_3_bits() << 5
    }
}

#[cfg(test)]
mod tests {
    use crate::io::WireType;
    use crate::io::WireType::VarInt;

    #[test]
    fn from_low_3_bits() {
        assert_eq!(WireType::from_low_3_bits(0b0000_0101), VarInt);
    }

    #[test]
    fn from_high_3_bits() {
        assert_eq!(WireType::from_high_3_bits(0b1010_0000), VarInt);
    }
    #[test]
    fn to_low_3_bits() {
        assert_eq!(VarInt.to_low_3_bits(), 0b0000_0101);
    }

    #[test]
    fn to_high_3_bits() {
        assert_eq!(VarInt.to_high_3_bits(), 0b1010_0000);
    }
}
