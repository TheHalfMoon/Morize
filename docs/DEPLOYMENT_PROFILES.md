# Morize Deployment and Service Profiles

## 1. Purpose

Morize is one semantic product with multiple deployment profiles. Deployment choice must not silently change memory meaning, authorization semantics, or exportability.

## 2. Profile A — Embedded local

Use case:

- application embeds Morize library directly;
- one local user/process boundary;
- no daemon required.

Properties:

- canonical vault on local filesystem;
- plaintext developer profile or qualified encrypted-at-rest vault profile, selected explicitly;
- local SQLite operational state;
- no network required;
- caller identity is explicitly configured by the embedding application;
- suitable for developer tools and single-process apps.

Risks:

- host application compromise can access embedded process memory;
- process-local lifetime must not be confused with durable writer coordination.

## 3. Profile B — Local daemon

Use case:

- multiple local agents/apps share one user-owned memory service.

Properties:

- local IPC/loopback transport;
- authenticated local client/session identity;
- one governed writer per vault;
- CLI/MCP/SDK clients share server contracts;
- local diagnostics and recovery;
- qualified encrypted-at-rest vault profile for sensitive local use;
- optional local model/vector adapters.

This is the primary v1 user experience.

## 4. Profile C — Self-hosted single-node

Use case:

- user/team runs Morize on their own server.

Properties:

- server authentication;
- TLS where networked;
- externalized backup path;
- configurable storage volume;
- optional Postgres/object storage adapters only when justified;
- explicit network/connector policies;
- no Morize-operated control plane required.

This profile may be operationally more complex than local daemon mode.

## 5. Profile D — Self-hosted team

Use case:

- organization/team shares governed memory.

Additional requirements:

- tenant/team identity;
- RBAC/ABAC policy;
- audit;
- invitation/membership lifecycle;
- tenant-isolated retrieval;
- rate/resource limits;
- backup/restore;
- optional SSO/SCIM only through dedicated specifications;
- side-channel tests.

## 6. Profile E — Managed Morize service

Future commercial surface.

Possible managed capabilities:

- hosted memory service;
- synchronization;
- managed backup/disaster recovery;
- managed search/embedding/reranking;
- connectors;
- organization administration;
- enterprise identity;
- audit/compliance tooling;
- observability;
- high availability;
- support/SLA.

This profile may be paid.

It is not authorized for production launch by the current planning branch.

## 7. Profile F — Hybrid / bring-your-own-provider

Users may combine Morize with external:

- model APIs;
- embedding services;
- vector stores;
- graph stores;
- object stores;
- databases;
- observability;
- cloud infrastructure.

Each adapter has explicit:

- endpoint/provider identity;
- credential scope;
- data classes allowed to leave Morize;
- cost ownership;
- timeout/retry policy;
- rate limits;
- fallback behavior;
- telemetry behavior;
- retention assumptions;
- exit/migration path.

## 8. Semantic invariants across profiles

All profiles preserve:

- typed stable identities;
- memory version semantics;
- valid/knowledge time meaning;
- provenance;
- conflict/supersession meaning;
- mutation action vocabulary;
- policy revision evidence;
- export semantics;
- unknown-outcome honesty.

A managed profile may scale these semantics but cannot redefine them silently.

## 9. Storage boundary

Initial local implementation:

```text
Markdown MVF
+ SQLite operational/search state
+ content-addressed local blobs
```

Future hosted implementations may use different physical stores.

Public APIs therefore operate on Morize domain objects, not SQLite row identities or file paths.

## 10. Network posture

### Embedded/local daemon

Default external network posture may be disabled until an explicit connector/provider requires it.

### Self-hosted/managed

Network access is unavoidable, but egress is still explicit by connector/provider class.

No local error automatically enables a different remote provider.

## 11. Secrets

Profiles must use secret handles/configured secret stores appropriate to deployment. When the encrypted-at-rest vault profile is enabled, encryption keys are stored/derived through a separate key-management path rather than inside the vault they protect.

Rules:

- secrets are not ordinary memory;
- connector/provider credentials have exact scope;
- logs redact sensitive material;
- secret rotation/revocation does not require rewriting canonical memory.

## 12. Backup and disaster recovery

Each persistent profile defines:

- canonical backup contents;
- operational state backup;
- encryption requirements;
- retention;
- recovery point objective where applicable;
- recovery time objective where applicable;
- restore validation;
- corrupt/partial backup behavior;
- cross-version restore compatibility.

Commercial hosted RPO/RTO commitments require a separate SLA specification.

## 13. High availability

HA is not a v1 local-core requirement.

Before introducing multi-writer/distributed HA, a controlled SpecGrain must define:

- consensus/coordination model;
- writer identity;
- linearizability/eventual consistency expectations;
- idempotency across nodes;
- partition behavior;
- conflict resolution;
- failover evidence;
- cost.

Do not generalize the local single-writer contract into distributed claims without proof.

## 14. Synchronization

Synchronization is different from backup.

Sync design must define:

- operation identity;
- version vectors/frontiers or other conflict evidence;
- offline edits;
- deletions/redactions;
- stale device behavior;
- branch behavior;
- attachment/blob transfer;
- encryption;
- peer/server trust.

Implicit last-write-wins is not acceptable for protected durable memory unless a narrowly scoped data type explicitly selects it.

## 15. Observability

Local profiles:

- local structured diagnostics;
- no hosted telemetry required.

Managed profiles:

- metrics/logs/traces must be privacy-aware;
- memory content is not analytics payload by default;
- tenant identifiers and sensitive metadata require minimization;
- customer-facing usage accounting must be auditable separately from model memory truth.

## 16. Resource isolation

Shared profiles require quotas for:

- storage;
- query size;
- graph traversal;
- concurrent requests;
- connector work;
- model/embedding calls;
- import size;
- export size/rate;
- background indexing.

Resource quota failure must not corrupt canonical state.

## 17. Commercial cost ownership

For each managed dependency:

```text
provider
service
required_for_which_profile
unit_cost
expected_scale
budget_cap
billing_owner
customer_pricing_linkage
degradation_behavior
exit_plan
```

Founder-zero-burn applies to the current pre-revenue development path. Revenue-backed production spend is allowed when explicitly approved.

## 18. Promotion between profiles

A user moving from local to self-hosted/managed should not need semantic conversion beyond documented storage/import migration.

Goals:

- stable export/import;
- stable identity where safe;
- no provider-specific proprietary canonical facts;
- explicit credential/connector re-binding;
- documented policy changes.

## 19. Deployment qualification

Every supported profile gets its own release evidence.

Do not infer that passing local-daemon tests proves:

- multi-user isolation;
- cloud network security;
- HA;
- billing correctness;
- sync;
- regional compliance.

Each is separately qualified.
