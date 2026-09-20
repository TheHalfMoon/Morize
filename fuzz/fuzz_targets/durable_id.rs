#![no_main]

use libfuzzer_sys::fuzz_target;
use morize_core::{
    BranchId, ConnectorBindingId, ContextBundleId, DecisionId, EvidenceId,
    ID_TEXT_LENGTH, MemoryId, MemoryVersionId, MutationId, ObservationId,
    PolicyRevisionId, PrincipalId, ProjectionGenerationId, PropositionId, RelationId,
    ScopeId, SnapshotId, SourceId, VaultId,
};

macro_rules! check_identifier {
    ($ty:ty, $text:expr) => {
        if let Ok(id) = $text.parse::<$ty>() {
            let rendered = id.to_string();
            assert_eq!(rendered, $text);
            assert_eq!(rendered.len(), ID_TEXT_LENGTH);
            assert!(rendered
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));

            let bytes = id.into_bytes();
            let rebuilt = <$ty>::from_bytes(bytes);
            assert_eq!(rebuilt.as_bytes(), &bytes);
            assert_eq!(rebuilt.into_bytes(), bytes);
            assert_eq!(rebuilt.to_string(), $text);
        }
    };
}

fuzz_target!(|data: &[u8]| {
    let Ok(text) = core::str::from_utf8(data) else {
        return;
    };

    check_identifier!(VaultId, text);
    check_identifier!(PrincipalId, text);
    check_identifier!(ScopeId, text);
    check_identifier!(SourceId, text);
    check_identifier!(ObservationId, text);
    check_identifier!(EvidenceId, text);
    check_identifier!(PropositionId, text);
    check_identifier!(MemoryId, text);
    check_identifier!(MemoryVersionId, text);
    check_identifier!(RelationId, text);
    check_identifier!(DecisionId, text);
    check_identifier!(MutationId, text);
    check_identifier!(PolicyRevisionId, text);
    check_identifier!(BranchId, text);
    check_identifier!(SnapshotId, text);
    check_identifier!(ProjectionGenerationId, text);
    check_identifier!(ContextBundleId, text);
    check_identifier!(ConnectorBindingId, text);
});
