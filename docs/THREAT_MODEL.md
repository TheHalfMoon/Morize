# Morize Threat Model

## Security objective

Morize stores long-lived context that can influence future agents. A compromised memory system can quietly become more dangerous than a compromised single chat because poisoned state may survive, spread across agents, and look authoritative later.

Morize therefore treats memory as both a **data-security surface** and an **agent-control surface**.

## Assets to protect

- canonical memory content;
- provenance and evidence lineage;
- user/project/team scope boundaries;
- sensitive information;
- credentials and secret handles;
- policy and ACL state;
- mutation history;
- memory versions and temporal history;
- agent/plugin identities;
- backups and exports;
- derived indexes where they may reveal protected content.

## Trust assumptions

Trusted only after validation:

- Morize-owned deterministic core;
- exact local configuration loaded through validated schemas;
- authenticated local principal identity;
- exact admitted source/runtime artifacts.

Untrusted by default:

- model output;
- MCP arguments;
- plugin payloads;
- web pages;
- documents;
- imported memory archives;
- Markdown/front matter;
- external connectors;
- remote responses;
- agent self-reports;
- summaries;
- embeddings;
- graph inferences;
- user-editable files until reconciled.

## Primary threat classes

### T1 — Memory poisoning

An untrusted page, document, tool result, or agent output tries to become durable trusted instruction or policy.

Required controls:

- source trust and taint metadata;
- candidate-before-durable lifecycle;
- protected memory classes;
- explicit promotion policy;
- no authority from repeated model statements;
- visible conflicts instead of silent overwrite.

### T2 — Prompt injection persistence

Malicious instructions are stored in memory and later injected into unrelated sessions.

Controls:

- distinguish data from executable/instruction memory;
- isolate untrusted instruction-like content;
- prevent untrusted sources from creating policy memories;
- render provenance/trust labels into context;
- context compiler excludes unsafe classes by default.

### T3 — Scope leakage

A user/agent receives memory from another project, user, team, or organization without authorization.

Controls:

- authorization before ranking/disclosure;
- scope-bound index keys;
- project-scoped indexes by default;
- no cross-scope semantic search before ACL filtering;
- leakage tests with indistinguishable forbidden records.

### T4 — Secret capture

Credentials, tokens, private keys, or secret-bearing tool output are persisted as ordinary memory.

Controls:

- memory firewall before durable promotion;
- secret-pattern and entropy scanners;
- brokered secret references instead of plaintext;
- safe logging defaults;
- redact-on-import capability;
- monotonic taint unless independent clean evidence exists.

### T5 — Stale or contradicted truth

Old memory continues to steer agents after reality changes.

Controls:

- bi-temporal fields;
- freshness policy;
- explicit supersession;
- contradiction sets;
- source-revision invalidation;
- current-truth query distinct from historical query.

### T6 — Model authority escalation

A model outputs a high-confidence mutation or forged role/scope and attempts to bypass policy.

Controls:

- closed typed actions;
- deterministic authorization;
- principal identity never accepted from model-controlled arguments;
- confidence never grants permission;
- policy/version identity bound to every mutation.

### T7 — Partial or ambiguous writes

Crash, disk exhaustion, process death, or power loss leaves Markdown, SQLite, and evidence state inconsistent.

Controls:

- single governed writer;
- PREPARED state before mutation;
- expected version/digest;
- durable idempotency;
- cross-store read-back;
- restart reconciliation;
- `UNKNOWN_OUTCOME` blocks dependent mutation.

### T8 — User-edit races

A human edits a canonical Markdown file while Morize writes an older observed version.

Controls:

- commit-time digest/version revalidation;
- compare-and-swap;
- user edit becomes reconciliation input, never silently overwritten.

### T9 — Path and filesystem attacks

Traversal, symlink/reparse/junction tricks, special files, mount aliases, or archive extraction escapes access boundaries.

Controls:

- bounded normalized paths;
- resolved target identity;
- allowed-root checks;
- deny special files unless explicitly supported;
- safe archive extraction;
- platform-specific tests;
- fail closed when identity cannot be preserved.

### T10 — MCP/plugin identity spoofing

A client claims another principal or advertises a capability as if it grants Morize authority.

Controls:

- authenticated transport/session identity;
- server-owned principal mapping;
- MCP capability metadata is descriptive only;
- exact binding/version/digest for adapters;
- revoked integrations invalidate cached authority.

### T11 — Retrieval side channels

Counts, timing, ranking, errors, graph degrees, or semantic scores reveal records the caller cannot read.

Controls:

- authorize candidate corpus before retrieval;
- bounded generic errors;
- no forbidden record counts;
- side-channel-focused tests for shared deployments.

### T12 — Malicious import/export

A memory archive contains oversized content, invalid schemas, paths, forged provenance, executable payloads, or conflicting IDs.

Controls:

- versioned import schema;
- bounded sizes/counts/nesting;
- content digests;
- imports create candidates/quarantine when trust cannot be established;
- imported provenance is descriptive unless independently verified.

### T13 — Graph explosion

Adversarial edges create unbounded traversal, expensive cycles, or misleading inferred relationships.

Controls:

- bounded depth/fan-out/result count;
- explicit vs inferred edge class;
- cycle-aware traversal;
- evidence requirement for inferred high-impact edges;
- resource budgets.

### T14 — Denial of service

Huge files, conversations, graphs, retries, embeddings, queues, or model calls exhaust CPU/RAM/disk.

Controls:

- finite limits everywhere;
- backpressure;
- per-operation budgets;
- bounded concurrency;
- cancellation;
- disk quotas/warnings;
- graceful degradation to deterministic search.

### T15 — Supply-chain compromise

A dependency, model, installer, release binary, generated file, or donor source is compromised.

Controls:

- exact source pins;
- lockfiles;
- cargo-deny / RustSec and source policy;
- SBOM;
- release checksums/attestations;
- model hashes/licenses;
- minimal dependencies;
- donor provenance records.

### T16 — Destructive forgetting claims

The system says data was erased when copies remain in exports, backups, Git history, remote systems, or agent transcripts.

Controls:

- define active-memory forgetting separately from external-copy erasure;
- report deletion scope accurately;
- backup/export retention policy;
- tombstone/minimal audit semantics without retaining prohibited plaintext.

## Privacy defaults

- local indexes stay local;
- telemetry disabled by default;
- no prompt/file upload for analytics;
- remote adapters require explicit enablement;
- cross-project retrieval denied by default;
- local AI is preferred for sensitive content;
- exports are user initiated.

## Security testing gates

Before v1:

- poisoning corpus;
- prompt-injection persistence tests;
- scope-leak tests;
- secret persistence tests;
- stale/supersession/contradiction tests;
- crash-at-every-mutation-boundary tests;
- symlink/path race tests on supported platforms;
- malicious MCP client tests;
- malicious import/archive tests;
- graph resource-exhaustion tests;
- backup/restore and index-rebuild tests;
- offline/no-network test suite;
- dependency and release supply-chain checks.

## Security rule

A memory system should never make unsafe information more authoritative merely because it remembered it longer.
