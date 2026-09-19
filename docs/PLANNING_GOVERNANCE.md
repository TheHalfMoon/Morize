# Morize Planning and Delivery Governance

## 1. Purpose

Morize uses **SpecGrain** to control decomposition and execution readiness, and **Diffcipline** to control the finish line.

The two systems have different jobs:

```text
Product intent
  -> SpecGrain: refine, bound, order, budget, define acceptance
  -> WorkPacket: immutable bounded execution context
  -> Human / coding agent: implement
  -> Diffcipline: inspect exact diff, policy, risk, and executed verification
  -> Independent acceptance/evidence review
  -> Canonical merge
```

Neither an agent's self-report nor a green command by itself establishes project completion.

## 2. Canonical authority order

When sources disagree, use this order:

1. live GitHub/repository truth;
2. `AGENTS.md`;
3. `.specgrain/` canonical project/spec state;
4. accepted architecture decisions and requirements;
5. `docs/ARCHITECTURE.md`, `docs/THREAT_MODEL.md`, and the founder-cost/commercial boundary;
6. active roadmap packet and Grain contract;
7. donor/reference research;
8. stale summaries or historical chat context.

No prose document can override a newer exact repository fact without an explicit planning change.

## 3. SpecGrain is the delivery decomposition system

The repository contains a canonical SpecGrain project under `.specgrain/`.

The current root is `SG-000001`. Its program-level children are deliberately broad `DRAFT` specifications. Their existence does **not** grant implementation authority.

Current lifecycle:

```text
DRAFT -> SHAPED -> REFINING -> GRAIN
```

A leaf becomes a Grain only when deterministic readiness establishes that all of these are bounded and explicit:

- outcome;
- scope in;
- scope out;
- acceptance;
- dependencies;
- risk and recovery;
- context budget;
- intended change surface;
- evidence requirements;
- minimality decision;
- safety status.

If a unit is still too broad to verify independently, refine it again. Do not solve context pressure by giving an agent a larger prompt.

## 4. Progressive planning

Morize uses rolling-wave planning.

### Later work

Keep later work coarse. Record architecture constraints and dependency direction, not hundreds of brittle implementation tasks.

### Next work

Shape the next dependency-eligible specification enough to expose uncertainty, risk, acceptance, and likely change surfaces.

### Now

Only a dependency-eligible Grain receives an execution packet.

This prevents the P10-P13 plan from becoming fake precision while P1/P2 realities are still unknown.

## 5. Method routing

Use SpecGrain method profiles deliberately.

### Controlled flow

Default for:

- canonical storage;
- migration;
- crash recovery;
- authorization;
- privacy/security;
- secret handling;
- multi-user isolation;
- release/signing;
- destructive forget/redact;
- synchronization.

These require stronger review, rollback/recovery proof, and explicit evidence.

### DMADV-lite

Default for meaningful new product capabilities whose behavior does not yet exist:

- typed memory decisions;
- Temporal Truth Ledger;
- Context Compiler;
- Memory Inspector;
- team mode;
- commercial managed-service interfaces.

Define -> Measure -> Analyze -> Design -> Verify.

### DMAIC-lite

Use for reproduced defects, regressions, performance problems, and reliability failures.

### Experiment flow

Use when the decision itself is uncertain:

- whether vectors materially improve retrieval;
- whether a graph backend is justified;
- local model choice;
- reranking;
- memory extraction approaches;
- benchmark protocol questions.

Experiments produce evidence and a decision. They do not silently become production code.

### Quick flow

Use only for low-risk already-understood changes such as isolated documentation corrections.

## 6. WorkPacket contract

A WorkPacket must bind:

- exact Grain revision;
- exact repository baseline;
- required context sources and their provenance/revisions;
- context budget;
- allowed scope/change surface;
- acceptance checks;
- evidence expectations;
- risk profile;
- recovery instructions.

Execution adapters may target different agents, but the packet is agent-neutral. Provider behavior never changes the Grain's authority.

## 7. Diffcipline is the proof-before-done layer

Morize adopts the Diffcipline v1 proof semantics:

- `PASS / 0` — configured hard requirements were observed and satisfied;
- `REVIEW / 1` — no hard requirement failed, but judgment or evidence remains;
- `FAIL / 2` — a hard requirement or verification failed;
- `64` — usage/execution error; no proof verdict exists.

Binding rule:

`NOT RUN != PASS`

Every implementation Grain must produce proof tied to the exact implementation revision.

## 8. Diffcipline evidence classes

For each Grain, capture:

1. **diff evidence** — exact changed paths and line counts;
2. **dependency evidence** — manifest changes;
3. **lockfile evidence** — lockfile changes;
4. **workspace evidence** — untracked state where applicable;
5. **scope evidence** — expected and forbidden surfaces;
6. **risk evidence** — selected R0-R3 profile;
7. **verification evidence** — exact commands and observed outcome;
8. **policy provenance** — policy inputs used.

A successful test command proves only that command. It does not imply unrelated acceptance criteria passed.

## 8A. Alibaba Open Code Review

Morize uses Alibaba Open Code Review (OCR) as a **review-only process tool** when available.

Preferred mode for agent-hosted review is OCR Delegation Mode:

```text
ocr delegate preview
 -> deterministic reviewable/excluded file inventory
ocr delegate rule
 -> deterministic rule resolution for reviewable files
host semantic reviewer
 -> inspect every reviewable file and explicitly account for exclusions
```

Rules:

