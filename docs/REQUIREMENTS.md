# Morize v1 Requirements Baseline

## Purpose

This document closes category-level planning gaps before implementation. It is not a substitute for SpecGrain refinement: each requirement must eventually map to one or more bounded Grains and exact verification evidence.

Requirement IDs are stable planning references.

## A. Product and ownership

### MZ-PROD-001 — Memory OS category

Morize shall provide a governed memory substrate for agents, applications, and humans rather than operating only as a vector database or transcript store.

### MZ-PROD-002 — Open-source core

The repository shall remain licensed under Apache License 2.0 unless an explicit governance change replaces that decision.

### MZ-PROD-003 — Commercial services allowed

The architecture shall permit future paid Morize-managed services without redefining canonical memory semantics.

### MZ-PROD-004 — Founder-zero-burn development

Pre-revenue development and ordinary dogfooding shall not require a recurring founder-paid service.

### MZ-PROD-005 — User pricing not frozen

The technical architecture shall not encode a permanent assumption that users receive hosted/managed services for free.

### MZ-PROD-006 — User-owned export

Users shall be able to export canonical memory and supported provenance in documented formats.

## B. Canonical data model

### MZ-DATA-001 — Stable identities

Every durable source, observation, proposition, memory, memory version, relation, decision, mutation, policy revision, context bundle, branch, and projection generation shall have stable typed identity.

### MZ-DATA-002 — Immutable memory versions

A committed memory version shall not be rewritten in place.

### MZ-DATA-003 — Human-inspectable canonical form

The local canonical vault shall have a documented human-inspectable representation.

### MZ-DATA-004 — Operational metadata separation

Operational coordination/index metadata shall not silently replace canonical knowledge content.

### MZ-DATA-005 — Content-addressed artifacts

Large immutable source artifacts should use digest-bound references rather than uncontrolled duplication.

### MZ-DATA-006 — Schema versioning

Every persisted public/canonical format shall carry an explicit version with compatibility rules.

### MZ-DATA-007 — Migration determinism

Schema/data migrations shall have deterministic preconditions, postconditions, and recovery/rollback behavior.

### MZ-DATA-008 — Unknown fields/version behavior

Readers shall define fail/ignore/preserve behavior for unsupported versions and unknown fields rather than guessing.

## C. Temporal truth and conflicts

### MZ-TIME-001 — Valid time

Morize shall distinguish when a proposition was valid in its domain.

### MZ-TIME-002 — Knowledge/transaction time

Morize shall distinguish when the system observed/committed a proposition.

### MZ-TIME-003 — Current and historical query

The query model shall support current truth and as-of/history semantics.

### MZ-TIME-004 — Supersession

A newer proposition may supersede an older proposition without deleting historical evidence.

### MZ-TIME-005 — Contradiction

Conflicting propositions shall remain explicit when the system cannot resolve them safely.

### MZ-TIME-006 — Contextual coexistence

Different scopes/contexts may legitimately contain different propositions without being collapsed as contradictions.

## D. Provenance and evidence

### MZ-PROV-001 — Source-bound derived memory

Machine-derived durable memory shall retain source/evidence references.

### MZ-PROV-002 — Transformation lineage

Summaries, extractions, embeddings, graph edges, and classifications shall record lineage sufficient to invalidate/rebuild derivatives.

### MZ-PROV-003 — Trust is separate from relevance

Retrieval rank shall not define source authority.

### MZ-PROV-004 — Model output is not verified fact

Model-produced content remains candidate/derived evidence until deterministic policy accepts an allowed durable action.

### MZ-PROV-005 — Explainable relation

Inferred graph relationships shall identify their inference/evidence class.

## E. Memory lifecycle

### MZ-LIFE-001 — Closed mutation vocabulary

Durable lifecycle actions shall use a versioned closed vocabulary.

### MZ-LIFE-002 — Candidate before durable

Untrusted/model-derived candidate content shall not bypass validation and authorization.

### MZ-LIFE-003 — Review/quarantine

Morize shall represent review-required and quarantined states.

### MZ-LIFE-004 — Expiry

Policies may expire active memory without rewriting historical evidence.

### MZ-LIFE-005 — Forget

