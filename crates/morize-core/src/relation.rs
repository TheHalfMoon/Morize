//! Baseline memory-relation classifications for deterministic Morize core contracts.
//!
//! This module defines only the initial in-memory relation vocabulary fixed by
//! docs/DATA_MODEL.md. It does not define relation endpoints, explicit/inferred
//! edge class, evidence sufficiency, truth, authority, graph traversal or
//! mutation, memory mutation, serialization, persistence, or transport behavior.

/// The initial memory-relation type vocabulary.
///
/// This vocabulary is intentionally non-exhaustive because the canonical data
/// model defines these as the initial relation types. A relation type is
/// descriptive classification only and does not itself establish truth,
/// authority, inference status, evidence sufficiency, or any graph or memory
/// mutation.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MemoryRelationType {
    /// The source supports the target.
    Supports,
    /// The source contradicts the target.
    Contradicts,
    /// The source supersedes the target.
    Supersedes,
    /// The source is derived from the target.
    DerivedFrom,
    /// The source depends on the target.
    DependsOn,
    /// The source was caused by the target.
    CausedBy,
    /// The source led to the target.
    LedTo,
    /// The source is a member of the target.
    MemberOf,
    /// The source is about the target.
    About,
    /// The source is related to the target.
    RelatedTo,
    /// The source is a source of the target.
    SourceOf,
}

#[cfg(test)]
mod tests {
    use super::MemoryRelationType;
    use std::collections::HashSet;

    const ALL_RELATION_TYPES: [MemoryRelationType; 11] = [
        MemoryRelationType::Supports,
        MemoryRelationType::Contradicts,
        MemoryRelationType::Supersedes,
        MemoryRelationType::DerivedFrom,
        MemoryRelationType::DependsOn,
        MemoryRelationType::CausedBy,
        MemoryRelationType::LedTo,
        MemoryRelationType::MemberOf,
        MemoryRelationType::About,
        MemoryRelationType::RelatedTo,
        MemoryRelationType::SourceOf,
    ];

    #[test]
    fn all_eleven_relation_types_are_distinct_hashable_values() {
        for (index, relation_type) in ALL_RELATION_TYPES.iter().enumerate() {
            for other in ALL_RELATION_TYPES.iter().skip(index + 1) {
                assert_ne!(relation_type, other);
            }
        }

        let unique: HashSet<_> = ALL_RELATION_TYPES.into_iter().collect();
        assert_eq!(unique.len(), 11);
    }

    #[test]
    fn memory_relation_type_is_copy() {
        let original = MemoryRelationType::RelatedTo;
        let copied = original;

        assert_eq!(original, copied);
    }
}
