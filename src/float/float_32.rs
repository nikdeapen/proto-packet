use crate::Packet;
use crate::io::WireType;
use enc::{DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen, Error};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};

/// A 32-bit IEEE 754 binary floating-point value.
///
/// Wraps an [`f32`].
///
/// # [Eq], [Ord] + [Hash]
/// In order to support equality and ordering custom implementations are provided. This makes `NaN`
/// equal to itself and greater than `+Infinity`. Hashing is also fixed with `NaN` and `+-0`.
///
/// # Wire Format
/// The wire format is [`WireType::Fixed4Byte`] and uses little-ending encoding. The values for
/// `NaN` and `+-0` will be normalized for `[]u8` comparison of serialized data.
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Float32 {
    value: f32,
}

impl Float32 {
    //! Constants

    /// The fixed encoded length, in bytes.
    pub const ENCODED_LEN: usize = 4;
}

impl Float32 {
    //! Construction

    /// Creates a new [Float32] wrapping the `value`.
    pub const fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Float32 {
    //! Properties

    /// Gets the underlying [f32] value.
    pub const fn value(self) -> f32 {
        self.value
    }
}

impl Float32 {
    //! Normalization

    /// Returns the normalized form of this value: all `NaN`s map to [`f32::NAN`], `+0.0` and
    /// `-0.0` both map to `+0.0`, and all other values pass through unchanged.
    ///
    /// Used by the wire encoding and by [Hash] so that values which compare equal under [Eq]
    /// encode to identical bytes and hash to identical values.
    pub fn normalize(self) -> f32 {
        if self.value.is_nan() {
            f32::NAN
        } else if self.value == 0.0 {
            0.0
        } else {
            self.value
        }
    }
}

impl From<f32> for Float32 {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl From<Float32> for f32 {
    fn from(value: Float32) -> Self {
        value.value
    }
}

impl PartialEq for Float32 {
    fn eq(&self, other: &Self) -> bool {
        (self.value.is_nan() && other.value.is_nan()) || (self.value == other.value)
    }
}

impl Eq for Float32 {}

impl Hash for Float32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.normalize().to_bits().hash(state);
    }
}

impl PartialOrd for Float32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Float32 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.value.is_nan(), other.value.is_nan()) {
            // All NaNs are equal to each other.
            (true, true) => Ordering::Equal,
            // NaN sorts as the largest element — greater than every non-NaN value.
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            // Both are non-NaN, so IEEE `partial_cmp` is total here. It also returns
            // `Equal` for `+0.0` and `-0.0`, which matches the desired semantics.
            // (`f32::total_cmp` would distinguish them, so we deliberately do not use it.)
            (false, false) => self
                .value
                .partial_cmp(&other.value)
                .expect("non-NaN floats are totally ordered by partial_cmp"),
        }
    }
}

impl EncodedLen for Float32 {
    fn encoded_len(&self) -> Result<usize, Error> {
        Ok(Self::ENCODED_LEN)
    }
}

impl EncodeToSlice for Float32 {
    unsafe fn encode_to_slice_unchecked(&self, target: &mut [u8]) -> Result<usize, Error> {
        let bytes: [u8; Self::ENCODED_LEN] = self.normalize().to_le_bytes();
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), target.as_mut_ptr(), Self::ENCODED_LEN);
        }
        Ok(Self::ENCODED_LEN)
    }
}

impl EncodeToWrite for Float32 {
    fn encode_to_write<W>(&self, w: &mut W) -> Result<usize, Error>
    where
        W: Write,
    {
        let bytes: [u8; Self::ENCODED_LEN] = self.normalize().to_le_bytes();
        w.write_all(&bytes)?;
        Ok(Self::ENCODED_LEN)
    }
}

impl DecodeFromRead for Float32 {
    fn decode_from_read<R>(r: &mut R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes: [u8; Self::ENCODED_LEN] = [0u8; Self::ENCODED_LEN];
        r.read_exact(&mut bytes)?;
        Ok(Self::new(f32::from_le_bytes(bytes)))
    }
}