Forget semantics shall define exactly which active/canonical/derived surfaces are affected.

### MZ-LIFE-006 — Redact

Redaction shall remove prohibited plaintext from applicable active/canonical/derived surfaces while retaining only permitted audit metadata.

### MZ-LIFE-007 — External user edit

Local human edits shall be reconciled through version/digest checks rather than silently overwritten.

## F. Persistence and reliability

### MZ-REL-001 — Single governed writer

The local canonical vault shall have a defined write-serialization model.

### MZ-REL-002 — Compare-and-swap

Durable updates shall bind expected identities/versions/digests.

### MZ-REL-003 — Idempotency

Consequential mutations shall support stable idempotency/retry semantics.

### MZ-REL-004 — Crash recovery

Crashes at documented mutation boundaries shall converge through deterministic reconciliation.

### MZ-REL-005 — Unknown outcome

Ambiguous partial completion shall not be reported as success.

### MZ-REL-006 — Backup/restore

Backup and restore shall be a release-qualified capability.

### MZ-REL-007 — Corruption detection

Morize shall detect invalid/corrupt canonical and operational state rather than silently normalizing it.

### MZ-REL-008 — Projection rebuild

Loss of FTS/vector/graph/cache projections shall not destroy canonical memory.

### MZ-REL-009 — Disk/resource failure

Write paths shall define behavior for disk full, permission failure, cancellation, and process death.

## G. Identity, scope, and authorization

### MZ-AUTH-001 — Principal identity

Every protected request shall bind an authenticated or explicitly local principal identity.

### MZ-AUTH-002 — Scope hierarchy

Morize shall support at least run/session, agent, project, user, team, organization, and reference/public scopes.

### MZ-AUTH-003 — Authorization before disclosure

Protected records shall be filtered for authority before retrieval metadata/content is disclosed.

### MZ-AUTH-004 — Integration identity

MCP/plugin/connector advertised capability shall not itself grant Morize authority.

### MZ-AUTH-005 — Policy revision binding

Protected decisions shall bind the policy revision used.

### MZ-AUTH-006 — Revocation

Revoked/stale integration or principal bindings shall invalidate cached authority.

## H. Privacy and security

### MZ-SEC-001 — Memory poisoning defense

Untrusted documents/web/tool output shall not self-promote into trusted durable instructions/policy.

### MZ-SEC-002 — Persistent prompt-injection isolation

Instruction-like untrusted content shall remain distinguishable from trusted policy/context.

### MZ-SEC-003 — Secret handling

Credentials and secrets shall not become ordinary plaintext memory by default.

### MZ-SEC-004 — Sensitive-data classification

The policy model shall support sensitivity classifications and safe logging.

### MZ-SEC-005 — Path safety

Filesystem operations shall defend against traversal, symlink/reparse/junction ambiguity, special files, and unsafe extraction.

### MZ-SEC-006 — Bounded inputs

Externally controlled sizes, counts, graph depth, retries, queue lengths, and processing budgets shall be finite.

### MZ-SEC-007 — Supply chain

Dependencies, donor source, models, binaries, installers, and release artifacts shall have provenance/security review appropriate to risk.

### MZ-SEC-008 — Tenant isolation

Multi-user/team mode shall include explicit tenant-isolation and side-channel testing.

### MZ-SEC-009 — Security reporting

The project shall maintain a private vulnerability reporting path.

### MZ-SEC-010 — At-rest encryption profile

Morize v1 shall define and qualify an encrypted-at-rest local vault profile for users who require protection of canonical memory, operational state, and blobs on disk. Raw encryption keys shall not be stored alongside encrypted vault data, and key lifecycle/recovery semantics shall be explicit.

The existence of a plaintext developer/local profile shall not be presented as equivalent protection.

## I. Retrieval and context

### MZ-RET-001 — Deterministic baseline

Exact, metadata, FTS/BM25, temporal, and bounded graph retrieval shall work without embeddings.

### MZ-RET-002 — Optional semantic retrieval

Vector/semantic retrieval may be added as a derivative adapter.

### MZ-RET-003 — Explainable recall

Returned context shall expose applicable selection signals and source/version references.

### MZ-RET-004 — Context budget

Context assembly shall use explicit byte/token/content budgets and record truncation/omissions.

