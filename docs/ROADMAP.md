# Morize Implementation Roadmap

Morize is implemented in dependency order. The roadmap is split into bounded planning packets so implementation agents do not need to ingest one oversized document.

## Sequence

```text
P0 Foundation
 -> P1 Deterministic Rust kernel
 -> P2 Vault and reliable writer
 -> P3 Scopes and policy
 -> P4 Temporal truth and evidence graph
 -> P5 Deterministic retrieval
 -> P6 Interfaces
 -> P7 Portability and integrations
 -> P8 Optional local intelligence
 -> P9 Optional semantic retrieval
 -> P10 Branches and experience records
 -> P11 Memory Inspector
 -> P12 Team/self-hosted mode
 -> P13 Memory Lab and v1 release
```

## Planning packets

- [Foundation through deterministic retrieval](roadmap/FOUNDATION_CORE.md) — P0–P5
- [Interfaces, portability, and optional intelligence](roadmap/INTERFACES_INTELLIGENCE.md) — P6–P9
- [Experience and Memory Inspector](roadmap/EXPERIENCE_UI.md) — P10–P11
- [Team mode and v1 release](roadmap/TEAM_RELEASE.md) — P12–P13

## Delivery discipline

Every implementation unit must define:

- exact intent and scope;
- source/donor disposition when relevant;
- acceptance checks;
- security considerations;
- tests and evidence;
- exact implementation revision.

Use SpecGrain-style bounded work packets and Diffcipline-style proof-before-done.

## First implementation frontier

After the planning package is accepted, implementation starts with P1 contracts. No UI, hosted service, model integration, vector database, or external graph server precedes the P1–P5 deterministic spine.
