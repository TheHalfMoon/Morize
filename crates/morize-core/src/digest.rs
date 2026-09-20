//! Stable SHA-256 digest value and canonical text semantics.
//!
//! This module defines representation only. It does not compute SHA-256,
//! canonicalize content, authenticate data, derive storage paths, or attach
//! authority or trust semantics to digest equality.

use core::{fmt, str::FromStr};

/// Number of opaque bytes in a SHA-256 digest value.
pub const SHA256_DIGEST_BYTE_LENGTH: usize = 32;

/// Number of ASCII characters in the canonical lowercase hexadecimal form.
pub const SHA256_DIGEST_TEXT_LENGTH: usize = SHA256_DIGEST_BYTE_LENGTH * 2;

/// Error returned when parsing a SHA-256 digest from canonical text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DigestParseError {
    /// The input does not contain exactly SHA256_DIGEST_TEXT_LENGTH bytes.
    InvalidLength {
        /// Actual UTF-8 byte length of the rejected input.
        actual: usize,
    },
    /// The input contains a byte outside canonical lowercase hexadecimal ASCII.
    NonCanonicalHex {
        /// Byte index of the first non-canonical byte.
        index: usize,
        /// Rejected byte.
        byte: u8,
    },
}

impl fmt::Display for DigestParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { actual } => write!(
                f,
                "SHA-256 digest must be exactly {SHA256_DIGEST_TEXT_LENGTH} lowercase hexadecimal ASCII characters; got {actual} bytes"
            ),
            Self::NonCanonicalHex { index, byte } => write!(
                f,
                "SHA-256 digest contains non-canonical hexadecimal byte 0x{byte:02x} at byte index {index}"
            ),
        }
    }
}

impl std::error::Error for DigestParseError {}

fn decode_lower_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn parse_bytes(input: &str) -> Result<[u8; SHA256_DIGEST_BYTE_LENGTH], DigestParseError> {
    let text = input.as_bytes();
    if text.len() != SHA256_DIGEST_TEXT_LENGTH {
        return Err(DigestParseError::InvalidLength { actual: text.len() });
    }

    let mut bytes = [0_u8; SHA256_DIGEST_BYTE_LENGTH];
    let mut index = 0;
    while index < SHA256_DIGEST_BYTE_LENGTH {
        let high_index = index * 2;
        let low_index = high_index + 1;
        let high =
            decode_lower_hex(text[high_index]).ok_or(DigestParseError::NonCanonicalHex {
                index: high_index,
                byte: text[high_index],
            })?;
        let low = decode_lower_hex(text[low_index]).ok_or(DigestParseError::NonCanonicalHex {
            index: low_index,
            byte: text[low_index],
        })?;
        bytes[index] = (high << 4) | low;
        index += 1;
    }

    Ok(bytes)
}

fn write_hex(
    bytes: &[u8; SHA256_DIGEST_BYTE_LENGTH],
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    for byte in bytes {
        write!(f, "{byte:02x}")?;
    }
    Ok(())
}

/// Opaque SHA-256 digest bytes with one strict canonical text representation.
///
/// Constructing this value from bytes does not assert that those bytes were
/// actually produced by the SHA-256 algorithm.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sha256Digest([u8; SHA256_DIGEST_BYTE_LENGTH]);

impl Sha256Digest {
    /// Construct a digest value from exactly 32 opaque bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; SHA256_DIGEST_BYTE_LENGTH]) -> Self {
        Self(bytes)
    }

    /// Borrow the exact opaque digest bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; SHA256_DIGEST_BYTE_LENGTH] {
        &self.0
    }

    /// Consume the digest and return its exact opaque bytes.
    #[must_use]
    pub const fn into_bytes(self) -> [u8; SHA256_DIGEST_BYTE_LENGTH] {
        self.0
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_hex(&self.0, f)
    }
}

impl fmt::Debug for Sha256Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sha256Digest")
            .field("byte_length", &SHA256_DIGEST_BYTE_LENGTH)
            .finish()
    }
}

