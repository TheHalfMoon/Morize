//! Stable typed durable identifier values.
//!
//! This module intentionally defines value and canonical-text semantics only.
//! Identifier generation, persistence, and wire-format integration belong to later Grains.

use core::{fmt, str::FromStr};

/// Number of bytes in every durable identifier value.
pub const ID_BYTE_LENGTH: usize = 16;

/// Number of ASCII characters in the canonical lowercase hexadecimal form.
pub const ID_TEXT_LENGTH: usize = ID_BYTE_LENGTH * 2;

/// Error returned when parsing a durable identifier from canonical text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum IdParseError {
    /// The input does not contain exactly ID_TEXT_LENGTH bytes.
    InvalidLength { actual: usize },
    /// The input contains a byte outside canonical lowercase hexadecimal ASCII.
    NonCanonicalHex { index: usize, byte: u8 },
}

impl fmt::Display for IdParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { actual } => write!(
                f,
                "identifier must be exactly {ID_TEXT_LENGTH} lowercase hexadecimal ASCII characters; got {actual} bytes"
            ),
            Self::NonCanonicalHex { index, byte } => write!(
                f,
                "identifier contains non-canonical hexadecimal byte 0x{byte:02x} at byte index {index}"
            ),
        }
    }
}

impl std::error::Error for IdParseError {}

fn decode_lower_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn parse_bytes(input: &str) -> Result<[u8; ID_BYTE_LENGTH], IdParseError> {
    let text = input.as_bytes();
    if text.len() != ID_TEXT_LENGTH {
        return Err(IdParseError::InvalidLength { actual: text.len() });
    }

    let mut bytes = [0_u8; ID_BYTE_LENGTH];
    let mut index = 0;
    while index < ID_BYTE_LENGTH {
        let high_index = index * 2;
        let low_index = high_index + 1;
        let high = decode_lower_hex(text[high_index]).ok_or(IdParseError::NonCanonicalHex {
            index: high_index,
            byte: text[high_index],
        })?;
        let low = decode_lower_hex(text[low_index]).ok_or(IdParseError::NonCanonicalHex {
            index: low_index,
            byte: text[low_index],
        })?;
        bytes[index] = (high << 4) | low;
        index += 1;
    }
    Ok(bytes)
}

fn write_hex(bytes: &[u8; ID_BYTE_LENGTH], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for byte in bytes {
        write!(f, "{byte:02x}")?;
    }
    Ok(())
}

macro_rules! define_identifier {
    ($($name:ident),+ $(,)?) => {
        $(
            #[doc = concat!("Typed durable identifier ", stringify!($name), ".")]
            #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $name([u8; ID_BYTE_LENGTH]);

            impl $name {
                /// Construct an identifier from its exact opaque bytes.
                #[must_use]
                pub const fn from_bytes(bytes: [u8; ID_BYTE_LENGTH]) -> Self {
                    Self(bytes)
                }

                /// Borrow the exact opaque bytes.
                #[must_use]
                pub const fn as_bytes(&self) -> &[u8; ID_BYTE_LENGTH] {
                    &self.0
                }

                /// Consume the identifier and return its exact opaque bytes.
                #[must_use]
                pub const fn into_bytes(self) -> [u8; ID_BYTE_LENGTH] {
                    self.0
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write_hex(&self.0, f)
                }
            }

            impl fmt::Debug for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "{}({})", stringify!($name), self)
                }
            }

            impl FromStr for $name {
                type Err = IdParseError;

                fn from_str(input: &str) -> Result<Self, Self::Err> {
                    parse_bytes(input).map(Self)
                }
            }
        )+
    };
}

define_identifier!(
    VaultId,
    PrincipalId,
    ScopeId,
    SourceId,
    ObservationId,
    EvidenceId,
    PropositionId,
    MemoryId,
    MemoryVersionId,
    RelationId,
    DecisionId,
    MutationId,
    PolicyRevisionId,
    BranchId,
    SnapshotId,
    ProjectionGenerationId,
    ContextBundleId,
    ConnectorBindingId,
);

#[cfg(test)]
mod tests {
    use super::*;

