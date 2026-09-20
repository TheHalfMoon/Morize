//! Baseline scope classifications for deterministic Morize core contracts.

/// The initial scope classifications defined by Morize's data model.
///
/// This vocabulary is intentionally non-exhaustive because the canonical
/// requirements define these as initial/at-least scope families. A scope kind
/// alone does not define hierarchy, inheritance, visibility, policy, or
/// authorization behavior.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScopeKind {
    /// Run-local scope classification.
    Run,
    /// Session-local scope classification.
    Session,
    /// Agent scope classification.
    Agent,
    /// Project scope classification.
    Project,
    /// User scope classification.
    User,
    /// Team scope classification.
    Team,
    /// Organization scope classification.
    Organization,
    /// Reference scope classification.
    ///
    /// This classification does not itself imply public visibility, anonymous
    /// access, inheritance, or authorization.
    Reference,
}

#[cfg(test)]
mod tests {
    use super::ScopeKind;
    use std::collections::HashSet;

    const ALL_KINDS: [ScopeKind; 8] = [
        ScopeKind::Run,
        ScopeKind::Session,
        ScopeKind::Agent,
        ScopeKind::Project,
        ScopeKind::User,
        ScopeKind::Team,
        ScopeKind::Organization,
        ScopeKind::Reference,
    ];

    #[test]
    fn all_eight_scope_kinds_are_distinct_hashable_values() {
        for (index, kind) in ALL_KINDS.iter().enumerate() {
            for other in ALL_KINDS.iter().skip(index + 1) {
                assert_ne!(kind, other);
            }
        }

        let unique: HashSet<_> = ALL_KINDS.into_iter().collect();
        assert_eq!(unique.len(), 8);
    }

    #[test]
    fn scope_kind_is_copy() {
        let original = ScopeKind::Reference;
        let copied = original;

        assert_eq!(original, copied);
    }
}
