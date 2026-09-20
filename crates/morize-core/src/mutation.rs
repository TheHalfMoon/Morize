//! Baseline mutation terminal-state classifications for deterministic Morize core contracts.
//!
//! This module defines only the planned terminal-state vocabulary fixed by
//! docs/DATA_MODEL.md. It does not define mutation records, state transitions,
//! dependent-mutation blocking, reconciliation execution, retry/idempotency,
//! compare-and-swap, serialization, persistence, or transport behavior.

/// The current baseline mutation terminal-state vocabulary.
///
/// This vocabulary is intentionally non-exhaustive because the canonical data
/// model describes these as planned terminal semantics rather than a permanently
/// closed public vocabulary.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MutationTerminalState {
    /// The mutation is known to have committed.
    Committed,
    /// The mutation was rejected.
    Rejected,
    /// The mutation failed.
    Failed,
    /// Completion is ambiguous and must not be treated as success.
    ///
    /// Reconciliation and dependent-mutation blocking are intentionally outside
    /// this value type.
    UnknownOutcome,
}

#[cfg(test)]
mod tests {
    use super::MutationTerminalState;
    use std::collections::HashSet;

    const ALL_CURRENT_TERMINAL_STATES: [MutationTerminalState; 4] = [
        MutationTerminalState::Committed,
        MutationTerminalState::Rejected,
        MutationTerminalState::Failed,
        MutationTerminalState::UnknownOutcome,
    ];

    #[test]
    fn all_four_current_terminal_states_are_distinct_hashable_values() {
        for (index, state) in ALL_CURRENT_TERMINAL_STATES.iter().enumerate() {
            for other in ALL_CURRENT_TERMINAL_STATES.iter().skip(index + 1) {
                assert_ne!(state, other);
            }
        }

        let unique: HashSet<_> = ALL_CURRENT_TERMINAL_STATES.into_iter().collect();
        assert_eq!(unique.len(), 4);
    }

    #[test]
    fn mutation_terminal_state_is_copy() {
        let original = MutationTerminalState::UnknownOutcome;
        let copied = original;

        assert_eq!(original, copied);
    }
}
