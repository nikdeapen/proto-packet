/// Implements `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` for an enum type by tag number.
///
/// The enum must implement `proto_packet::io::WithTagNumber`.
#[macro_export]
macro_rules! impl_enum_eq {
    ($enum_type:ty) => {
        impl PartialEq for $enum_type {
            fn eq(&self, other: &Self) -> bool {
                use $crate::io::WithTagNumber;
                self.tag() == other.tag()
            }
        }

        impl Eq for $enum_type {}

        impl PartialOrd for $enum_type {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $enum_type {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                use $crate::io::WithTagNumber;
                self.tag().cmp(&other.tag())
            }
        }

        impl std::hash::Hash for $enum_type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                use $crate::io::WithTagNumber;
                self.tag().hash(state);
            }
        }
    };
}
