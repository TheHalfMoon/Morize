# Morize Canonical Data Model

## 1. Purpose

This document turns the architecture's conceptual memory objects into an implementation contract that can be refined by SpecGrain.

The model separates:

- source evidence;
- normalized propositions;
- logical memories;
- immutable versions;
- relationships;
- policy/decision state;
- projections;
- delivery context.

No vector, graph database, or model-provider object is canonical truth.

## 2. Identity rules

Every durable identity is typed and non-interchangeable.

Planned identity classes include:

- `VaultId`
- `PrincipalId`
- `ScopeId`
- `SourceId`
- `ObservationId`
- `EvidenceId`
- `PropositionId`
- `MemoryId`
- `MemoryVersionId`
- `RelationId`
- `DecisionId`
- `MutationId`
- `PolicyRevisionId`
- `BranchId`
- `SnapshotId`
- `ProjectionGenerationId`
- `ContextBundleId`
- `ConnectorBindingId`

Externally supplied strings do not become internal identities merely by occupying an ID field.

IDs must have:

- canonical serialization;
- explicit maximum length;
- stable equality semantics;
- no implicit cross-type conversion.

## 3. Content identity

Where exact content identity matters, use a cryptographic digest over canonical bytes.

A digest identifies content; it does not by itself identify:

- authority;
- principal;
- time;
- scope;
- operation occurrence.

Repeated operations over identical content require distinct operation/attempt identity.

## 4. Principal

`Principal` represents an authenticated or explicitly local actor.

Conceptual fields:

```text
principal_id
principal_type
display_name
status
created_at
revoked_at?
external_bindings[]
```

Principal types may include:

- local user;
- agent;
- service;
- team member;
- organization service account;
- connector.

Model output never supplies authoritative principal identity.

## 5. Scope

A `Scope` defines where a memory belongs and who may reason about it.

Initial scope kinds:

```text
run
session
agent
project
user
team
organization
reference
```

A scope has stable identity and policy attachment. Scope inheritance/visibility must be explicit; it is not inferred from string prefixes.

## 6. SourceDescriptor

Represents an origin from which observations can be made.

Conceptual fields:

```text
source_id
source_kind
canonical_locator
source_revision
observed_digest?
owner_scope
trust_class
sensitivity_hint
connector_binding_id?
created_at
revoked_at?
```

Examples:

- repository file at a commit;
- local document;
- MCP resource;
- web resource;
- agent message;
- human statement;
- database record;
- imported archive entry.

A source record is descriptive. Trust class is policy-owned metadata.

## 7. Observation

An `Observation` is one bounded event where Morize saw source content.

Conceptual fields:

```text
observation_id
source_id
source_revision
observed_at
content_digest
span_or_selector?
raw_artifact_ref?
observer_principal_id
scope_id
trust_class
taint_class
sensitivity
ingestion_policy_revision
```

Observations are evidence inputs, not automatically durable memory.

## 8. EvidenceArtifact

An `EvidenceArtifact` binds inspectable support material.

Examples:

- exact text span;
- file/blob digest;
- image/document region;
- structured tool result;
- imported record;
- deterministic parser output.

Conceptual fields:

```text
evidence_id
observation_id
artifact_kind
content_or_blob_ref
selector
digest
created_at
transformation_lineage[]
```

Derived evidence retains lineage to its parents.

## 9. Proposition

A `Proposition` is a normalized claim candidate.

Conceptual fields:

```text
proposition_id
subject
predicate
object_or_value
qualifiers
scope_id
valid_from?
valid_to?
confidence?
evidence_refs[]
created_by_engine
created_at
```

A proposition may remain uncommitted, disputed, quarantined, or historical.

Confidence is descriptive and never grants write/read authority.

## 10. MemoryRecord

A `MemoryRecord` is the stable logical identity of one governed memory lineage.

It does not contain mutable truth directly.

Conceptual fields:

```text
memory_id
memory_kind
owner_scope_id
created_at
created_by
active_branch
status
```

Current content is obtained through an active `MemoryVersion` projection.

## 11. MemoryVersion

Every committed durable change creates an immutable `MemoryVersion`.

Conceptual fields:

```text
memory_version_id
memory_id
version_number_or_order
proposition_ref
canonical_content
valid_from?
valid_to?
observed_at
committed_at
status
sensitivity
source_authority_class
provenance_refs[]
support_refs[]
contradiction_refs[]
predecessor_refs[]
decision_id
mutation_id
policy_revision_id
creator_principal_id
writer_identity
content_digest
```

A version is never rewritten to hide a prior value.

## 12. MemoryRelation

Relations connect durable semantic objects.

Initial relation vocabulary:

```text
SUPPORTS
CONTRADICTS
SUPERSEDES
DERIVED_FROM
DEPENDS_ON
CAUSED_BY
LED_TO
MEMBER_OF
ABOUT
RELATED_TO
SOURCE_OF
```

Every relation has:

```text
relation_id
relation_type
from_ref
to_ref
edge_class = explicit | inferred
evidence_refs[]
engine_identity?
confidence?
created_at
policy_revision_id
```

High-impact inferred relationships require policy-defined evidence thresholds.

## 13. DecisionEnvelope

