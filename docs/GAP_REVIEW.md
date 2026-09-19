# Morize Master Plan Gap Review

**Review date:** 2026-09-19  
**Purpose:** challenge the current plan before implementation and identify missing category-level obligations.

## Review standard

A planning area is considered covered only when the repository contains:

1. a clear product/architecture position;
2. a requirement or explicit non-goal;
3. a roadmap owner/dependency direction;
4. a verification/evidence expectation where applicable.

“Mentioned somewhere” is not enough.

## Executive result

The original foundation plan had a strong memory-core architecture but was incomplete as a full professional product program.

The largest gaps were:

- founder-cost intent was incorrectly expressed as a user-free promise;
- licensing was MIT rather than the requested Apache-2.0;
- commercial/hosted service boundaries were under-specified;
- SpecGrain and Diffcipline were referenced but not made canonical delivery governance;
- persisted schema compatibility/migration policy needed stronger treatment;
- multi-device synchronization/conflict semantics were deferred without explicit requirements;
- public API/SDK compatibility policy was not explicit;
- operational observability and telemetry consent were not explicit;
- future billing/entitlement failure modes were absent;
- release support, upgrade, and maintenance policy were incomplete;
- project sustainability/founder recurring-cost review was absent;
- data residency/retention/compliance boundaries needed explicit non-claims;
- extension/plugin trust and revocation needed stronger requirements.

This revision closes those **category-level planning gaps**. It does not claim implementation detail is already fully shaped; SpecGrain refinement remains required.

## Coverage matrix

| Area | Prior state | Revised planning status | Canonical owner |
|---|---|---|---|
| Product category / non-goals | Strong | Covered | Product thesis |
| Apache-2.0 license | Missing | Covered | LICENSE, NOTICE |
| Founder-zero-burn meaning | Incorrect scope | Covered | Founder cost boundary |
| Future user monetization | Missing | Covered | Founder cost boundary / requirements |
| Open-source vs managed service boundary | Partial | Covered | Founder cost boundary |
| Donor/source provenance | Strong | Strengthened | Donor policy / source ledger |
| SpecGrain program graph | Mentioned only | Covered | `.specgrain/`, planning governance |
| Diffcipline finish-line proof | Mentioned only | Covered | Planning governance / requirements |
| Canonical data model | Strong conceptual | Covered; still needs Grain shaping | Architecture / requirements |
| Schema versioning | Partial | Covered | Requirements |
| Migration compatibility | Partial | Covered | Requirements / P2 |
| Crash consistency | Strong | Covered | Architecture / threat / P2 |
| Backup/restore | Present | Covered | Requirements / P2 / release |
| Corruption detection | Partial | Covered | Requirements |
| External user-edit reconciliation | Strong | Covered | Architecture / requirements |
| Temporal valid/knowledge time | Strong | Covered | Architecture / P4 |
| Contradiction/supersession | Strong | Covered | Architecture / P4 |
| Provenance/taint | Strong | Covered | Architecture / threat |
| Principal identity / ACL | Strong | Covered | P3 / requirements |
| Revocation/cache invalidation | Partial | Covered | Requirements |
| Prompt-injection persistence | Strong | Covered | Threat model |
| Secrets / sensitive content | Strong | Covered | Threat model / requirements |
| Path/archive safety | Strong | Covered | Threat model / requirements |
| Retrieval hierarchy | Strong | Covered | Architecture / P5 |
| Explainable retrieval | Strong | Covered | Architecture / requirements |
| Context budgets | Strong | Covered | Kernux-derived architecture / requirements |
| Semantic/vector optionality | Strong | Covered | P9 |
| API versioning | Weak | Covered | Requirements |
| SDK contract parity | Partial | Covered | Requirements |
| MCP least authority | Strong | Covered | P6 |
| Connector identity/data boundary | Partial | Covered | Requirements |
| Extension/plugin trust | Partial | Covered | Requirements |
| Stable project identity | Present | Covered | P7 / requirements |
| Model/provider identity | Present | Covered | P8 / requirements |
| AI confidence/abstention | Strong | Covered | Typed decision model / requirements |
| Branch/snapshot behavior | Present | Covered | P10 |
| Multi-device sync | Weak | Covered at requirement level | P12 / requirements |
| Offline concurrent conflicts | Missing | Covered at requirement level | Requirements |
| Forget/redact sync propagation | Missing | Covered at requirement level | Requirements |
| Memory Inspector | Present | Covered | P11 |
| Review/quarantine UX | Partial | Covered | Requirements / P11 |
| Accessibility | Present | Covered | P11 / requirements |
| Localization/i18n | Not required for v1 | Explicitly deferred | Future spec after user evidence |
| Structured operational events | Weak | Covered | Requirements |
| Telemetry consent/privacy | Missing | Covered | Requirements |
| Performance budgets | Present but loose | Covered; thresholds await baseline | Benchmark plan / requirements |
| SLOs for hosted service | Premature | Deferred to managed-service spec | Commercial boundary |
| Hosted control plane | Partial | Covered as optional future surface | Founder cost boundary / P12 |
| Usage metering | Missing | Covered as future commercial requirement | Requirements |
| Entitlements | Missing | Covered as future product layer | Requirements |
| Billing outage behavior | Missing | Covered | Requirements |
| Pricing | Premature | Intentionally not frozen | Founder cost boundary |
| User cancellation/export | Partial | Covered | Requirements |
| Retention | Partial | Covered for hosted/team profiles | Requirements |
| Data residency | Missing | Covered as explicit hosted concern | Requirements |
| Domain compliance claims | Risky/implicit | Explicitly bounded | Requirements |
| Release SBOM/provenance | Present | Covered | P13 |
| Artifact integrity | Present | Covered | P13 / requirements |
| Upgrade proof | Weak | Covered | Requirements |
| Support/security version policy | Missing | Covered | Requirements |
| Docs/release parity | Partial | Covered | Requirements |
| Founder recurring-cost ledger | Missing | Covered | Founder cost boundary |
| Dependency cost/exit plan | Partial | Covered | Planning governance |
| Trademark strategy | Missing | Open decision before public brand scale | Gap action G-01 |
| Contribution legal policy (DCO/CLA) | Missing | Foundation resolved: DCO 1.1, no CLA | DCO / GOVERNANCE / G-02 |
| Governance succession/maintainers | Missing | Founder-led foundation resolved; multi-maintainer details deferred until needed | GOVERNANCE / G-03 |
| Hosted privacy policy/ToS | Not yet applicable | Deferred to hosted launch | Gap action G-04 |
| Payment processor/tax operations | Not yet applicable | Deferred to monetization spec | Gap action G-05 |

