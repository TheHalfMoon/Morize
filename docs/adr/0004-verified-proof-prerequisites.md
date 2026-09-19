# ADR-0004 — Verified-Proof Prerequisites Until Post-Grain Writers Exist

**Status:** Accepted for the current SpecGrain frontier  
**Date:** 2026-09-19

## Context

Morize pins SpecGrain at:

```text
TheHalfMoon/SpecGrain@5de7d6499bb0a9e3a191fc0934399cf099d1980a
```

That source has two intentionally separate truths:

1. native dependency eligibility treats an upstream dependency as satisfied only when its SpecNode state is `VERIFIED` or `CONTROLLED`;
2. independent verification is recorded through `VerificationReport` and immutable hash-chained `EvidenceRecord` values, and `specgrain prove` can report `verified=true`.

The same source intentionally does **not** provide a supported native writer for:

```text
GRAIN -> READY -> RUNNING -> VERIFYING -> VERIFIED -> CONTROLLED
```

A `VerificationReport` explicitly does not mutate lifecycle state.

Morize has now exercised this exact boundary with `SG-000011`:

```text
SpecNode state = GRAIN
specgrain prove SG-000011 = verified=true
latest record =
sha256:1239df705eb00068720cd5c765f2566bb7a8519afe638a1ded4ddc3eee4e70b9
implementation revision =
b54879d7c04ba914997f14034c3c1b262b9629f6
```

Manually editing the node to `VERIFIED` would bypass SpecGrain's explicit mutation-authority boundary. Encoding a native dependency on a node that the supported writer cannot move into a dependency-satisfied state would instead make downstream work permanently ineligible.

## Decision

Until Morize adopts a SpecGrain revision with a supported post-Grain lifecycle writer, a completed executable prerequisite MAY be represented as a **verified-proof prerequisite** rather than a native `dependencies` edge, but only under the fail-closed contract below.

A bounded successor Grain that relies on already-completed canonical work MUST declare each such prerequisite in:

```json
{
  "metadata": {
    "morize": {
      "proof_prerequisites": [
        {
          "spec_id": "SG-000011",
          "spec_revision": "sha256:...",
          "record_digest": "sha256:...",
          "implementation_revision": "<git-sha>"
        }
      ]
    }
  }
}
```

The corresponding native `dependencies` entry is omitted only because the pinned SpecGrain source cannot lawfully transition the proven predecessor to a dependency-satisfied lifecycle state.

This bridge is **not** native SpecGrain dependency satisfaction and MUST NOT be described as such.

## Pre-packet gate

Before a WorkPacket may be exported or executed for a Grain using `proof_prerequisites`, all of the following are required:

1. the prerequisite evidence record is present on canonical `main`;
2. `specgrain prove <SPEC_ID>` returns `verified=true` with no evidence-chain issue;
3. the latest required record digest exactly matches the Grain metadata;
4. the proof's `spec_revision` exactly matches the metadata;
5. the proof's `implementation_revision` exactly matches the metadata;
6. the implementation revision is an ancestor of the packet baseline;
7. the evidence record itself is present at the packet baseline;
8. the prerequisite evidence record and this ADR are included as WorkPacket context sources;
9. any mismatch, missing proof, failed proof, forked chain, changed prerequisite identity, or non-canonical implementation blocks packet export and execution.

The gate MUST be rerun against the exact packet baseline. Historical PASS is not enough.

## Restrictions

This bridge MUST NOT be used to:

- omit an unresolved executable dependency;
- treat executor self-report as verification;
- treat a broad program DRAFT as completed work;
- bypass a native dependency that can already be represented and satisfied by the adopted SpecGrain source;
- fabricate `VERIFIED` or `CONTROLLED` lifecycle state;
- skip required review, CI, Diffcipline, migration, security, or recovery evidence;
- infer dependency satisfaction merely because code exists on `main`.

A successor that has both unresolved native dependencies and verified-proof prerequisites must satisfy **both** classes of prerequisite before execution.

## Migration / expiry

This bridge is temporary.

When Morize adopts a qualified SpecGrain revision that provides supported post-Grain lifecycle mutation and a safe migration path:

1. stop creating new proof-only prerequisite declarations where native dependency state can represent the relationship;
2. migrate future successor Grains to native `dependencies` edges;
3. preserve historical evidence records and proof-prerequisite metadata for auditability;
4. do not rewrite prior commits or evidence chains.

## Consequences

- Morize can continue bounded delivery without falsifying SpecNode lifecycle state.
- Exact proof remains stronger than executor self-report and remains hash-bound to the implementation.
- Native SpecGrain semantics remain authoritative where the current product surface can actually express them.
- The temporary bridge adds an explicit pre-packet governance check that must remain fail-closed.
- A future SpecGrain writer removes this compatibility bridge rather than creating a second permanent dependency system.

## Revisit triggers

- SpecGrain adds a supported post-Grain lifecycle writer;
- `specgrain packet` gains native verified-proof prerequisite semantics;
- evidence-chain semantics change;
- a proof-prerequisite mismatch or bypass is observed in Morize;
- the bridge becomes materially harder to audit than completing the missing SpecGrain capability.
