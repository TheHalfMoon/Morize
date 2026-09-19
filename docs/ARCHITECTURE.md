# Morize Architecture

## 1. Architectural objective

Build a Rust-first, local-first memory system whose correctness-sensitive behavior is deterministic and inspectable, while probabilistic models remain replaceable adapters.

```text
Sources / Agents / Apps / Files / Web / Tools
                    |
                    v
             Observation Bus
                    |
                    v
       Normalization + Content Identity
                    |
                    v
         Memory Firewall / Taint Gate
                    |
                    v
             Candidate Extractor
                    |
                    v
          Typed Decision Engine
                    |
          +---------+---------+
          |                   |
   deterministic rules   optional local model
          |                   |
          +---------+---------+
                    v
           Policy / Scope Gate
                    |
                    v
          Governed Memory Writer
          |         |          |
          v         v          v
     Memory Vault  Evidence   Operational
      Markdown      Ledger      SQLite
          |                     |
          +----------+----------+
                     v
             Derived Projections
       FTS | graph | optional vectors
                     |
                     v
             Retrieval Router
                     |
                     v
             Context Compiler
                     |
                     v
             Agent / App / UI
```

## 2. Trust boundaries

### Deterministic trusted core

Owns:

- identities and digests;
- schemas and validation;
- scope and capability policy;
- provenance/taint propagation;
- temporal invariants;
- mutation state machine;
- idempotency;
- crash recovery and reconciliation;
- index invalidation;
- authorization before retrieval disclosure;
- import/export validation;
- limits and resource budgets.

### Probabilistic adapters

May propose:

- memory candidates;
- entity links;
- summaries;
- contradiction hypotheses;
- relevance signals;
- classifications;
- suggested graph edges.

Their output is parsed into bounded typed structures and independently validated.

### External integrations

Agents, MCP clients, plugins, browsers, web pages, files, repositories, remote APIs, and connector outputs are untrusted sources. Connectivity never grants memory authority.

## 3. Canonical storage

### 3.1 Memory Vault

Canonical durable knowledge is represented as human-readable Markdown records in a user-owned vault.

Conceptual layout:

```text
.morize/
  vault/
    user/
    projects/
    teams/
    organizations/
  blobs/
    sha256/
  state/
    control.sqlite
  exports/
```

The exact on-disk format is versioned as the Morize Vault Format (MVF).

### 3.2 Operational store

SQLite stores:

- record/version identities;
- writer state;
- expected digests;
- scopes and ACL metadata;
- mutation intents;
- idempotency keys;
- conflict/supersession links;
- temporal intervals;
- reconciliation state;
- audit references;
- derivative-index generations.

It does not silently replace the canonical Markdown body.

### 3.3 Evidence ledger

Evidence records are append-oriented and digest-linked. The local chain supports integrity, ordering, and reconciliation checks under the declared storage/writer threat model; it is **not** described as tamper-proof against an attacker who can rewrite the entire vault and its trust anchors. They bind:

- observation identity;
- source identity and revision;
- observed time;
- source digest/span;
- principal;
- trust/taint classification;
- transformation lineage;
- mutation decision;
- policy/version;
- resulting memory version.

### 3.4 Blobs

Large immutable inputs are content-addressed by digest. A memory points to a blob instead of duplicating it.

## 4. Memory data model

### Observation

Raw bounded evidence from a source.

### Proposition

A normalized claim that can be supported, contradicted, or superseded.

### MemoryRecord

A stable logical identity for one governed memory lineage. It owns identity, scope ownership, creation metadata, and lifecycle status; it does not hold one mutable branch pointer or silently overwrite historical truth.

### MemoryVersion

An immutable version containing the durable proposition/content, temporal fields, sensitivity, provenance, support/contradiction links, policy/decision references, and content digest for one logical memory.

The canonical field-level contract is defined in `docs/DATA_MODEL.md`.

### MemoryRelation

Explainable typed edge. Every inferred edge records why it exists and which evidence supports it.

### ContextBundle