## Remaining intentional open decisions

The plan should not pretend every future business/legal choice is already known.

### G-01 — Trademark policy

Before meaningful external brand distribution, decide:

- whether “Morize” and logos receive a separate trademark policy;
- acceptable downstream naming;
- compatibility with Apache-2.0 section 6.

This does not block P1 implementation.

### G-02 — Contribution legal mechanism — RESOLVED FOR FOUNDATION

Foundation decision:

- Apache-2.0 inbound/outbound licensing;
- Developer Certificate of Origin 1.1;
- contributor sign-off via `Signed-off-by`;
- no CLA required initially.

Revisit only if later financing, corporate structure, or legal/commercial requirements justify a different contribution agreement.

### G-03 — Maintainer governance — FOUNDATION MODEL RESOLVED

`GOVERNANCE.md` now establishes a founder-led initial model with explicit product, architecture, release, security, branding, and commercial-service authority.

Detailed multi-maintainer admission/removal, inactivity, succession, and conflict procedures remain intentionally deferred until a real maintainer community exists.

### G-04 — Hosted legal/privacy package

Before operating Morize Cloud or processing customer data, create a dedicated managed-service specification covering:

- privacy policy;
- terms of service;
- subprocessors;
- retention;
- deletion;
- security commitments;
- regional/data residency choices;
- incident response;
- account closure/export.

### G-05 — Monetization operations

Before charging users, create a dedicated commercial-launch specification covering:

- pricing metric;
- plan/entitlement model;
- free trial/free tier if any;
- usage metering;
- payment processor;
- refunds/credits;
- tax/VAT obligations;
- quota/budget controls;
- abuse/fraud handling;
- billing outage behavior;
- support/SLA commitments.

## Architecture challenge findings

### 1. Markdown canonical memory is powerful but must not become an arbitrary authority channel

Mitigation:
- content is user-readable;
- reserved authority metadata remains governed;
- external edits become reconciliation inputs;
- policy/ACL/effect state stays outside user-editable body authority.

### 2. SQLite-first is appropriate for local v1 but team scale needs an abstraction boundary

Mitigation:
- freeze semantic repository interfaces, not SQLite-specific public APIs;
- keep hosted/team storage adapters behind the same versioned contracts;
- do not prematurely introduce Postgres/Neo4j/Qdrant before measured need.

### 3. Evidence graph can grow into an unbounded ontology

Mitigation:
- small versioned relation vocabulary;
- explicit vs inferred edge class;
- bounded traversal;
- source-bound high-impact edges;
- ontology changes require planning revision.

### 4. “Memory OS” can sprawl into a general agent operating system

Mitigation:
- Morize owns memory lifecycle/context delivery, not general shell/browser/computer orchestration;
- integrations call external agents; Morize does not absorb their entire runtimes.

### 5. Commercial features can accidentally contaminate canonical semantics

Mitigation:
- billing/entitlements operate above semantic contracts;
- memory meaning and export remain independent of plan status;
- managed-service loss never rewrites canonical facts.

### 6. Local-first can be mistaken for “never cloud”

Mitigation:
- local/self-hosted remains a first-class architecture;
- cloud/managed services are allowed explicitly;
- data boundary and cost ownership are always visible.

### 7. Benchmark optimization can distort product quality

Mitigation:
- multiple benchmark families;
- Morize-specific adversarial/reliability suites;
- negative evidence preserved;
- no universal-superiority claim from one score.

### 8. Donor abundance can create architectural incoherence

Mitigation:
- source ledger is a candidate pool, not a shopping list;
- one Morize-owned contract per capability;
- selective source admission;
- dependency restraint and exact provenance.

## Planning completeness rule

After this gap review, no **known category-level** omission blocks the start of P1 shaping.

This does **not** mean “no future gaps can exist.” The correct rule is:

```text
new evidence -> record gap -> revise requirement/spec -> refine Grain -> implement -> prove
```

If a future implementation discovery materially affects architecture, security, cost, license, compatibility, or acceptance, it must return to planning rather than being smuggled through as incidental code.
