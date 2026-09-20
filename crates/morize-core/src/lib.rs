#![forbid(unsafe_code)]
//! Deterministic core contracts for Morize.

pub mod bounded;
pub mod bounded_collection;
pub mod compatibility;
pub mod decision;
pub mod error;
pub mod identity;
pub mod relation;
pub mod scope;

pub use bounded::{BoundedUtf8, Utf8BoundsError};
pub use bounded_collection::{
    BoundedVec, BoundedVecBuildError, BoundedVecPushError, CollectionBoundsError,
};
pub use compatibility::CompatibilityClass;
pub use decision::MemoryAction;
pub use error::ErrorCategory;
pub use identity::{
    BranchId, ConnectorBindingId, ContextBundleId, DecisionId, EvidenceId, ID_BYTE_LENGTH,
    ID_TEXT_LENGTH, IdParseError, MemoryId, MemoryVersionId, MutationId, ObservationId,
    PolicyRevisionId, PrincipalId, ProjectionGenerationId, PropositionId, RelationId, ScopeId,
    SnapshotId, SourceId, VaultId,
};
pub use relation::MemoryRelationType;
pub use scope::ScopeKind;
