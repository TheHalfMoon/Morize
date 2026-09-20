# Donor and Provenance Policy

## Purpose

Morize is open source and may reuse authorized source code, but source permission does not eliminate engineering, security, attribution, or redistribution obligations.

Every copied or adapted component must remain traceable to an exact source state.

## Founder authorization recorded by planning

The founder has stated that Morize may use source code from repositories accessible in the founder's GitHub account, including public and private repositories, and has separately stated permission to copy/use the Memanto source for Morize. The founder has also stated permission to evaluate and copy source associated with Laya and TinyFish/Monid for Morize.

This planning record treats that statement as **eligibility to evaluate and transfer**, not as a reason to erase upstream notices or skip component-level review.

Private-source content is not published automatically. A public transfer from a private repository requires an exact transfer record.

## Source dispositions

Every candidate component receives one disposition:

- `REFERENCE` — ideas/behavior only;
- `DEPEND` — use as an external dependency;
- `ADAPT` — Morize-owned implementation informed by the source;
- `COPY` — selected source transferred into Morize;
- `REJECT` — do not use.

Broad repository-level permission never means `COPY EVERYTHING`.

## Required source record

Before source transfer, record:

```text
source_repository
source_visibility
source_revision
source_paths
source_license_or_permission_basis
founder_permission_reference_if_applicable
upstream_notices
generated_or_vendored_content
transitive_dependencies
network_behavior
telemetry_behavior
credential_behavior
filesystem_behavior
unsafe_ffi_process_boundaries
selected_reuse_strategy
Morize_destination_paths
independent_tests
security_review
founder_cost_and_commercial_impact
status
```

## Admission states

- `CANDIDATE`
- `QUALIFYING`
- `ADMITTED`
- `REJECTED`
- `REVOKED`

Only `ADMITTED` components may enter the trusted runtime path.

## License rule

Morize's project license is Apache License 2.0.

However, code copied from another source keeps any obligations that apply to that code unless Morize has an explicit valid relicensing grant covering the exact material.

Therefore:

- preserve copyright notices when required;
- preserve LICENSE/NOTICE obligations;
- list copied/adapted third-party material in `THIRD_PARTY_NOTICES.md`;
- prefer a dependency or clean adaptation when direct copying would complicate the repository's licensing;
- never assume “public on GitHub” means unrestricted copying;
- never assume founder permission over one repository grants rights over unrelated third-party vendored content inside it.

## Private source rule

When a useful source is private:

1. inspect it only through authorized access;
2. identify the smallest useful component;
3. verify the founder's right to publish that component;
4. remove unrelated secrets/configuration/data;
5. record its original revision privately during qualification;
6. create a public attribution/provenance record appropriate to the permission terms;
7. independently test the transferred code.

The planning branch itself does not copy private source text.

## Memanto-specific rule

Memanto is a primary direct donor candidate.

High-value areas to qualify include:

- memory extraction;
- read/write service behavior;
- memory policy;
- namespace patterns;
- migration/export;
- scheduling/maintenance;
- CLI/connect UX;
- MCP integration;
- coding-agent integrations;
- TypeScript SDK patterns.

Morize should not blindly retain:

- cloud-provider assumptions;
- hosted-service coupling;
- storage/search architecture that weakens Morize canonical-vault rules;
- any behavior that conflicts with typed governance, founder-cost constraints, commercial boundaries, or local/self-hosted semantics.

Prefer selective ports behind Morize-owned contracts.

## Laya-specific rule

Laya is a P8 optional-intelligence candidate, not a core authority dependency.

Before any model artifact, runtime code, or adapter is admitted:

- bind the exact model artifact digest and runtime/configuration identity;
- verify Apache-2.0 obligations and any component-specific notices;
- benchmark calibration, selective accuracy, abstention, latency, memory use, and failure behavior on Morize-relevant typed decisions;
- preserve deterministic Morize policy as the only authority for durable side effects;
- make model removal non-destructive to canonical memory;
- keep the adapter optional and replaceable.

Permission to copy does not authorize copying model weights or code into the repository without a component-level source record and size/distribution decision.

## TinyFish/Monid-specific rule

TinyFish/Monid is a candidate remote web-acquisition integration, not a required retrieval or memory dependency.

Before code transfer or runtime use:

- pin the exact source repository, revision, and selected paths;
- preserve MIT notices for copied Monid/TinyFish sample or connector code where applicable;
- document every remote endpoint, credential, request/response data boundary, rate limit, cost rule, and failure mode;
- treat current free Search/Fetch availability as provider state, not a permanent contract;
- route acquired web material through normal source trust, taint, provenance, bounds, and Memory Firewall controls before durable promotion;
- keep local/offline Morize correctness fully functional when the provider is unavailable or disabled;
- require a separate Grain before browser automation receives any side-effecting capability.


## Internal-source priority

The highest-value internal architecture sources identified during planning are:

1. `TheHalfMoon/Golam` — memory governance, provenance, reconciliation, strict-local design;
2. `TheHalfMoon/kernux` — context compiler/search/scopes/memory poisoning defenses;
3. `TheHalfMoon/SpecGrain` — bounded planning and evidence packets;
4. `TheHalfMoon/Diffcipline` — proof-before-done;
5. `TheHalfMoon/Himsat` — local-first durable/evidence architecture;
6. `TheHalfMoon/MedScale` — provenance/privacy/source qualification;
7. security and authority patterns from Kodac, Sentrdel, Tarif, and related internal projects.

## Dependency restraint

An external dependency is justified only when it is materially better than a small Morize-native implementation.

Evaluate:

- maintenance activity;
- auditability;
- binary size;
- transitive closure;
- unsafe/FFI;
- network/telemetry;
- platform support;
- license;
- deterministic behavior;
- replaceability;
- offline operation.

## Source update rule

A source admission binds an exact version or revision. Upstream movement does not silently update Morize authority.

Updating a donor component requires a new diff review and evidence.

## Provenance rule

```text
PERMISSION != ADMISSION
REPOSITORY != COMPONENT
COMPONENT != AUTHORITY
UPSTREAM_UPDATE != AUTOMATIC_UPDATE
COPYING != TRUST
```

Source reuse should accelerate Morize without making its trust model dependent on unexamined donor behavior.
