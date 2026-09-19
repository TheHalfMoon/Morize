# Morize Execution Master Plan

**Program mode:** P1_PROGRESSIVE_EXECUTION
**Canonical baseline:** `cc78368cd8cf8f63cc55d0b733cf1ae8ed295543`
**Foundation planning PR:** #1
**Current implementation authority:** NONE — the next bounded Grain is not yet authorized
**Completed/proven Grain:** `SG-000011`
**Current SpecGrain state:** broad program DRAFTs plus canonical SG-000011 GRAIN with verified proof

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

It also supports independent `VerificationReport` / hash-chained `EvidenceRecord` proof and
`specgrain prove`, but the pinned source intentionally does not expose a supported writer for:

```text
GRAIN -> READY -> RUNNING -> VERIFYING -> VERIFIED -> CONTROLLED
```

Morize must not fabricate those lifecycle states. ADR-0004 defines a temporary, fail-closed
verified-proof prerequisite bridge for canonically completed work until a qualified SpecGrain
revision supplies that writer.

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

The broad program nodes are children of the `SG-000001` program root. `SG-000011` is the
first bounded child under broad program node `SG-000010`. Refinement parent/child structure,
native execution dependencies, and ADR-0004 verified-proof prerequisites are distinct contracts:

```text
SG-000002  Foundation/governance/license/commercial
           planning contract; no execution dependency

SG-000010  Deterministic Rust kernel/data contracts [program DRAFT]
  |
  +-- refinement child SG-000011
      Minimal Rust workspace/verification spine [GRAIN]
      canonical implementation: b54879d7c04ba914997f14034c3c1b262b9629f6
      canonical SpecGrain proof: verified=true

SG-000003  Vault/persistence/migration/recovery [program DRAFT]
SG-000004  Identity/policy/privacy/firewall [program DRAFT]
SG-000005  Temporal truth/provenance/evidence graph [program DRAFT]
SG-000006  Retrieval/context/APIs/MCP [program DRAFT]
SG-000007  Portability/integrations/intelligence [program DRAFT]
SG-000008  Experience/UI/team/commercial boundary [program DRAFT]
SG-000009  Memory Lab/security/release/sustainability [program DRAFT]
```

The exact JSON dependency graph in `.specgrain/specs/` is canonical for native dependencies.
The diagram is explanatory and must not be used to infer edges that are absent from the JSON.
ADR-0004 proof prerequisites are explicit metadata/governance contracts and are not native
SpecGrain dependency satisfaction.

All current program-level nodes remain DRAFT by design. `SG-000011` is a bounded child Grain,
not a promoted program-level node.

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

PR #1 was canonically merged as
`ba4732d348da245c503ab115c2c41e9a91914dcb` before its independent semantic-review gate
closed. The missing gate was subsequently closed by the documented post-merge independent
semantic-review PASS against that exact canonical content. Together, the merge record and the
required review/evidence closure satisfy the foundation planning governance prerequisite; they
are not a SpecGrain lifecycle transition for broad program DRAFT `SG-000002`.

Do not start from the full roadmap. Refine the P1 program node into bounded implementation
Grains and execute only leaves whose native dependencies and any ADR-0004 verified-proof
prerequisites are satisfied on the exact packet baseline.

Current proven progression:

```text
planning closeout: PR #1 merge + exact-canonical remediation review
 -> SG-000011 minimal Rust workspace/tooling Grain
 -> implementation b54879d7c04ba914997f14034c3c1b262b9629f6
 -> canonical evidence record
    sha256:1239df705eb00068720cd5c765f2566bb7a8519afe638a1ded4ddc3eee4e70b9
 -> specgrain prove SG-000011 = verified=true
 -> NEXT: shape a bounded deterministic identity/data-contract Grain under SG-000010
```

The next Grain may rely on SG-000011 only through ADR-0004's exact verified-proof prerequisite
contract while the current SpecGrain pin lacks a supported post-Grain lifecycle writer.
No packet may be exported until that proof is revalidated against the exact packet baseline.

Broad program dependencies must not be treated as executable satisfied state. When SG-000003 or
SG-000004 is refined, their bounded children must depend on the specific accepted kernel/data
contracts they actually require rather than using a broad DRAFT as a completion proxy.

## 8. Completed first Rust Grain

`SG-000011` delivered the minimal repository execution spine:

- root Cargo workspace;
- one first-party `morize-core` crate;
- zero third-party runtime/build dependencies;
- real offline format/check/clippy/test commands;
- checked-in Diffcipline policy;
- Ubuntu/macOS/Windows CI;
- exact developer verification commands;
- no memory-domain behavior.

Canonical implementation:

```text
b54879d7c04ba914997f14034c3c1b262b9629f6
```

Canonical evidence:

```text
record =
sha256:1239df705eb00068720cd5c765f2566bb7a8519afe638a1ded4ddc3eee4e70b9
specgrain prove SG-000011 = verified=true
```

The next implementation surface is not authorized until its own bounded Grain is shaped,
reviewed, merged, and packet prerequisites pass.

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
