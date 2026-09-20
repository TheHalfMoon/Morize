//! Dependency-free bounded UTF-8 value semantics.
//!
//! This module enforces byte ceilings only. It does not normalize, trim,
//! case-fold, serialize, persist, or apply field-specific content policy.

use core::{fmt, str::FromStr};

/// Error returned when UTF-8 text exceeds its configured byte ceiling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Utf8BoundsError {
    /// The input contains more UTF-8 bytes than the configured maximum.
    TooLong {
        /// Maximum accepted UTF-8 byte length.
        max_bytes: usize,
        /// Actual UTF-8 byte length of the rejected input.
        actual_bytes: usize,
    },
}

impl fmt::Display for Utf8BoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong {
                max_bytes,
                actual_bytes,
            } => write!(
                f,
                "UTF-8 text exceeds the {max_bytes}-byte limit: {actual_bytes} bytes"
            ),
        }
    }
}

impl std::error::Error for Utf8BoundsError {}

/// Owned UTF-8 text whose encoded byte length cannot exceed `MAX_BYTES`.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoundedUtf8<const MAX_BYTES: usize>(String);

impl<const MAX_BYTES: usize> BoundedUtf8<MAX_BYTES> {
    /// Construct a bounded value from an owned UTF-8 string.
    pub fn new(value: String) -> Result<Self, Utf8BoundsError> {
        Self::validate(value.len())?;
        Ok(Self(value))
    }

    fn validate(actual_bytes: usize) -> Result<(), Utf8BoundsError> {
        if actual_bytes > MAX_BYTES {
            return Err(Utf8BoundsError::TooLong {
                max_bytes: MAX_BYTES,
                actual_bytes,
            });
        }
        Ok(())
    }

    /// Borrow the preserved UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return the preserved UTF-8 byte length.
    #[must_use]
    pub fn len_bytes(&self) -> usize {
        self.0.len()
    }

    /// Return whether the preserved text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consume the value and return the preserved owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }

    /// Return the configured UTF-8 byte ceiling.
    #[must_use]
    pub const fn max_bytes() -> usize {
        MAX_BYTES
    }
}

impl<const MAX_BYTES: usize> TryFrom<String> for BoundedUtf8<MAX_BYTES> {
    type Error = Utf8BoundsError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MAX_BYTES: usize> TryFrom<&str> for BoundedUtf8<MAX_BYTES> {
    type Error = Utf8BoundsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::validate(value.len())?;
        Ok(Self(value.to_owned()))
    }
}

impl<const MAX_BYTES: usize> FromStr for BoundedUtf8<MAX_BYTES> {
    type Err = Utf8BoundsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::try_from(value)
    }
}

