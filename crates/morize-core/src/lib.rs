#![forbid(unsafe_code)]
//! Deterministic core contracts for Morize.

pub mod identity;

pub use identity::{
    BranchId, ConnectorBindingId, ContextBundleId, DecisionId, EvidenceId, ID_BYTE_LENGTH,
    ID_TEXT_LENGTH, IdParseError, MemoryId, MemoryVersionId, MutationId, ObservationId,
    PolicyRevisionId, PrincipalId, ProjectionGenerationId, PropositionId, RelationId, ScopeId,
    SnapshotId, SourceId, VaultId,
};
