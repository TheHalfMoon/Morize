# Morize Execution Master Plan

**Program mode:** FOUNDATION_PLANNING  
**Canonical planning branch:** `plan/morize-memory-os`  
**Planning PR:** #1  
**Current implementation authority:** NONE  
**Current SpecGrain state:** program-level DRAFT decomposition only

## 1. Mission

Build Morize into a production-grade Memory Operating System whose durable semantics are governed, temporal, provenance-aware, explainable, portable, and safe to use across agents, applications, local/self-hosted deployments, and future managed commercial services.

## 2. Founder and licensing decisions

Binding foundation decisions:

```text
license = Apache-2.0
contribution_attestation = DCO-1.1
local_self_hosted = first-class
founder_pre_revenue_required_recurring_paid_service = none authorized
future_paid_managed_services = allowed
future_user_pricing = intentionally not frozen
```

Founder-zero-burn is an engineering/economic constraint on the current project-operated path, not a user-free-forever promise.

## 3. Delivery system

Morize uses two independent control layers:

### SpecGrain — work readiness

Pinned research state for planning:

```text
repository = TheHalfMoon/SpecGrain
observed_commit = 5de7d6499bb0a9e3a191fc0934399cf099d1980a
```

Current source supports the native preparation lifecycle:

```text
DRAFT -> SHAPED -> REFINING -> GRAIN
```

The historical published v0.3.0 release does not contain every current-main preparation command. Actual implementation tooling must pin an exact supported SpecGrain source/release rather than assuming floating-main behavior.

### Diffcipline — finish-line proof

Planning reference:

```text
repository = TheHalfMoon/Diffcipline
current observed repository head = 1e6d14f77b95bb132b42276f10d67f1018ab5bb6
immutable v1.0.0 release commit = 5cb1c77340b75649f6168e0e8f66479ea047ea96
```

Implementation/CI should pin an immutable qualified Diffcipline identity.

Binding rule:

`NOT RUN != PASS`

## 4. Canonical authority order

When facts conflict:

1. live GitHub/repository truth;
2. `AGENTS.md`;
3. canonical `.specgrain/` state;
4. accepted ADRs and requirements;
5. architecture, threat, data-model, compatibility, cost/commercial, deployment, and sync contracts;
6. active Grain/WorkPacket;
7. roadmap;
8. donor/source research;
9. historical summaries/chat handoffs.

No previous completion claim overrides a newer repository fact.

## 5. Current program graph

```text
SG-000001  Deliver Morize v1
 |
 +-- SG-000002  Foundation/governance/license/commercial
 |
 +-- SG-000003  Vault/persistence/migration/recovery
 |      |
 +-- SG-000004  Identity/policy/privacy/firewall
 |      |
 +------+- SG-000005  Temporal truth/provenance/evidence graph
                  |
                  +-- SG-000006  Retrieval/context/APIs/MCP
                         |
                         +-- SG-000007  Portability/integrations/intelligence
                         |
                         +------------- SG-000008  Experience/UI/team/commercial boundary
                                          |
                                          +-- SG-000009  Memory Lab/security/release/sustainability
```

The exact JSON dependency graph in `.specgrain/specs/` is canonical.

All current nodes are DRAFT by design.

## 6. Planning closeout conditions

Foundation planning is ready to merge only when:

- Apache-2.0/NOTICE/DCO/governance are internally consistent;
- founder-cost meaning is unambiguous;
- future paid-service architecture remains allowed;
- requirements cover all known product/system categories;
- architecture/data/API/deployment/sync/operations contracts agree;
- threat model and risk register cover material known risks;
- donor/source policy is exact about permission vs admission;
- roadmap maps to SpecGrain decomposition;
- no current broad DRAFT is falsely marked executable;
- PR diff contains no stale MIT/user-free-forever contract;
- a substantive independent semantic review is completed or its absence remains an explicit blocker;
- planning PR remains unmerged until its applicable review gate closes.

## 7. First implementation frontier

After the planning package is canonically merged, do not start from the full roadmap.

Start from the first dependency-eligible program child and refine it.

The first likely implementation sequence is:

```text
SG-000002 planning closeout
 -> refine SG-000003/P1-P2 foundation into bounded children
 -> first Rust workspace/kernel Grain
 -> concrete persisted type/serialization Grains
 -> vault-format/writer/recovery Grains
```

