/// Returns `true` if all elements in the slice are equal.
pub(crate) fn all_equal(bytes: &[u8]) -> bool {
    bytes.iter().all(|&v| v == bytes[0])
}

/// Generates boilerplate trait implementations for document structs.
///
/// Implements `AsRef<str>`, `FromStr`, and `Debug` for a given type.
/// Requires that the type has an `as_str()` method and a `parse_strict`
/// function exists in scope.
macro_rules! impl_document_traits {
    ($Type:ident, $Error:ident) => {
        impl AsRef<str> for $Type {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl core::str::FromStr for $Type {
            type Err = $Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                parse_strict(s)
            }
        }

        impl core::fmt::Debug for $Type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}({self})", stringify!($Type))
            }
        }
    };
}

pub(crate) use impl_document_traits;