impl<const MAX_BYTES: usize> AsRef<str> for BoundedUtf8<MAX_BYTES> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const MAX_BYTES: usize> fmt::Display for BoundedUtf8<MAX_BYTES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const MAX_BYTES: usize> fmt::Debug for BoundedUtf8<MAX_BYTES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoundedUtf8")
            .field("max_bytes", &MAX_BYTES)
            .field("len_bytes", &self.len_bytes())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash<const MAX_BYTES: usize>(value: &BoundedUtf8<MAX_BYTES>) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn byte_boundaries_cover_ascii_multibyte_and_zero_limits() {
        assert_eq!(BoundedUtf8::<4>::try_from("").unwrap().len_bytes(), 0);
        assert_eq!(BoundedUtf8::<4>::try_from("abcd").unwrap().len_bytes(), 4);
        assert_eq!(BoundedUtf8::<4>::try_from("éé").unwrap().len_bytes(), 4);

        assert_eq!(
            BoundedUtf8::<4>::try_from("abcde"),
            Err(Utf8BoundsError::TooLong {
                max_bytes: 4,
                actual_bytes: 5,
            })
        );
        assert_eq!(
            BoundedUtf8::<4>::try_from("ééa"),
            Err(Utf8BoundsError::TooLong {
                max_bytes: 4,
                actual_bytes: 5,
            })
        );
        assert_eq!(BoundedUtf8::<0>::try_from("").unwrap().as_str(), "");
        assert_eq!(
            BoundedUtf8::<0>::try_from("a"),
            Err(Utf8BoundsError::TooLong {
                max_bytes: 0,
                actual_bytes: 1,
            })
        );
    }

    #[test]
    fn construction_preserves_bytes_without_normalization_or_truncation() {
        let decomposed = "e\u{301}";
        let owned = BoundedUtf8::<3>::try_from(decomposed.to_owned()).unwrap();
        let borrowed = BoundedUtf8::<3>::try_from(decomposed).unwrap();
        let parsed: BoundedUtf8<3> = decomposed.parse().unwrap();

        assert_eq!(owned.as_str().as_bytes(), decomposed.as_bytes());
        assert_eq!(borrowed.as_str().as_bytes(), decomposed.as_bytes());
        assert_eq!(parsed.as_str().as_bytes(), decomposed.as_bytes());
        assert_eq!(
            owned.clone().into_string().as_bytes(),
            decomposed.as_bytes()
        );
        assert_eq!(BoundedUtf8::<3>::max_bytes(), 3);
        assert!(!owned.is_empty());

        let over = "abcdef";
        assert_eq!(
            BoundedUtf8::<5>::try_from(over),
            Err(Utf8BoundsError::TooLong {
                max_bytes: 5,
                actual_bytes: 6,
            })
        );
        assert_eq!(over, "abcdef");
    }

    #[test]
    fn value_semantics_and_display_use_preserved_text() {
        let alpha = BoundedUtf8::<8>::try_from("alpha").unwrap();
        let alpha_again = BoundedUtf8::<8>::try_from("alpha").unwrap();
        let beta = BoundedUtf8::<8>::try_from("beta").unwrap();

        assert_eq!(alpha, alpha_again);
        assert_ne!(alpha, beta);
        assert!(alpha < beta);
        assert_eq!(hash(&alpha), hash(&alpha_again));
        assert_eq!(alpha.to_string(), "alpha");
        assert_eq!(alpha.as_ref(), "alpha");
    }

    #[test]
    fn debug_and_errors_do_not_disclose_text() {
        let secret = "classified-memory";
        let value = BoundedUtf8::<64>::try_from(secret).unwrap();
        let debug = format!("{value:?}");
        assert!(!debug.contains(secret));
        assert!(debug.contains("max_bytes"));
        assert!(debug.contains("len_bytes"));

        let rejected = "sensitive-secret";
        let error = BoundedUtf8::<4>::try_from(rejected).unwrap_err();
        let message = error.to_string();
        assert!(!message.contains(rejected));
        assert_eq!(
            error,
            Utf8BoundsError::TooLong {
                max_bytes: 4,
                actual_bytes: rejected.len(),
            }
        );
    }

    fn assert_utf8_property_matrix<const MAX_BYTES: usize>() {
        const CORPUS: &[&str] = &[
            "",
            "a",
            "ab",
            "ascii",
            "é",
            "éé",
            "🙂",
            "e\u{301}",
            "mañana",
            "東京",
            "memory-safe",
        ];

        for value in CORPUS {
            let actual_bytes = value.len();
            let should_accept = actual_bytes <= MAX_BYTES;

            match BoundedUtf8::<MAX_BYTES>::try_from(*value) {
                Ok(bounded) => {
                    assert!(should_accept);
                    assert_eq!(bounded.len_bytes(), actual_bytes);
                    assert_eq!(bounded.as_str().as_bytes(), value.as_bytes());
                    assert_eq!(bounded.clone().into_string().as_bytes(), value.as_bytes());
                }
                Err(error) => {
                    assert!(!should_accept);
                    assert_eq!(
                        error,
                        Utf8BoundsError::TooLong {
                            max_bytes: MAX_BYTES,
                            actual_bytes,
                        }
                    );
                }
            }
        }
    }

    #[test]
    fn bounded_utf8_property_matrix_matches_exact_byte_limits() {
        assert_utf8_property_matrix::<0>();
        assert_utf8_property_matrix::<1>();
        assert_utf8_property_matrix::<2>();
        assert_utf8_property_matrix::<3>();
        assert_utf8_property_matrix::<4>();
        assert_utf8_property_matrix::<5>();
        assert_utf8_property_matrix::<8>();
        assert_utf8_property_matrix::<16>();
    }

}