### MZ-RET-005 — Staleness

Stale source/index/summary state shall be invalidated, excluded, or visibly marked according to policy.

### MZ-RET-006 — Sufficiency/abstention

The Context Compiler shall represent insufficient evidence rather than forcing an answer.

### MZ-RET-007 — Ranking composition

Hybrid ranking signals shall be versioned and inspectable.

## J. Interfaces and interoperability

### MZ-API-001 — Stable local API

Public local APIs shall have explicit versioning and compatibility policy.

### MZ-API-002 — CLI

Morize shall provide an inspectable CLI for core lifecycle, retrieval, maintenance, export/import, health, and diagnostics.

### MZ-API-003 — MCP least authority

MCP shall expose bounded read/search/explain/proposal surfaces without conflating connectivity with administrative authority.

### MZ-API-004 — SDKs

Rust is the reference implementation surface; Python/TypeScript SDKs shall preserve the same contract semantics.

### MZ-API-005 — Migration/import

Imports from supported systems shall be schema-validated, bounded, and provenance-aware.

### MZ-API-006 — Export portability

Export shall not require a Morize-hosted service.

### MZ-API-007 — Backward compatibility

Breaking API/persistence changes require explicit major-version or migration policy.

## K. Integrations and extension model

### MZ-EXT-001 — Connector boundary

Connectors shall have explicit identity, data boundary, secrets, network, rate, and revocation behavior.

### MZ-EXT-002 — Provider adapters

Model/vector/graph/storage providers shall be replaceable adapters where practical.

### MZ-EXT-003 — Plugin trust

Extension discovery/installability shall not imply runtime authority.

### MZ-EXT-004 — Project identity

Coding-agent integrations shall derive stable project identity from repository/workspace identity rather than transient paths alone.

### MZ-EXT-005 — Connector failure

Connector failure shall degrade explicitly without corrupting canonical memory.

## L. Local intelligence and models

### MZ-AI-001 — Optionality

Model inference shall not be required for canonical storage/recovery/policy semantics.

### MZ-AI-002 — Typed output

Model-assisted decisions shall resolve to bounded schemas.

### MZ-AI-003 — Confidence is not authority

Confidence may route review but shall not bypass access or mutation policy.

### MZ-AI-004 — Engine identity

Model artifact/provider/revision/runtime/configuration shall be recorded when inference materially affects durable state.

### MZ-AI-005 — Evaluation before promotion

New extraction/ranking/decision approaches require preregistered evidence appropriate to their impact.

## M. Branches, sync, and collaboration

### MZ-COLLAB-001 — Logical branches/snapshots

Alternative active memory projections shall not rewrite evidence history.

### MZ-COLLAB-002 — Merge proposal

Branch merging shall identify conflicts and require policy-appropriate review.

### MZ-COLLAB-003 — Sync identity

Multi-device/team sync shall bind durable operation/version identities.

### MZ-COLLAB-004 — Offline conflict

Offline concurrent edits shall have an explicit conflict model; last-write-wins shall not be an undocumented default.

### MZ-COLLAB-005 — Deletion propagation

Forget/redact synchronization shall define propagation and unresolved-device behavior.

## N. Product UX

### MZ-UX-001 — Memory Inspector

Users shall be able to inspect records, versions, provenance, timelines, relations, conflicts, and retrieval explanations.

### MZ-UX-002 — Review queue

Quarantined/review-required candidates shall have a human management surface.

### MZ-UX-003 — Safe destructive UX

Forget/redact/reset/import-overwrite actions shall communicate scope and consequences.

### MZ-UX-004 — Accessibility

Supported first-party UI shall include keyboard navigation and baseline accessibility.

### MZ-UX-005 — Diagnostics

Users shall have a health/doctor surface that explains broken indexes, migrations, connectors, and recovery states.

## O. Operations and observability

### MZ-OPS-001 — Structured events

Operationally relevant actions shall have structured, bounded, privacy-aware events.

### MZ-OPS-002 — Local diagnostics

Core diagnostics shall remain available without hosted telemetry.

### MZ-OPS-003 — Telemetry consent

Product analytics/telemetry shall be explicit and documented; sensitive memory contents shall not be collected for analytics by default.

