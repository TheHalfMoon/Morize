# ADR-0002 — SpecGrain for Readiness, Diffcipline for Proof

**Status:** Accepted for foundation planning  
**Date:** 2026-09-19

## Context

Large AI-generated plans can create false readiness, context sprawl, and weak completion claims.

## Decision

Use SpecGrain as the canonical decomposition/readiness system and Diffcipline as the deterministic finish-line proof system.

```text
Intent -> SpecGrain refinement -> Grain -> WorkPacket -> Execute -> Diffcipline proof -> Acceptance -> Merge
```

Broad roadmap phases do not grant implementation authority.

`NOT RUN != PASS`.

## Consequences

- work is progressively refined rather than overplanned;
- exact scope, context, risk, acceptance, and evidence are explicit;
- agent/provider choice is secondary to WorkPacket contract;
- completion claims bind exact diffs and executed verification;
- higher-risk persistence/security/release work receives stronger gates.

## Revisit triggers

- SpecGrain or Diffcipline contracts materially change;
- evidence shows the combined process creates more waste than control;
- a simpler deterministic replacement proves equivalent or better.
