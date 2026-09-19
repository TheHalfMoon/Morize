# Roadmap A — Foundation Through Deterministic Retrieval

## P0 — Foundation and source freeze

Outcome: implementation begins with product, cost, source, security, and evaluation boundaries already fixed.

Work:
- freeze product thesis and non-goals;
- freeze zero-cost contract;
- freeze canonical-vs-derived storage boundary;
- freeze typed decision vocabulary;
- pin primary donor/reference states;
- freeze threat model;
- freeze benchmark policy;
- establish open-source governance and third-party provenance.

Gate:
- no mandatory paid API, cloud service, vector service, graph service, or model;
- planning documents are internally consistent.

## P1 — Rust deterministic kernel

Outcome: dependency-light core with no model or network requirement.

Work:
- stable IDs and content digests;
- bounded strings, collections, and timestamps;
- scope identities;
- Observation;
- Proposition;
- MemoryCandidate;
- MemoryRecord;
- MemoryVersion;
- MemoryRelation;
- EvidenceRef;
- ContextBundle;
- typed decision envelope;
- versioned configuration;
- deterministic serialization;
- stable error taxonomy;
- property and fuzz tests for parsers/invariants.

Gate:
- Linux, macOS, and Windows build/test;
- deterministic fixtures;
- malformed or unbounded inputs fail closed;
- no network/model required.

## P2 — Morize Vault Format and reliable writer

Outcome: human-readable durable memory with deterministic recovery.

Work:
- versioned MVF directory and Markdown record format;
- safe vault initialization/discovery;
- SQLite operational schema and migrations;
- content-addressed blob storage;
- one governed writer per vault;
- durable idempotency;
- expected-version compare-and-swap;
- prepared and terminal mutation state;
- startup reconciliation;
- external user-edit detection;
- backup/restore;
- corruption detection;
- index-loss recovery.

Gate:
- injected failures around each mutation boundary converge to one explainable state;
- no silent overwrite;
- canonical memory survives deletion of every derived index.

## P3 — Scopes, policy, and Memory Firewall

Outcome: memory cannot become a permission bypass.

Work:
- run/session/agent/project/user/team/organization/reference scopes;
- principal and capability model;
- read/write/promotion policy;
- source trust and taint;
- sensitivity classes;
- safe logging;
- deterministic sensitive-content filters;
- quarantine/review state;
- authorization before retrieval disclosure;
- policy revision binding.

Gate:
- untrusted fixtures cannot promote themselves into trusted policy;
- cross-scope access tests pass.

## P4 — Temporal Truth Ledger and evidence graph

Outcome: change is represented without erasing history.

Work:
- valid-time intervals;
- observed/committed-time intervals;
- current/as-of/history queries;
- SUPPORTS;
- CONTRADICTS;
- SUPERSEDES;
- DERIVED_FROM;
- explicit vs inferred edge class;
- bounded SQLite graph traversal;
- conflict sets;
- multi-context coexistence;
- deterministic duplicate/update/supersession rules;
- graph provenance;
- timeline and diff commands.

Gate:
- temporal and contradiction suites pass without an LLM or external graph database.

## P5 — Deterministic retrieval and Context Compiler

Outcome: useful recall exists before semantic vectors or model inference.

Work:
1. exact ID/path/alias lookup;
2. scope/metadata filtering;
3. SQLite FTS5/BM25;
4. incremental invalidation/rebuild;
5. temporal filtering;
6. graph traversal;
7. duplicate/conflict-aware assembly;
8. byte and token-budget accounting;
9. source-bound ContextBundle;
10. machine-readable retrieval explanation;
11. sufficiency/abstention state;
12. large-vault boundedness tests.

Gate:
- recall, timeline, graph, and context assembly pass with network blocked, models disabled, and vector search absent.

## Dependency spine

```text
P0 -> P1 -> P2 -> P3 -> P4 -> P5
```

No UI, hosted service, model integration, vector database, or external graph database precedes this spine.
