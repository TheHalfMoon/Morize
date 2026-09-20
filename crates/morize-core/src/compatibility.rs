//! Dependency-free compatibility classification semantics.
//!
//! This module defines only the in-memory compatibility vocabulary fixed by
//! docs/API_COMPATIBILITY.md. It does not define serialization, parsing,
//! ordering, version negotiation, deprecation timing, persistence, or error
//! taxonomy behavior.

/// Stability status for a Morize surface.
///
/// The five variants match the canonical repository compatibility policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CompatibilityClass {
    /// No compatibility promise exists outside the owning implementation.
    Internal,
    /// The surface may change with release notes or migration guidance.
    Experimental,
    /// The documented compatibility contract applies.
    Stable,
    /// The surface remains supported for a declared deprecation window.
    Deprecated,
    /// The surface is no longer supported.
    Removed,
}

#[cfg(test)]
mod tests {
    use super::CompatibilityClass;
    use std::collections::HashSet;

    const ALL_CLASSES: [CompatibilityClass; 5] = [
        CompatibilityClass::Internal,
        CompatibilityClass::Experimental,
        CompatibilityClass::Stable,
        CompatibilityClass::Deprecated,
        CompatibilityClass::Removed,
    ];

    #[test]
    fn all_five_classes_are_distinct_hashable_values() {
        for (index, class) in ALL_CLASSES.iter().enumerate() {
            for other in ALL_CLASSES.iter().skip(index + 1) {
                assert_ne!(class, other);
            }
        }

        let unique: HashSet<_> = ALL_CLASSES.into_iter().collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn compatibility_class_is_copy() {
        let original = CompatibilityClass::Stable;
        let copied = original;

        assert_eq!(original, copied);
    }
}
