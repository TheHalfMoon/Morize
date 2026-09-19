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


## SpecGrain execution mapping

The roadmap is directional planning only. Canonical implementation authority comes from `.specgrain/`.

Current program decomposition:

- `SG-000001` — Morize v1 root program;
- `SG-000002` — product/governance/license/commercial contracts;
- `SG-000003` — vault/persistence/migrations/recovery;
- `SG-000004` — identity/policy/privacy/firewall;
- `SG-000005` — temporal truth/provenance/evidence graph;
- `SG-000006` — retrieval/context/APIs/MCP;
- `SG-000007` — portability/integrations/optional intelligence;
- `SG-000008` — experience/inspector/team/commercial boundary;
- `SG-000009` — Memory Lab/security/release/sustainability.

All are currently DRAFT. They must be progressively refined before implementation.

## Diffcipline phase gate

Every implementation Grain closes with evidence appropriate to its risk profile under `docs/PLANNING_GOVERNANCE.md`.

At minimum:

```text
SpecGrain revision
+ exact implementation revision
+ exact diff
+ scope evidence
+ dependency/lockfile evidence
+ risk profile
+ executed verification
+ policy provenance
+ semantic acceptance where required
```

`NOT RUN != PASS`.

## Release progression

The program should publish evidence-shaped previews rather than hold all feedback until one giant v1:

- **v0.1 Core Preview** — P1-P5 deterministic kernel, vault, policy, temporal model, retrieval;
- **v0.2 Integration Preview** — P6-P7 CLI/API/MCP and major agent/migration integrations;
- **v0.3 Intelligence Preview** — selected evidence-justified P8-P9 local intelligence/semantic capabilities;
- **v0.4 Experience Preview** — P10-P11 branches/experience and Memory Inspector;
- **v0.9 Release Candidate** — P12 shared/self-hosted capabilities selected for v1 plus complete migration/security evidence;
- **v1.0 Stable** — P13 release qualification closes.

A preview number is not authorized merely by reaching a phase. Each release receives its own release SpecGrain and exact evidence.

## Commercial follow-on

A paid Morize-managed service is a separate controlled program after its business, security, privacy, billing, retention, support, and cost contracts are shaped.

Open-source v1 implementation must not accidentally commit Morize to a permanent free-hosting promise or a specific pricing model.