- OCR review evidence binds an exact repository base/head.
- Every OCR-reviewable file must end as reviewed or skipped with a reason.
- Files excluded by OCR because of unsupported extensions (for example planning Markdown) are **not silently omitted**; they receive separate semantic review when they are part of the change's authority surface.
- OCR findings are input to the review gate, not automatic truth.
- OCR does not replace SpecGrain readiness, Diffcipline verification, source/license review, or required independent review for R3 work.
- OCR is not a runtime dependency of Morize.
- A third-party status check that did not perform substantive review is not treated as semantic-review evidence.

## 9. Morize risk-profile mapping

### R0 — Documentation / non-behavioral

Examples: typo, prose clarification, source ledger metadata.

Minimum:
- exact diff inspection;
- formatting/link/basic structural checks available to the repository;
- no unrelated changes.

### R1 — Low-risk product behavior

Examples: isolated parser/helper, additive CLI presentation, non-persistent deterministic utility.

Minimum:
- formatting;
- static analysis;
- focused tests;
- impacted integration tests.

### R2 — Persistent or boundary behavior

Examples:
- schema changes;
- vault writes;
- retrieval behavior;
- MCP/API behavior;
- connector imports;
- migration code;
- new dependencies.

Minimum:
- R1;
- full workspace tests;
- persistence/migration compatibility tests where relevant;
- dependency/source review;
- failure-path tests;
- platform coverage appropriate to the change.

### R3 — High-risk trust/recovery/release behavior

Examples:
- authorization;
- secrets;
- cross-tenant isolation;
- forget/redact;
- crash reconciliation;
- synchronization;
- cryptography;
- release/update integrity.

Minimum:
- R2;
- adversarial/security tests;
- rollback/recovery rehearsal;
- fault injection where applicable;
- cross-platform checks;
- independent semantic review;
- exact-head release/merge qualification.

A Grain may require stronger checks than its nominal profile.

## 10. Diffcipline repository policy activation

The planning branch does not fabricate an executable Diffcipline `.toml` for a Rust workspace that does not exist yet.

P1 must create and validate the repository's `.diffcipline.toml` together with the real workspace commands. Until then:

- planning uses the Diffcipline proof model;
- documentation-only PRs remain reviewable through exact diff and semantic review;
- no claim is made that runtime verification passed.

Once P1 exists, the policy becomes checked-in canonical repository state and CI pins an immutable Diffcipline release/source identity.

## 11. Dependency policy

Before adding a runtime dependency, the owning Grain must answer:

1. What measured problem does it solve?
2. Can standard library/native code solve it sufficiently?
3. Is an already-admitted dependency sufficient?
4. What is the transitive closure?
5. What license/notice obligations apply?
6. Does it add network, telemetry, credential, unsafe/FFI, process, or platform behavior?
7. What is the founder-cost impact?
8. How is it removed or replaced?

Manifest and lockfile changes are at least R2 review surfaces.

## 12. Donor code policy

Founder permission makes a donor eligible, not trusted.

Every copied/adapted component must pass the donor/provenance policy and be linked to an exact source revision and selected paths.

A donor's architecture, approval semantics, telemetry, hosted assumptions, or security model never silently becomes Morize policy.

## 13. Change control

A material change to any of the following requires a planning/spec revision before implementation:

- canonical storage semantics;
- public API compatibility rules;
- scope/identity model;
- trust/authorization rules;
- data-retention semantics;
- license;
- founder-cost boundary;
- commercial-service boundary;
- release support policy;
- required infrastructure;
- benchmark claim protocol.

Do not bury architecture changes inside an implementation PR.

## 14. Phase exit gate

A roadmap phase is complete only when:

- every required child Grain is accepted;
- no dependency-required Grain remains open;
- evidence binds exact revisions;
- documentation matches implemented truth;
- applicable Diffcipline proofs are PASS;
- semantic review has no unresolved material finding;
- migration/recovery evidence exists where persistent state changed;
- known residual risks are recorded;
- the canonical merge and post-merge checks are successful where configured.

## 15. Release gate

A release candidate additionally requires:

- source/license audit;
- SBOM;
- supported-platform proof;
- upgrade/migration proof;
- backup/restore proof;
- security test closure;
- benchmark evidence for public claims;
- installer/package checks;
- release asset checksums and provenance;
- compatibility statement;
- founder-operated cost ledger update;
- documentation and examples bound to released behavior.

## 16. Planning metrics

Track process quality, not individual productivity:

- First-Pass Verification Rate;
- Rework Ratio;
- Grain Cycle Time;
- Context Efficiency;
- Spec Drift Rate;
- Escaped Defect Rate;
- Change-Scope Accuracy;
- dependency growth;
- benchmark reproducibility rate;
- unresolved-risk age.

Metrics must not incentivize smaller diffs at the expense of correctness or safety.

## 17. Current authority

The current `.specgrain` nodes are program-level **DRAFT** specifications only.

PR #1 was merged through GitHub web-flow at `2026-09-19T12:58:13Z` while the required independent semantic review of its exact planning content remained open. This is a governance breach that must be remediated forward; it is **not** implementation authority.

Shared history must not be force-pushed, rebased, rewritten, or otherwise altered to conceal the premature merge.

Current remediation gate before runtime implementation:

1. reverify exact live `main`, PR #1, and SpecGrain truth;
2. bind the effective canonical planning content to merge commit `ba4732d348da245c503ab115c2c41e9a91914dcb`;
3. complete substantive independent semantic review against that effective canonical planning content;
4. resolve every valid material finding through normal forward commits and PRs;
5. rerun exact structural, source/provenance, Alibaba OCR accounting, and applicable semantic qualification on the resulting exact head;
6. only after that remediation gate closes, progressively shape SG-000010/P1 from DRAFT -> SHAPED -> REFINING -> GRAIN;
7. create the real Rust workspace and Diffcipline policy only inside the first dependency-eligible accepted Grain.

No current DRAFT is implementation authority.
