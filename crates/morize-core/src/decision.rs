//! Closed durable-memory action vocabulary and known decision-reason labels.
//!
//! This module defines the current in-memory action vocabulary, its in-memory
//! compatibility-version identity, and the currently documented known reason
//! labels fixed by docs/TYPED_DECISION_MODEL.md and docs/DATA_MODEL.md. Parsing,
//! authorization, mutation execution, side effects, serialization, persistence,
//! and transport behavior are intentionally outside this contract.

/// The closed initial durable-memory action vocabulary.
///
/// A value classifies a structured decision only. It does not grant
/// authorization, select a target, or execute any mutation or side effect.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryAction {
    /// Store a new governed memory.
    Store,
    /// Update an existing governed memory lineage.
    Update,
    /// Merge compatible memory state.
    Merge,
    /// Supersede prior memory state while preserving history.
    Supersede,
    /// Record an explicit contradiction.
    Contradict,
    /// Mark governed memory as expired under later policy.
    Expire,
    /// Represent a requested forget action without executing it.
    Forget,
    /// Represent a requested redact action without executing it.
    Redact,
    /// Route material into quarantine.
    Quarantine,
    /// Take no durable-memory action.
    Ignore,
    /// Require review before any later authorized mutation.
    RequireReview,
}

/// The in-memory compatibility version for the durable-memory action vocabulary.
///
/// A value identifies which closed `MemoryAction` vocabulary a decision uses.
/// It is descriptive compatibility metadata only and does not grant
/// authorization, select a target, or execute any mutation or side effect.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryActionSetVersion {
    /// Version 1 identifies exactly `Store`, `Update`, `Merge`, `Supersede`,
    /// `Contradict`, `Expire`, `Forget`, `Redact`, `Quarantine`, `Ignore`,
    /// and `RequireReview`.
    V1,
}

/// A currently documented known reason label for structured decision analysis.
///
/// These variants are descriptive in-memory evidence only. The enum is
/// intentionally non-exhaustive: the known labels below do not freeze a
/// complete future reason-code vocabulary, stable textual spelling, numeric
/// representation, parser, serialization, persistence, authorization rule, or
/// execution behavior.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DecisionReasonCode {
    /// The candidate is an exact duplicate of governed memory.
    ExactDuplicate,
    /// A candidate originates from a newer source.
    NewerSource,
    /// The candidate changes valid-time information.
    ValidTimeChanged,
    /// Available sources conflict.
    SourceConflict,
    /// Source authority is too low for the contemplated decision.
    LowSourceAuthority,
    /// Relevant evidence conflicts across scopes.
    CrossScopeConflict,
    /// A source is stale for the contemplated decision.
    StaleSource,
    /// The decision reflects an explicit user operation.
    ExplicitUserOperation,
    /// Available evidence is insufficient.
    InsufficientEvidence,
    /// Entity resolution remains ambiguous.
    AmbiguousEntity,
    /// Policy review is required before later authorized action.
    PolicyReviewRequired,
}

#[cfg(test)]
mod tests {
    use super::{DecisionReasonCode, MemoryAction, MemoryActionSetVersion};
    use std::collections::HashSet;

    const ALL_ACTIONS: [MemoryAction; 11] = [
        MemoryAction::Store,
        MemoryAction::Update,
        MemoryAction::Merge,
        MemoryAction::Supersede,
        MemoryAction::Contradict,
        MemoryAction::Expire,
        MemoryAction::Forget,
        MemoryAction::Redact,
        MemoryAction::Quarantine,
        MemoryAction::Ignore,
        MemoryAction::RequireReview,
    ];

    const ALL_KNOWN_REASON_CODES: [DecisionReasonCode; 11] = [
        DecisionReasonCode::ExactDuplicate,
        DecisionReasonCode::NewerSource,
        DecisionReasonCode::ValidTimeChanged,
        DecisionReasonCode::SourceConflict,
        DecisionReasonCode::LowSourceAuthority,
        DecisionReasonCode::CrossScopeConflict,
        DecisionReasonCode::StaleSource,
        DecisionReasonCode::ExplicitUserOperation,
        DecisionReasonCode::InsufficientEvidence,
        DecisionReasonCode::AmbiguousEntity,
        DecisionReasonCode::PolicyReviewRequired,
    ];

    fn assert_action_set_version_traits<T>()
    where
        T: Clone + Copy + std::fmt::Debug + Eq + std::hash::Hash,
    {
    }

    fn assert_decision_reason_code_traits<T>()
    where
        T: Clone + Copy + std::fmt::Debug + Eq + std::hash::Hash,
    {
    }

    #[test]
    fn all_eleven_actions_are_distinct_hashable_values() {
        for (index, action) in ALL_ACTIONS.iter().enumerate() {
            for other in ALL_ACTIONS.iter().skip(index + 1) {
                assert_ne!(action, other);
            }
        }

        let unique: HashSet<_> = ALL_ACTIONS.into_iter().collect();
        assert_eq!(unique.len(), 11);
    }

    #[test]
    fn memory_action_is_copy() {
        let original = MemoryAction::RequireReview;
        let copied = original;

        assert_eq!(original, copied);
    }

    #[test]
    fn memory_action_set_v1_has_in_memory_value_semantics() {
        assert_action_set_version_traits::<MemoryActionSetVersion>();

        let original = MemoryActionSetVersion::V1;
        let copied = original;
        let unique: HashSet<_> = [original, copied].into_iter().collect();

        assert_eq!(original, copied);
        assert_eq!(unique.len(), 1);
    }

    #[test]
    fn memory_action_set_v1_identifies_exact_current_vocabulary() {
        let version = MemoryActionSetVersion::V1;

        match version {
            MemoryActionSetVersion::V1 => assert_eq!(
                ALL_ACTIONS,
                [
                    MemoryAction::Store,
                    MemoryAction::Update,
                    MemoryAction::Merge,
                    MemoryAction::Supersede,
                    MemoryAction::Contradict,
                    MemoryAction::Expire,
                    MemoryAction::Forget,
                    MemoryAction::Redact,
                    MemoryAction::Quarantine,
                    MemoryAction::Ignore,
                    MemoryAction::RequireReview,
                ]
            ),
        }
    }

    #[test]
    fn all_eleven_known_reason_codes_are_distinct_hashable_values() {
        assert_decision_reason_code_traits::<DecisionReasonCode>();

        for (index, reason) in ALL_KNOWN_REASON_CODES.iter().enumerate() {
            for other in ALL_KNOWN_REASON_CODES.iter().skip(index + 1) {
                assert_ne!(reason, other);
            }
        }

        let unique: HashSet<_> = ALL_KNOWN_REASON_CODES.into_iter().collect();
        assert_eq!(unique.len(), 11);
    }

    #[test]
    fn decision_reason_code_is_copy() {
        let original = DecisionReasonCode::PolicyReviewRequired;
        let copied = original;
        let unique: HashSet<_> = [original, copied].into_iter().collect();

        assert_eq!(original, copied);
        assert_eq!(unique.len(), 1);
    }
}