Identity/policy work under SG-000004 may proceed in dependency-safe parallel only when shared type/storage boundaries are stable enough to avoid speculative duplication.

## 8. First Rust Grain requirements

The first actual implementation Grain should be narrow.

Expected outcome class:

- create the minimal Rust workspace/tooling foundation;
- no feature-rich memory implementation yet;
- establish real build/test/static-analysis commands;
- create the actual repository `.diffcipline.toml`;
- establish cross-platform CI;
- add Apache/DCO/source-policy checks where appropriate;
- prove no unnecessary runtime infrastructure dependency.

Its exact scope/change surface is not frozen until SpecGrain shaping against merged main.

## 9. Dependency strategy

Default decision ladder:

1. do not build unnecessary behavior;
2. reuse an existing Morize primitive;
3. use Rust standard library/platform facilities when sufficient;
4. use an already-admitted dependency when appropriate;
5. add the smallest new dependency only with measured need and source/cost/security review;
6. selectively port authorized donor mechanisms only when superior to a small Morize-native implementation.

Donor abundance must not become dependency abundance.

## 10. No premature infrastructure

The following are not default prerequisites for the deterministic spine:

- managed cloud;
- Postgres;
- Redis;
- Kafka;
- Qdrant;
- Neo4j;
- Elasticsearch;
- Kubernetes;
- Docker;
- hosted model APIs;
- hosted embedding APIs.

They may become justified profile-specific adapters through later SpecGrains.

## 11. Architecture change triggers

Return to planning before implementation if a change materially affects:

- canonical storage;
- persistent schema;
- memory identity/version meaning;
- temporal semantics;
- provenance;
- authorization;
- forget/redact;
- distributed synchronization;
- public stable API;
- license/contribution terms;
- founder recurring cost;
- required hosted infrastructure;
- commercial entitlement/billing separation;
- release/security support policy.

Do not hide an ADR-sized decision in an implementation diff.

## 12. Verification levels

### Planning/docs changes

Exact diff, structural consistency, source/link validation where possible, semantic review.

### R1 implementation

Formatting/static analysis/focused tests/impacted integration.

### R2 persistent/API/dependency

R1 + full workspace tests + failure paths + migration/compat/dependency evidence + applicable platform proof.

### R3 trust/recovery/release

R2 + adversarial/security/fault injection/recovery/rollback + independent semantic review + exact-head qualification.

## 13. Evidence policy

Never fabricate or infer:

- test success;
- CI;
- benchmark result;
- semantic review;
- source pin;
- provider/model behavior;
- runtime compatibility;
- cost;
- release readiness.

Preserve negative and failed evidence.

## 14. Release sequence

Directional release program:

```text
v0.1  deterministic core preview
v0.2  interfaces/integration preview
v0.3  evidence-justified intelligence preview
v0.4  experience/inspector preview
v0.9  release candidate
v1.0  stable
```

Every release has its own SpecGrain. These numbers are not deadlines and do not authorize publication automatically.

## 15. Commercial program boundary

Do not implement billing merely because the architecture allows it.

A commercial launch gets a separate controlled specification covering:

- customer/market evidence;
- pricing;
- metering;
- entitlements;
- payments;
- taxes;
- retention/export/cancellation;
- privacy/terms/subprocessors;
- service security;
- support/SLA;
- operating cost/margins;
- billing/provider failure.

The memory semantic core must remain valid whether a user is self-hosted, BYO-provider, or managed.

## 16. Open decisions that do not block P1

Tracked future decisions:

- formal trademark policy;
- detailed multi-maintainer succession when a maintainer community exists;
- hosted privacy/terms/subprocessor package before cloud launch;
- exact commercial pricing/payment/tax model before charging users;
- SLO/SLA numbers after operational baselines;
- semantic vector/graph/model backend choices after experiments.

These are not implementation gaps in the deterministic P1-P5 spine.

## 17. Continuation rule

At every continuation:

1. reverify live main/PR/SpecGrain truth;
2. identify the next dependency-eligible bounded gap;
3. refuse to promote broad DRAFTs into implementation authority;
4. refine only as far as current evidence supports;
5. execute one bounded Grain;
6. collect exact proof;
7. close/reconcile it before advancing;
8. update planning only when new evidence changes the design.

The project should advance by proven Grains, not by accumulating optimistic prose.