A retrieval result destined for one agent/app invocation. It binds exact memory/source versions, selection reasons, omissions, byte/token budgets, policy revision, and bundle digest.

## 5. Bi-temporal truth

Morize stores two distinct notions of time:

- **valid time** — when the proposition was true in its domain;
- **transaction/knowledge time** — when Morize observed or committed it.

Queries include:

- current truth;
- truth as-of valid time;
- knowledge as-of observation/commit time;
- complete change timeline.

This prevents “latest write wins” from erasing historical meaning.

## 6. Typed memory mutation state machine

All durable changes resolve to a closed action set:

```text
STORE
UPDATE
MERGE
SUPERSEDE
CONTRADICT
EXPIRE
FORGET
REDACT
QUARANTINE
IGNORE
REQUIRE_REVIEW
```

A candidate cannot invent a new authority-bearing action through model text.

Mutation lifecycle:

```text
CANDIDATE
  -> VALIDATED
  -> DECIDED
  -> AUTHORIZED
  -> PREPARED
  -> WRITING
  -> RECONCILING
  -> COMMITTED | REJECTED | UNKNOWN_OUTCOME
```

`UNKNOWN_OUTCOME` blocks dependent writes until reconciliation.

## 7. Retrieval hierarchy

Morize uses the cheapest trustworthy retrieval method before escalating:

1. exact ID/path/alias lookup;
2. scope/metadata filters;
3. SQLite FTS5/BM25;
4. temporal filtering;
5. explicit graph traversal;
6. optional local semantic vectors;
7. optional local reranking;
8. optional synthesis.

Every stage is bounded. A missing embedding model or vector extension must never make ordinary memory unusable.

## 8. Evidence graph

The initial graph is stored in SQLite adjacency/relation tables and queried with bounded traversal/recursive CTEs. No external graph database is required.

Graph types include:

- SUPPORTS;
- CONTRADICTS;
- SUPERSEDES;
- DERIVED_FROM;
- CAUSED_BY;
- LED_TO;
- DEPENDS_ON;
- RELATED_TO;
- MEMBER_OF;
- ABOUT;
- SOURCE_OF.

Graph databases may be optional adapters later only if benchmarks prove a material need.

## 9. Memory branches

Morize versions are immutable, enabling logical branches without rewriting history.

```text
main
  |
  +-- experiment/a
  +-- agent/research
  +-- project/migration
```

Operations:

- branch;
- diff;
- merge proposal;
- selective apply;
- snapshot;
- rewind projection.

The evidence ledger remains append-only. A rewind changes the active projection, not past evidence.

## 10. Context compiler

```text
intent
 -> required evidence classes
 -> source routing
 -> authorization filter
 -> retrieve
 -> freshness/taint filter
 -> deduplicate
 -> contradiction handling
 -> rank
 -> sufficiency check
 -> budget
 -> ContextBundle
```

The compiler prefers exact source evidence over derived summaries and records omissions/truncation explicitly.

## 11. Memory firewall

Before promotion or disclosure, classify:

- secret;
- credential;
- PII;
- sensitive domain data;
- confidential project data;
- untrusted web/document data;
- prompt-injection indicators;
- source trust;
- cross-scope risk.

Initial rules:

- secrets are never ordinary plaintext memory;
- untrusted text cannot promote itself to trusted policy;
- transformations do not automatically declassify tainted content;
- source deletion/revocation invalidates dependent derived indexes;
- retrieval authorization occurs before result disclosure.

## 12. Model/runtime adapters

The default core requires no model.

Optional local adapters:

- Ollama;
- llama.cpp;
- qualified embedded ONNX/Candle/mistral.rs-style runtimes.

Optional remote providers may exist behind explicit configuration, but cannot be required by build, CI, boot, import/export, exact search, policy, or canonical storage.

## 13. Interfaces

### CLI

```bash
morize init
morize daemon
morize remember
morize recall
morize search
morize timeline
morize explain
morize diff
morize forget
morize redact
morize export
morize import
morize connect
morize doctor --offline
morize bench
```