impl FromStr for Sha256Digest {
    type Err = DigestParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_bytes(input).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash(value: Sha256Digest) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn exact_bytes_and_canonical_text_round_trip() {
        let bytes = [0xab; SHA256_DIGEST_BYTE_LENGTH];
        let digest = Sha256Digest::from_bytes(bytes);
        let canonical = "ab".repeat(SHA256_DIGEST_BYTE_LENGTH);

        assert_eq!(SHA256_DIGEST_BYTE_LENGTH, 32);
        assert_eq!(SHA256_DIGEST_TEXT_LENGTH, 64);
        assert_eq!(digest.as_bytes(), &bytes);
        assert_eq!(digest.into_bytes(), bytes);
        assert_eq!(digest.to_string(), canonical);

        let parsed: Sha256Digest = canonical.parse().expect("canonical digest must parse");
        assert_eq!(parsed, digest);
        assert_eq!(parsed.into_bytes(), bytes);
    }

    #[test]
    fn invalid_lengths_report_only_actual_byte_count() {
        let invalid_inputs = [
            String::new(),
            String::from("0"),
            "0".repeat(SHA256_DIGEST_TEXT_LENGTH - 1),
            "0".repeat(SHA256_DIGEST_TEXT_LENGTH + 1),
        ];

        for input in invalid_inputs {
            assert_eq!(
                input.parse::<Sha256Digest>(),
                Err(DigestParseError::InvalidLength {
                    actual: input.len(),
                })
            );
        }

        assert_eq!(
            DigestParseError::InvalidLength { actual: 63 }.to_string(),
            "SHA-256 digest must be exactly 64 lowercase hexadecimal ASCII characters; got 63 bytes"
        );
    }

    #[test]
    fn repeated_byte_property_matrix_is_canonical() {
        for repeated_byte in u8::MIN..=u8::MAX {
            let bytes = [repeated_byte; SHA256_DIGEST_BYTE_LENGTH];
            let digest = Sha256Digest::from_bytes(bytes);
            let text = digest.to_string();

            assert_eq!(text.len(), SHA256_DIGEST_TEXT_LENGTH);
            assert!(
                text.bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            );
            assert_eq!(text, text.to_ascii_lowercase());

            let parsed: Sha256Digest = text.parse().expect("canonical matrix value must parse");
            assert_eq!(parsed, digest);
            assert_eq!(parsed.into_bytes(), bytes);
        }
    }

    #[test]
    fn parser_rejects_noncanonical_ascii_at_every_text_position() {
        const NON_CANONICAL_ASCII: [u8; 4] = [b'A', b'G', b'/', b':'];

        for index in 0..SHA256_DIGEST_TEXT_LENGTH {
            for byte in NON_CANONICAL_ASCII {
                let mut candidate = vec![b'0'; SHA256_DIGEST_TEXT_LENGTH];
                candidate[index] = byte;
                let candidate =
                    String::from_utf8(candidate).expect("matrix input is deterministic ASCII");

                assert_eq!(
                    candidate.parse::<Sha256Digest>(),
                    Err(DigestParseError::NonCanonicalHex { index, byte })
                );
            }
        }
    }

    #[test]
    fn debug_and_parse_errors_do_not_disclose_digest_text() {
        let digest = Sha256Digest::from_bytes([0xab; SHA256_DIGEST_BYTE_LENGTH]);
        let canonical = digest.to_string();
        let debug = format!("{digest:?}");

        assert_eq!(debug, "Sha256Digest { byte_length: 32 }");
        assert!(!debug.contains(&canonical));

        let mut candidate = "0".repeat(SHA256_DIGEST_TEXT_LENGTH);
        candidate.replace_range(17..18, "G");
        let error = candidate.parse::<Sha256Digest>().unwrap_err();
        let message = error.to_string();

        assert_eq!(
            error,
            DigestParseError::NonCanonicalHex {
                index: 17,
                byte: b'G',
            }
        );
        assert!(!message.contains(&candidate));
    }

    #[test]
    fn value_semantics_are_deterministic() {
        let low = Sha256Digest::from_bytes([0x00; SHA256_DIGEST_BYTE_LENGTH]);
        let low_again = Sha256Digest::from_bytes([0x00; SHA256_DIGEST_BYTE_LENGTH]);
        let high = Sha256Digest::from_bytes([0xff; SHA256_DIGEST_BYTE_LENGTH]);

        assert_eq!(low, low_again);
        assert_ne!(low, high);
        assert!(low < high);
        assert_eq!(hash(low), hash(low_again));

        let copied = low;
        assert_eq!(copied, low);
    }
}