A `DecisionEnvelope` is a bounded analysis result, not mutation authority.

Conceptual fields:

```text
decision_id
candidate_ref
allowed_action_set_version
selected_action
action_scores?
confidence?
abstained
reason_codes[]
evidence_refs[]
relevant_memory_versions[]
decision_engine_identity
policy_revision_id
created_at
```

Allowed actions are versioned and closed.

Initial set:

```text
STORE
UPDATE
MERGE
SUPERSEDE
CONTRADICT
EXPIRE
FORGET
REDACT
QUARANTINE
IGNORE
REQUIRE_REVIEW
```

## 14. MutationIntent

A `MutationIntent` is the immutable request to perform one consequential memory change.

Conceptual fields:

```text
mutation_id
decision_id
principal_id
scope_id
action
target_memory_id?
expected_versions[]
expected_digests[]
new_content_digest?
policy_revision_id
authorization_ref
idempotency_key
prepared_at?
terminal_status?
terminal_evidence_ref?
```

After PREPARED, material target/action/precondition changes require a new mutation identity.

## 15. Mutation terminal states

Planned terminal semantics:

- `COMMITTED`
- `REJECTED`
- `FAILED`
- `UNKNOWN_OUTCOME`

Unknown outcome is not success and blocks dependent mutations until reconciliation.

## 16. PolicyRevision

Every authority-sensitive result binds a stable policy revision.

Conceptual fields:

```text
policy_revision_id
policy_bundle_digest
effective_scope
created_at
supersedes?
```

Policy source may be human-managed files, organization configuration, or hosted control-plane state in future profiles. The evaluated revision is what matters to evidence.

## 17. Branch

A `Branch` selects an alternative active projection over immutable history.

Conceptual fields:

```text
branch_id
scope_id
name
base_snapshot_id
created_at
created_by
status
```

Branch operations do not erase evidence history.

## 18. Snapshot

A `Snapshot` binds an exact projection frontier.

```text
snapshot_id
branch_id
created_at
memory_version_refs[]
policy_revision_id
digest
```

Implementation may optimize representation; semantic identity remains stable.

## 19. ProjectionGeneration

Derived indexes are generation-bound.

Examples:

- FTS;
- graph adjacency/cache;
- semantic vectors;
- summaries;
- reranker artifacts.

Conceptual fields:

```text
projection_generation_id
projection_kind
source_frontier_digest
builder_identity
configuration_digest
created_at
status
```

Projection loss never implies canonical-memory loss.

## 20. ContextBundle

A `ContextBundle` is the exact bounded context delivered to a client invocation.

Conceptual fields:

```text
context_bundle_id
requesting_principal_id
target_agent_or_app
scope_ids[]
intent_digest
policy_revision_id
selected_items[]
omitted_items_summary
budget
created_at
bundle_digest
```

Each selected item includes:

- exact source/memory version reference;
- selection reason;
- retrieval signals;
- freshness;
- trust/sensitivity labels allowed for the caller;
- byte/token estimate.

## 21. ConnectorBinding

A connector's advertised tools/resources are separate from its trusted binding.

Conceptual fields:

```text
connector_binding_id
connector_type
instance_identity
configuration_digest
principal_mapping
allowed_scopes
network_policy
secret_handles
status
created_at
revoked_at?
```

Revocation invalidates cached authority derived from the binding.

## 22. Temporal invariants

Morize is bi-temporal.

For every durable proposition/version where applicable:

- valid time answers “when was this true in the domain?”;
- knowledge/transaction time answers “when did Morize know/commit this?”.

Open intervals must have canonical representation.

The query API must define:

- now/current;
- valid-as-of;
- known-as-of;
- combined bitemporal query;
- full lineage/history.

## 23. Conflict invariants

A conflict is not merely “two different strings.”

Conflict detection considers:

- subject/entity identity;
- predicate;
- scope;
- qualifiers;
- overlapping valid time;
- source authority;
- evidence sufficiency.

The system may abstain and preserve both propositions.

## 24. Deletion/redaction invariants

FORGET and REDACT must define effects on:

- active projection;
- canonical plaintext;
- operational metadata;
- evidence references;
- derived indexes;
- backups;
- synchronized peers;
- exported/external copies.

Morize must not claim deletion from systems it does not control.

## 25. Serialization invariants

Canonical formats require:

- UTF-8;
- deterministic field rules where digests depend on serialization;
- explicit schema/version identifiers;
- bounded field sizes;
- duplicate handling rules;
- unknown-field/version policy;
- no executable semantics from untrusted metadata.

## 26. Storage abstraction rule

SQLite/Markdown are the initial local implementation choices.

Public semantic contracts must not force future hosted/team backends to expose SQLite-specific behavior.

The abstraction boundary is the Morize domain model and versioned repository interfaces, not raw SQL tables.

## 27. Implementation rule

This document is planning authority, not code schema.

The first data-model SpecGrains must refine:

- concrete Rust types;
- exact serialized field names;
- maximum sizes/counts;
- ID formats;
- canonicalization;
- timestamp representation;
- error taxonomy;
- compatibility behavior;
- property/fuzz test oracles.

No implementation may invent incompatible persistent semantics without revising this contract.