    const KNOWN_BYTES: [u8; ID_BYTE_LENGTH] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ];
    const KNOWN_TEXT: &str = "00112233445566778899aabbccddeeff";

    macro_rules! assert_identifier_contract {
        ($type:ty) => {{
            let id = <$type>::from_bytes(KNOWN_BYTES);
            assert_eq!(id.as_bytes(), &KNOWN_BYTES);
            assert_eq!(id.into_bytes(), KNOWN_BYTES);
            assert_eq!(id.to_string(), KNOWN_TEXT);
            assert_eq!(
                format!("{id:?}"),
                format!("{}({KNOWN_TEXT})", stringify!($type))
            );

            let parsed: $type = KNOWN_TEXT.parse().expect("canonical identifier must parse");
            assert_eq!(parsed, id);

            assert_eq!(
                "0".repeat(ID_TEXT_LENGTH - 1).parse::<$type>(),
                Err(IdParseError::InvalidLength {
                    actual: ID_TEXT_LENGTH - 1
                })
            );

            let uppercase = KNOWN_TEXT.replace('a', "A");
            assert!(matches!(
                uppercase.parse::<$type>(),
                Err(IdParseError::NonCanonicalHex { .. })
            ));

            let invalid = format!("g{}", &KNOWN_TEXT[1..]);
            assert_eq!(
                invalid.parse::<$type>(),
                Err(IdParseError::NonCanonicalHex {
                    index: 0,
                    byte: b'g'
                })
            );
        }};
    }

    #[test]
    fn every_durable_identifier_obeys_the_same_value_contract() {
        assert_identifier_contract!(VaultId);
        assert_identifier_contract!(PrincipalId);
        assert_identifier_contract!(ScopeId);
        assert_identifier_contract!(SourceId);
        assert_identifier_contract!(ObservationId);
        assert_identifier_contract!(EvidenceId);
        assert_identifier_contract!(PropositionId);
        assert_identifier_contract!(MemoryId);
        assert_identifier_contract!(MemoryVersionId);
        assert_identifier_contract!(RelationId);
        assert_identifier_contract!(DecisionId);
        assert_identifier_contract!(MutationId);
        assert_identifier_contract!(PolicyRevisionId);
        assert_identifier_contract!(BranchId);
        assert_identifier_contract!(SnapshotId);
        assert_identifier_contract!(ProjectionGenerationId);
        assert_identifier_contract!(ContextBundleId);
        assert_identifier_contract!(ConnectorBindingId);
    }

    #[test]
    fn parse_error_messages_are_stable_and_bounded() {
        assert_eq!(
            IdParseError::InvalidLength { actual: 31 }.to_string(),
            "identifier must be exactly 32 lowercase hexadecimal ASCII characters; got 31 bytes"
        );
        assert_eq!(
            IdParseError::NonCanonicalHex {
                index: 4,
                byte: b'A'
            }
            .to_string(),
            "identifier contains non-canonical hexadecimal byte 0x41 at byte index 4"
        );
    }

    macro_rules! assert_identifier_round_trip_matrix {
        ($type:ty) => {{
            for repeated_byte in u8::MIN..=u8::MAX {
                let bytes = [repeated_byte; ID_BYTE_LENGTH];
                let id = <$type>::from_bytes(bytes);
                let text = id.to_string();

                assert_eq!(text.len(), ID_TEXT_LENGTH);
                assert!(text
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
                assert_eq!(text, text.to_ascii_lowercase());

                let parsed: $type = text.parse().expect("canonical matrix value must parse");
                assert_eq!(parsed, id);
                assert_eq!(parsed.into_bytes(), bytes);
            }
        }};
    }

    macro_rules! assert_identifier_parser_rejection_matrix {
        ($type:ty) => {{
            const NON_CANONICAL_ASCII: [u8; 4] = [b'A', b'G', b'/', b':'];

            for index in 0..ID_TEXT_LENGTH {
                for byte in NON_CANONICAL_ASCII {
                    let mut candidate = vec![b'0'; ID_TEXT_LENGTH];
                    candidate[index] = byte;
                    let candidate =
                        String::from_utf8(candidate).expect("matrix input is deterministic ASCII");

                    assert_eq!(
                        candidate.parse::<$type>(),
                        Err(IdParseError::NonCanonicalHex { index, byte })
                    );
                }
            }
        }};
    }

    #[test]
    fn durable_identifier_round_trip_property_matrix_is_canonical() {
        assert_identifier_round_trip_matrix!(VaultId);
        assert_identifier_round_trip_matrix!(PrincipalId);
        assert_identifier_round_trip_matrix!(ScopeId);
        assert_identifier_round_trip_matrix!(SourceId);
        assert_identifier_round_trip_matrix!(ObservationId);
        assert_identifier_round_trip_matrix!(EvidenceId);
        assert_identifier_round_trip_matrix!(PropositionId);
        assert_identifier_round_trip_matrix!(MemoryId);
        assert_identifier_round_trip_matrix!(MemoryVersionId);
        assert_identifier_round_trip_matrix!(RelationId);
        assert_identifier_round_trip_matrix!(DecisionId);
        assert_identifier_round_trip_matrix!(MutationId);
        assert_identifier_round_trip_matrix!(PolicyRevisionId);
        assert_identifier_round_trip_matrix!(BranchId);
        assert_identifier_round_trip_matrix!(SnapshotId);
        assert_identifier_round_trip_matrix!(ProjectionGenerationId);
        assert_identifier_round_trip_matrix!(ContextBundleId);
        assert_identifier_round_trip_matrix!(ConnectorBindingId);
    }

    #[test]
    fn durable_identifier_parser_property_matrix_rejects_noncanonical_ascii() {
        assert_identifier_parser_rejection_matrix!(VaultId);
        assert_identifier_parser_rejection_matrix!(PrincipalId);
        assert_identifier_parser_rejection_matrix!(ScopeId);
        assert_identifier_parser_rejection_matrix!(SourceId);
        assert_identifier_parser_rejection_matrix!(ObservationId);
        assert_identifier_parser_rejection_matrix!(EvidenceId);
        assert_identifier_parser_rejection_matrix!(PropositionId);
        assert_identifier_parser_rejection_matrix!(MemoryId);
        assert_identifier_parser_rejection_matrix!(MemoryVersionId);
        assert_identifier_parser_rejection_matrix!(RelationId);
        assert_identifier_parser_rejection_matrix!(DecisionId);
        assert_identifier_parser_rejection_matrix!(MutationId);
        assert_identifier_parser_rejection_matrix!(PolicyRevisionId);
        assert_identifier_parser_rejection_matrix!(BranchId);
        assert_identifier_parser_rejection_matrix!(SnapshotId);
        assert_identifier_parser_rejection_matrix!(ProjectionGenerationId);
        assert_identifier_parser_rejection_matrix!(ContextBundleId);
        assert_identifier_parser_rejection_matrix!(ConnectorBindingId);
    }

}
