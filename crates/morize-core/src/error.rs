//! Dependency-free public error-category semantics.
//!
//! This module defines only the in-memory baseline category vocabulary fixed
//! by docs/API_COMPATIBILITY.md. It does not define concrete error payloads,
//! message text, source chaining, retry policy, transport mappings,
//! serialization, numeric encoding, persistence, or authorization disclosure
//! behavior.

/// Baseline category for a public Morize error outcome.
///
/// The canonical compatibility policy requires public APIs to distinguish at
/// least these categories, so this enum remains open to future additions.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ErrorCategory {
    /// Input does not satisfy the applicable contract.
    InvalidInput,
    /// The requested version is not supported.
    UnsupportedVersion,
    /// Authentication did not establish an accepted principal.
    AuthenticationFailure,
    /// The authenticated principal is not authorized for the operation.
    AuthorizationDenial,
    /// A conflict or stale precondition prevents the operation.
    ConflictOrStalePrecondition,
    /// The operation requires review before it can proceed.
    ReviewRequired,
    /// The operation or material is quarantined.
    Quarantine,
    /// The requested resource is not available; this category is also used
    /// where protected existence must not be revealed.
    NotFound,
    /// A resource or budget limit prevents the operation.
    ResourceOrBudgetExceeded,
    /// A provider failure is transient.
    TransientProviderFailure,
    /// A connector failure is permanent.
    PermanentConnectorFailure,
    /// The outcome of a write cannot be resolved or is unknown.
    UnresolvedOrUnknownWriteOutcome,
    /// Internal corruption or recovery work is required.
    InternalCorruptionOrRecoveryRequired,
}

#[cfg(test)]
mod tests {
    use super::ErrorCategory;
    use std::collections::HashSet;

    const ALL_CATEGORIES: [ErrorCategory; 13] = [
        ErrorCategory::InvalidInput,
        ErrorCategory::UnsupportedVersion,
        ErrorCategory::AuthenticationFailure,
        ErrorCategory::AuthorizationDenial,
        ErrorCategory::ConflictOrStalePrecondition,
        ErrorCategory::ReviewRequired,
        ErrorCategory::Quarantine,
        ErrorCategory::NotFound,
        ErrorCategory::ResourceOrBudgetExceeded,
        ErrorCategory::TransientProviderFailure,
        ErrorCategory::PermanentConnectorFailure,
        ErrorCategory::UnresolvedOrUnknownWriteOutcome,
        ErrorCategory::InternalCorruptionOrRecoveryRequired,
    ];

    #[test]
    fn all_baseline_categories_are_distinct_hashable_values() {
        for (index, category) in ALL_CATEGORIES.iter().enumerate() {
            for other in ALL_CATEGORIES.iter().skip(index + 1) {
                assert_ne!(category, other);
            }
        }

        let unique: HashSet<_> = ALL_CATEGORIES.into_iter().collect();
        assert_eq!(unique.len(), 13);
    }

    #[test]
    fn error_category_is_copy() {
        let original = ErrorCategory::ReviewRequired;
        let copied = original;

        assert_eq!(original, copied);
    }
}
