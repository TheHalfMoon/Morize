//! Normalized dependency-free timestamp value semantics.
//!
//! This module defines only an in-memory instant representation required by
//! docs/DATA_MODEL.md. It does not acquire clocks, perform duration arithmetic,
//! interpret calendars/timezones/leap seconds, parse or format text, serialize,
//! persist, or define transport behavior.

const NANOS_PER_SECOND: u32 = 1_000_000_000;

/// Error returned when timestamp parts are not in canonical normalized form.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimestampError {
    /// The subsecond nanosecond field is outside the canonical range.
    NanosecondsOutOfRange {
        /// Rejected subsecond nanosecond value.
        actual: u32,
    },
}

/// A normalized instant represented as Unix floor-seconds plus nanoseconds.
///
/// `unix_seconds` is the mathematical floor second relative to the Unix epoch,
/// while `subsecond_nanoseconds` is always in `0..=999_999_999`. This gives
/// every represented instant one value representation, including instants
/// before the epoch. For example, one nanosecond before the epoch is
/// `(-1, 999_999_999)`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Timestamp {
    unix_seconds: i64,
    subsecond_nanoseconds: u32,
}

impl Timestamp {
    /// The Unix epoch instant.
    pub const UNIX_EPOCH: Self = Self {
        unix_seconds: 0,
        subsecond_nanoseconds: 0,
    };

    /// Construct a timestamp from already-normalized Unix parts.
    ///
    /// Values with `subsecond_nanoseconds >= 1_000_000_000` are rejected
    /// rather than normalized, truncated, wrapped, or otherwise changed.
    pub const fn from_unix_parts(
        unix_seconds: i64,
        subsecond_nanoseconds: u32,
    ) -> Result<Self, TimestampError> {
        if subsecond_nanoseconds >= NANOS_PER_SECOND {
            return Err(TimestampError::NanosecondsOutOfRange {
                actual: subsecond_nanoseconds,
            });
        }

        Ok(Self {
            unix_seconds,
            subsecond_nanoseconds,
        })
    }

    /// Return the signed Unix floor-second component.
    #[must_use]
    pub const fn unix_seconds(&self) -> i64 {
        self.unix_seconds
    }

    /// Return the normalized subsecond nanosecond component.
    #[must_use]
    pub const fn subsecond_nanoseconds(&self) -> u32 {
        self.subsecond_nanoseconds
    }
}

#[cfg(test)]
mod tests {
    use super::{Timestamp, TimestampError};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn hash(value: Timestamp) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn epoch_and_valid_nanosecond_boundaries_are_exact() {
        assert_eq!(Timestamp::UNIX_EPOCH.unix_seconds(), 0);
        assert_eq!(Timestamp::UNIX_EPOCH.subsecond_nanoseconds(), 0);

        let last_nanosecond =
            Timestamp::from_unix_parts(0, 999_999_999).expect("maximum nanosecond is valid");
        let next_second = Timestamp::from_unix_parts(1, 0).expect("whole second is valid");

        assert_eq!(last_nanosecond.unix_seconds(), 0);
        assert_eq!(last_nanosecond.subsecond_nanoseconds(), 999_999_999);
        assert!(last_nanosecond < next_second);
    }

    #[test]
    fn invalid_nanoseconds_are_rejected_without_normalization() {
        assert_eq!(
            Timestamp::from_unix_parts(7, 1_000_000_000),
            Err(TimestampError::NanosecondsOutOfRange {
                actual: 1_000_000_000,
            })
        );
        assert_eq!(
            Timestamp::from_unix_parts(-7, u32::MAX),
            Err(TimestampError::NanosecondsOutOfRange { actual: u32::MAX })
        );
    }

    #[test]
    fn negative_instants_follow_floor_second_ordering() {
        let half_second_before =
            Timestamp::from_unix_parts(-1, 500_000_000).expect("normalized negative instant");
        let one_nanosecond_before =
            Timestamp::from_unix_parts(-1, 999_999_999).expect("normalized negative instant");
        let one_nanosecond_after =
            Timestamp::from_unix_parts(0, 1).expect("normalized positive instant");

        assert!(half_second_before < one_nanosecond_before);
        assert!(one_nanosecond_before < Timestamp::UNIX_EPOCH);
        assert!(Timestamp::UNIX_EPOCH < one_nanosecond_after);
    }

    #[test]
    fn timestamp_value_semantics_are_deterministic_and_copy() {
        let original = Timestamp::from_unix_parts(-42, 123_456_789).unwrap();
        let copied = original;
        let same = Timestamp::from_unix_parts(-42, 123_456_789).unwrap();
        let different = Timestamp::from_unix_parts(-42, 123_456_790).unwrap();

        assert_eq!(original, copied);
        assert_eq!(original, same);
        assert_ne!(original, different);
        assert_eq!(hash(original), hash(same));
        assert!(original < different);
    }
}