### MCP

Expose a compact least-authority surface:

- inventory/search/read;
- explain/timeline;
- propose memory;
- propose correction;
- propose forget/redact;
- context bundle.

Administrative mutation/ACL operations remain separate and are never discoverable merely because a client can connect.

### SDKs

Rust first, followed by Python and TypeScript generated or wrapped from stable schemas.

## 14. Crate ownership

Do not create empty crates before evidence requires separation. Target ownership:

- `morize-core` — pure types, identities, validation;
- `morize-store` — MVF, SQLite, blobs, recovery;
- `morize-policy` — scopes, ACL, taint, firewall;
- `morize-memory` — candidate/decision/mutation lifecycle;
- `morize-retrieval` — FTS, graph, optional semantic routing;
- `morize-mcp` — MCP adapter;
- `morize-server` — local daemon/API;
- `morize-cli` — user interface.

Split only when implementation evidence establishes an ownership boundary.

## 15. Reliability

Required invariants:

- one governed writer per vault;
- compare-and-swap against expected versions/digests;
- durable idempotency;
- crash-safe mutation journal;
- deterministic startup reconciliation;
- no silent partial success;
- bounded queues/results/retries/timeouts;
- backup/restore proof;
- index deletion/rebuild proof;
- schema migration rollback/recovery plan.

## 16. Performance philosophy

Optimize in this order:

1. correct bounded deterministic retrieval;
2. incremental indexes;
3. zero-copy/content-addressed reads where useful;
4. local FTS/graph;
5. optional vectors only when measured;
6. optional model work only when it improves outcome quality enough to justify resource cost.

## 17. Architecture invariants

```text
MODEL_OUTPUT != VERIFIED_FACT
MEMORY_CANDIDATE != DURABLE_MEMORY
RETRIEVAL_SCORE != SOURCE_AUTHORITY
CONFIDENCE != AUTHORITY
INDEX != CANONICAL_TRUTH
LOCALHOST != AUTHENTICATION
MCP_CAPABILITY != MORIZE_AUTHORITY
DELETE_INDEX != DELETE_MEMORY
REWIND != ERASE_HISTORY
FOUNDER_ZERO_BURN != USER_FREE_FOREVER
```


## 18. Deployment abstraction

The semantic architecture is independent from deployment profile.

Supported planning profiles are defined in `docs/DEPLOYMENT_PROFILES.md`:

- embedded local;
- local daemon;
- self-hosted single-node;
- self-hosted team;
- future managed Morize service;
- hybrid / bring-your-own-provider.

SQLite/Markdown are initial local implementation choices, not public semantic APIs.

## 19. Compatibility boundary

Persistent formats, APIs, MCP schemas, CLI machine output, and SDK contracts evolve under `docs/API_COMPATIBILITY.md`.

Persistent compatibility is correctness-sensitive. Unsupported newer vault formats fail safely or enter an explicitly supported read-only mode; writers never guess.

## 20. Commercial boundary

Morize's open-source source code is Apache-2.0.

Future managed services may charge for operations such as:

- hosted memory;
- synchronization;
- managed backup;
- team/enterprise administration;
- managed inference/search;
- connectors;
- high availability;
- support/SLA.

Billing and entitlements operate above canonical memory semantics.

`PAID_PLAN != DIFFERENT_MEMORY_TRUTH`

## 21. Founder-cost boundary

The current pre-revenue engineering path cannot require an unapproved recurring founder-paid service.

This does not constrain future user pricing.

A proposed required paid dependency must state:

- cost owner;
- budget cap;
- why local/free options are insufficient;
- customer-revenue linkage where applicable;
- degradation/fallback;
- exit/migration plan.

## 22. Delivery governance

Architecture implementation follows `docs/PLANNING_GOVERNANCE.md`.

SpecGrain controls decomposition/readiness. Diffcipline controls exact-diff/verification proof.

Roadmap prose does not grant implementation authority. Only a dependency-eligible bounded Grain with explicit acceptance/evidence may enter execution.
