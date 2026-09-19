# ADR-0003 — Local Markdown/SQLite Implementation Behind Provider-Neutral Semantics

**Status:** Accepted for foundation planning  
**Date:** 2026-09-19

## Context

Morize needs a low-cost, inspectable local implementation but may later operate team/managed deployments at larger scale.

## Decision

Use human-inspectable Markdown canonical content, SQLite operational/search state, and content-addressed local blobs for the initial local implementation.

Do not expose SQLite table identities or filesystem layout as the semantic public API.

Public contracts use Morize domain identities, versions, scopes, temporal semantics, and mutation rules.

## Consequences

Positive:
- founder-zero-burn local development;
- easy inspection/backup;
- minimal infrastructure;
- future hosted storage can change physical backend.

Trade-offs:
- Markdown/SQLite cross-store recovery must be engineered explicitly;
- hosted scale may later require new physical stores;
- migration tests are essential.

## Revisit triggers

- measured local performance is insufficient;
- hosted/team scale proves a different store necessary;
- correctness/recovery evidence shows the split is too complex.
