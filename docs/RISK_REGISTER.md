# Morize Risk Register

**Planning baseline:** 2026-09-19  
**Status:** living program-level risk register.

Ratings are qualitative planning signals, not statistical probabilities.

| ID | Risk | Likelihood | Impact | Primary controls | Owning phase |
|---|---|---|---|---|---|
| R-001 | Memory poisoning persists malicious instructions across sessions | High | Critical | candidate-before-durable, taint, policy memory class, quarantine, adversarial fixtures | P3/P13 |
| R-002 | Cross-scope retrieval leaks private records or metadata | Medium | Critical | authorize before retrieval, scope-bound indexes, side-channel tests | P3/P12/P13 |
| R-003 | Crash produces file/SQLite/evidence divergence | Medium | Critical | prepared journal, CAS, idempotency, reconciliation, fault injection | P2/P13 |
| R-004 | Human edits are silently overwritten | Medium | High | expected digest/version, commit-time revalidation, reconciliation | P2 |
| R-005 | Schema migration corrupts long-lived memory | Medium | Critical | versioned formats, compatibility fixtures, rehearsal, backup/restore, rollback policy | P2/P13 |
| R-006 | Markdown canonical format is abused to inject authority metadata | Medium | Critical | reserved metadata separation, strict parser, user content vs authority distinction | P2/P3 |
| R-007 | Secrets are captured into durable memory/logs | High | Critical | memory firewall, secret references, safe logging, synthetic leak tests | P3/P13 |
| R-008 | Stale memory continues to steer agents after reality changes | High | High | bi-temporal model, invalidation, supersession, freshness policy | P4/P5 |
| R-009 | Contradictory facts are destructively collapsed | Medium | High | explicit conflict sets, context-aware coexistence, abstention | P4 |
| R-010 | Graph ontology expands without control and becomes inconsistent | Medium | Medium | bounded relation vocabulary, explicit/inferred separation, schema governance | P4 |
| R-011 | Retrieval quality becomes dependent on embeddings/cloud providers | Medium | High | deterministic retrieval baseline, optional vectors, rebuildable projections | P5/P9 |
| R-012 | Context Compiler overfills models or silently drops critical evidence | Medium | High | mandatory/optional source classes, explicit budgets, omission records, sufficiency state | P5 |
| R-013 | MCP/plugin connectivity is mistaken for authorization | High | Critical | authenticated bindings, least-authority tools, server-owned principal mapping | P6 |
| R-014 | API/SDK changes strand integrations | Medium | High | versioned public contracts, compatibility policy, migration/deprecation tests | P6/P7/P13 |
| R-015 | Imports bring forged provenance, paths, or oversized payloads | High | High | bounded schema validation, quarantine, safe extraction, provenance downgrade | P7 |
| R-016 | Donor code imports hidden hosted/telemetry/security assumptions | Medium | High | exact component qualification, source ledger, dependency/security review | All |
| R-017 | Too many donor projects create architectural incoherence | High | High | Morize-owned contracts, selective admission, reject overlap, measured need | P0+ |
| R-018 | Optional local AI gains accidental mutation authority | Medium | Critical | typed outputs, confidence != authority, policy gate, engine identity | P8 |
| R-019 | AI decisions are poorly calibrated and over-automate changes | Medium | High | abstention, threshold evaluation, calibration benchmarks, review routing | P8/P13 |
| R-020 | Vector/graph infrastructure adds operating cost without measurable benefit | Medium | Medium | experiment flow, deterministic baseline, cost/performance comparison, removable adapters | P9 |
| R-021 | Memory branches create confusing or unsafe merge semantics | Medium | High | immutable versions, merge proposals, explicit conflicts, branch provenance | P10 |
| R-022 | Multi-device offline sync loses edits via implicit last-write-wins | High | Critical | operation identity, conflict model, no undocumented LWW, sync-specific Grains | P12 |
| R-023 | Forget/redact does not propagate to offline/derived copies | Medium | Critical | deletion scope contract, tombstones/receipts, derivative invalidation, sync policy | P3/P12 |
| R-024 | Hosted multi-tenant mode leaks through timing/count/index side channels | Medium | Critical | tenant-isolated candidate corpus, generic errors, side-channel tests | P12/P13 |
| R-025 | Founder incurs unplanned recurring infrastructure cost | Medium | High | founder-cost contract, cost ledger, explicit budget owner/cap, local fallback | All |
| R-026 | Future paid fallback silently bills customers or founder | Medium | High | explicit provider selection, cost visibility, no hidden fallback | P7/P8/P12 |
| R-027 | Commercial entitlements corrupt canonical memory semantics | Low | Critical | billing/entitlement layer above semantic core, failure isolation | P12/future cloud |
| R-028 | Billing outage blocks export or damages user state | Medium | High | export independent from billing path, durable service boundaries | future cloud |
| R-029 | Apache-2.0 source reuse misses third-party obligations | Medium | High | NOTICE/third-party register, per-component source record, license scanning | P0/P13 |
| R-030 | Private donor source is published without sufficient rights/cleanup | Low | Critical | explicit public-transfer review, selected paths, secret scan, provenance record | All donor work |
| R-031 | Supply-chain compromise enters dependency/model/release artifacts | Medium | Critical | pins/locks, RustSec/cargo-deny/vet, SBOM, attestations, hashes | P1+/P13 |
| R-032 | CI free-tier limitations tempt weakening required verification | Medium | High | local reproducibility, tiered suites, self-hosted option, revenue-funded CI later | All/P13 |
| R-033 | Performance degrades catastrophically on large vaults/graphs | Medium | High | bounded operations, representative benchmarks, incremental indexes, degradation rules | P5/P13 |
| R-034 | SQLite local architecture leaks into public semantics and blocks scale | Medium | High | repository interfaces, storage abstraction at semantic boundary, no SQL-shaped public API | P2/P12 |
| R-035 | “Memory OS” scope expands into general agent orchestration | High | High | explicit non-goals, SpecGrain scope-out, separate execution systems | All |
| R-036 | Benchmark chasing produces misleading superiority claims | High | Medium | preregistration, multiple suites, preserve failures, exact pins | P13 |
| R-037 | Users misunderstand deletion guarantees for external exports/backups | Medium | High | deletion-scope UX/docs, external-copy caveat, receipts | P3/P11/P12 |
| R-038 | Telemetry collects sensitive memory content | Low | Critical | opt-in/explicit telemetry, metadata minimization, no memory content by default | P11/P12 |
| R-039 | Hosted service launches without retention/residency/incident contracts | Medium | Critical | dedicated hosted-launch SpecGrain before customer data | future cloud |
| R-040 | Maintenance burden exceeds founder capacity | High | High | narrow dependency surface, progressive scope, automation, cost/maintenance review | All |
| R-041 | Public contribution governance creates legal/release ambiguity | Medium | Medium | DCO/CLA decision before scale, maintainer/release authority policy | pre-community scale |
| R-042 | Trademark/name confusion grows after downstream forks | Medium | Medium | trademark-policy decision before brand scale | pre-brand scale |
| R-043 | Release binaries diverge from tested source | Low | Critical | exact-head build, checksums, attestations, reproducible release evidence | P13 |
| R-044 | Unsupported upgrade paths silently corrupt vaults | Medium | Critical | supported-version matrix, migration fixtures, downgrade/rollback rules | P2/P13 |
| R-045 | Recovery logic itself guesses ambiguous state | Low | Critical | fail closed, preserve unknown outcome, independent recovery tests | P2/P13 |

## Risk ownership rule

Every SHAPED SpecGrain that touches one or more register risks must:

- reference the applicable risk IDs;
- state whether likelihood/impact changes;
- define mitigation and residual risk;
- define recovery where change is difficult to reverse;
- include evidence for the mitigation in acceptance.

A new material risk discovered during implementation is added here before the Grain is closed.

## R3 default triggers

The following surfaces default to Diffcipline R3 unless a stronger repository policy supersedes them:

- authorization/tenant boundaries;
- secret handling;
- canonical write/recovery logic;
- migration of existing durable data;
- forget/redact;
- synchronization conflict/deletion behavior;
- cryptography/key management;
- release/update integrity.

## Founder escalation

Any risk that creates:

- uncapped recurring founder spend;
- paid-service lock-in in the required development path;
- customer data liability;
- irreversible license/provenance exposure;
- destructive data migration;
- cross-tenant disclosure;

requires explicit founder-visible planning evidence before implementation authority is granted.
