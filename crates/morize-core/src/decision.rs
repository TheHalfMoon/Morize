//! Closed durable-memory action vocabulary for deterministic Morize decisions.
//!
//! This module defines only the current in-memory action vocabulary fixed by
//! docs/TYPED_DECISION_MODEL.md and docs/DATA_MODEL.md. Action-set version
//! binding, parsing, authorization, mutation execution, side effects,
//! serialization, persistence, and transport behavior are intentionally
//! outside this contract.

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

#[cfg(test)]
mod tests {
    use super::MemoryAction;
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
}