impl DecodeFromReadPrefix for Float32 {
    fn decode_from_read_prefix_with_first_byte<R>(r: &mut R, first: u8) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut bytes: [u8; Self::ENCODED_LEN] = [0u8; Self::ENCODED_LEN];
        bytes[0] = first;
        r.read_exact(&mut bytes[1..])?;
        Ok(Self::new(f32::from_le_bytes(bytes)))
    }
}

impl Packet for Float32 {
    fn wire() -> WireType {
        WireType::Fixed4Byte
    }
}

#[cfg(test)]
mod tests {
    use crate::Packet;
    use crate::float::Float32;
    use crate::io::WireType;
    use enc::{DecodeFromRead, DecodeFromReadPrefix, EncodeToSlice, EncodeToWrite, EncodedLen};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash_of(value: Float32) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn new() {
        let value: Float32 = Float32::new(1.5);
        assert_eq!(value.value(), 1.5);
    }

    #[test]
    fn value() {
        let cases: &[f32] = &[
            0.0,
            -0.0,
            1.0,
            -1.0,
            f32::MIN,
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        for case in cases {
            assert_eq!(Float32::new(*case).value().to_bits(), case.to_bits());
        }
    }

    #[test]
    fn eq() {
        // Same value compares equal.
        assert_eq!(Float32::new(1.0), Float32::new(1.0));
        // Different values are not equal.
        assert_ne!(Float32::new(1.0), Float32::new(2.0));
        // +0.0 and -0.0 compare equal (despite different bit patterns).
        assert_eq!(Float32::new(0.0), Float32::new(-0.0));
        // All NaN values compare equal regardless of sign or payload.
        assert_eq!(Float32::new(f32::NAN), Float32::new(f32::NAN));
        assert_eq!(Float32::new(f32::NAN), Float32::new(-f32::NAN));
        // NaN is not equal to any non-NaN value.
        assert_ne!(Float32::new(f32::NAN), Float32::new(0.0));
        assert_ne!(Float32::new(f32::NAN), Float32::new(f32::INFINITY));
    }

    #[test]
    fn hash() {
        // Hash agrees with Eq: equal values have equal hashes.
        assert_eq!(hash_of(Float32::new(1.0)), hash_of(Float32::new(1.0)));
        // All NaNs hash to the same value.
        assert_eq!(
            hash_of(Float32::new(f32::NAN)),
            hash_of(Float32::new(-f32::NAN))
        );
        // +0.0 and -0.0 hash to the same value.
        assert_eq!(hash_of(Float32::new(0.0)), hash_of(Float32::new(-0.0)));
    }

    #[test]
    fn cmp() {
        let pos_nan: Float32 = Float32::new(f32::NAN);
        let neg_nan: Float32 = Float32::new(-f32::NAN);
        let pos_inf: Float32 = Float32::new(f32::INFINITY);
        let neg_inf: Float32 = Float32::new(f32::NEG_INFINITY);
        let pos_zero: Float32 = Float32::new(0.0);
        let neg_zero: Float32 = Float32::new(-0.0);
        let one: Float32 = Float32::new(1.0);

        // Normal IEEE ordering for non-NaN values.
        assert!(neg_inf < one);
        assert!(one < pos_inf);
        assert!(neg_inf < pos_inf);

        // +0.0 and -0.0 are tied.
        assert_eq!(pos_zero, neg_zero);

        // NaN sorts as the largest element — greater than +∞.
        assert!(pos_nan > pos_inf);
        assert!(pos_nan > neg_inf);
        assert!(pos_nan > one);
        assert!(pos_nan > pos_zero);

        // All NaNs are tied.
        assert_eq!(pos_nan, neg_nan);
    }

    #[test]
    fn encoded_len() {
        let value: Float32 = Float32::new(1.0);
        assert_eq!(value.encoded_len().unwrap(), 4);
    }

    #[test]
    fn encode_to_slice() {
        // Each case: (input value, expected canonical encoded bytes).
        let cases: &[(f32, [u8; 4])] = &[
            (0.0, [0, 0, 0, 0]),
            (-0.0, [0, 0, 0, 0]), // -0 → +0
            (1.0, 1.0_f32.to_le_bytes()),
            (-1.0, (-1.0_f32).to_le_bytes()),
            (std::f32::consts::PI, std::f32::consts::PI.to_le_bytes()),
            (f32::MIN, f32::MIN.to_le_bytes()),
            (f32::MAX, f32::MAX.to_le_bytes()),
            (f32::INFINITY, f32::INFINITY.to_le_bytes()),
            (f32::NEG_INFINITY, f32::NEG_INFINITY.to_le_bytes()),
            (f32::NAN, f32::NAN.to_le_bytes()),  // canonical NaN
            (-f32::NAN, f32::NAN.to_le_bytes()), // any NaN → canonical NaN
        ];
        for (input, expected) in cases {
            let value: Float32 = Float32::new(*input);
            let mut target: [u8; 4] = [0u8; 4];
            let written: usize = value.encode_to_slice(&mut target).unwrap();
            assert_eq!(written, 4);
            assert_eq!(target, *expected);
        }
    }

    #[test]
    fn encode_canonical() {
        // Different NaN bit patterns must encode to identical bytes.
        let pos_nan: Float32 = Float32::new(f32::NAN);
        let neg_nan: Float32 = Float32::new(-f32::NAN);
        assert_eq!(
            pos_nan.encode_as_vec().unwrap(),
            neg_nan.encode_as_vec().unwrap()
        );

        // +0.0 and -0.0 must encode to identical bytes.
        let pos_zero: Float32 = Float32::new(0.0);
        let neg_zero: Float32 = Float32::new(-0.0);
        assert_eq!(
            pos_zero.encode_as_vec().unwrap(),
            neg_zero.encode_as_vec().unwrap()
        );

        // The canonical encoding of zero is all zero bytes.
        assert_eq!(neg_zero.encode_as_vec().unwrap(), vec![0, 0, 0, 0]);
    }

    #[test]
    fn normalize() {
        // Non-special values pass through.
        assert_eq!(Float32::new(1.5).normalize().to_bits(), 1.5_f32.to_bits());
        assert_eq!(
            Float32::new(f32::INFINITY).normalize().to_bits(),
            f32::INFINITY.to_bits()
        );

        // -0.0 maps to +0.0.
        assert_eq!(Float32::new(-0.0).normalize().to_bits(), 0.0_f32.to_bits());

        // Any NaN maps to canonical f32::NAN.
        assert_eq!(
            Float32::new(-f32::NAN).normalize().to_bits(),
            f32::NAN.to_bits()
        );
    }

    #[test]
    fn encode_to_write() {
        let value: Float32 = Float32::new(std::f32::consts::PI);
        let mut buffer: Vec<u8> = Vec::new();
        let written: usize = value.encode_to_write(&mut buffer).unwrap();
        assert_eq!(written, 4);
        assert_eq!(buffer, std::f32::consts::PI.to_le_bytes());
    }

    #[test]
    fn decode_from_read() {
        let cases: &[f32] = &[
            0.0,
            -0.0,
            1.0,
            -1.0,
            std::f32::consts::PI,
            f32::MIN,
            f32::MAX,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::NAN,
        ];
        for case in cases {
            let bytes: [u8; 4] = case.to_le_bytes();
            let value: Float32 = Float32::decode_from_read(&mut bytes.as_slice()).unwrap();
            assert_eq!(value, Float32::new(*case));
        }
    }

    #[test]
    fn decode_from_read_prefix_with_first_byte() {
        let original: f32 = std::f32::consts::PI;
        let bytes: [u8; 4] = original.to_le_bytes();
        let value: Float32 =
            Float32::decode_from_read_prefix_with_first_byte(&mut &bytes[1..], bytes[0]).unwrap();
        assert_eq!(value, Float32::new(original));
    }

    #[test]
    fn wire() {
        assert_eq!(<Float32 as Packet>::wire(), WireType::Fixed4Byte);
    }
}