### MZ-OPS-004 — Performance budgets

Representative cold start, query, mutation, rebuild, memory, disk, and large-vault behavior shall be measured.

### MZ-OPS-005 — Resource degradation

Resource pressure shall produce bounded partial/failure behavior rather than uncontrolled growth.

### MZ-OPS-006 — Supportability

Errors shall expose stable categories and actionable diagnostics without leaking protected data.

## P. Hosted/commercial readiness

### MZ-COM-001 — Service boundary

Managed Morize services shall sit behind explicit APIs and shall not redefine canonical data semantics.

### MZ-COM-002 — Usage accounting

Any future usage-based service shall have auditable metering, quotas, and cost caps.

### MZ-COM-003 — Entitlements

Future plan/feature entitlements shall be a service/product layer, not encoded into canonical memory meaning.

### MZ-COM-004 — Billing failure

Billing/entitlement outages shall not corrupt user memory.

### MZ-COM-005 — Data portability

Commercial service cancellation shall preserve documented export/migration paths subject to published retention policy.

### MZ-COM-006 — Founder cost ownership

Every founder-operated paid dependency shall have an owner, budget, revenue/funding source, and exit plan.

## Q. Compliance and data lifecycle

### MZ-LIFELEGAL-001 — Retention policy

Hosted/team deployments shall support explicit retention configuration where required.

### MZ-LIFELEGAL-002 — Deletion semantics

Product documentation shall distinguish deletion of active Morize state from copies outside Morize control.

### MZ-LIFELEGAL-003 — Audit minimization

Audit evidence shall not retain prohibited plaintext merely to preserve history.

### MZ-LIFELEGAL-004 — Domain boundaries

Morize core shall not claim domain-specific medical/legal/financial compliance merely because it supports secure memory primitives.

### MZ-LIFELEGAL-005 — Data residency adapters

Hosted deployment design shall keep residency and provider placement explicit rather than implied.

## R. Release, distribution, and sustainability

### MZ-RELSE-001 — Cross-platform core

v1 core shall support Windows, macOS, and Linux.

### MZ-RELSE-002 — Reproducible release evidence

Release claims shall bind exact source, dependency, benchmark, and build identities.

### MZ-RELSE-003 — SBOM and notices

Releases shall include applicable dependency/source notices and an SBOM or equivalent machine-readable inventory.

### MZ-RELSE-004 — Artifact integrity

Release artifacts shall have checksums and provenance/attestation appropriate to supported distribution channels.

### MZ-RELSE-005 — Upgrade proof

Supported upgrade paths shall be tested from declared prior versions.

### MZ-RELSE-006 — Support policy

Released versions shall document supported platforms, compatibility guarantees, and security-support expectations.

### MZ-RELSE-007 — Documentation parity

Examples/docs shall not claim behavior absent from the released source.

### MZ-RELSE-008 — Project sustainability

Release planning shall include founder-operated recurring cost review and maintenance burden.

## S. Engineering governance

### MZ-GOV-001 — SpecGrain authority

Implementation work shall originate from dependency-eligible bounded Grains rather than roadmap prose alone.

### MZ-GOV-002 — Diffcipline proof

Implementation completion shall require exact-diff and executed-verification evidence appropriate to risk.

### MZ-GOV-003 — NOT RUN is not PASS

Configured required verification that did not run shall never be represented as passed.

### MZ-GOV-004 — Donor admission

Copied/adapted code shall have exact source provenance, obligations, selected paths, and independent Morize tests.

### MZ-GOV-005 — Architecture change control

Material changes to persistence, identity, trust, licensing, commercial boundary, or required infrastructure shall update planning authority before implementation.

### MZ-GOV-006 — Negative evidence

Failed benchmark/security/performance evidence shall not be silently discarded to support marketing claims.

## Requirement closure rule

A requirement is not “done” because it appears in this document.

Closure requires:

1. mapping to a SpecGrain revision;
2. bounded acceptance criteria;
3. implementation at an exact revision;
4. required Diffcipline/verification evidence;
5. semantic acceptance where deterministic commands are insufficient;
6. documentation matching implemented truth.

If implementation uncovers a category not covered here, create a new requirement and planning revision rather than hiding it inside a Grain.
