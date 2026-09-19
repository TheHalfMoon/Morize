# Zero-Cost Operating Contract

## Goal

A normal Morize user must be able to build, run, test, back up, restore, search, retrieve, inspect, import, export, and connect local agents **without paying Morize or any required third-party service**.

This means zero required service fees, not zero physical resource usage. The user still supplies their own computer, storage, electricity, and optional network connection.

## Binding product rules

### ZC-1 — No mandatory paid API

The default product cannot require:

- OpenAI;
- Anthropic;
- Google model APIs;
- Jev / TypeSafe AI;
- Moorcheh cloud;
- managed vector databases;
- hosted graph databases;
- hosted observability;
- paid authentication;
- paid sync;
- any Morize cloud account.

### ZC-2 — No mandatory model

Morize remains useful with every model adapter disabled.

Without an LLM, users retain:

- exact lookup;
- metadata queries;
- FTS/BM25;
- temporal queries;
- explicit graph traversal;
- manual and deterministic memory creation;
- explicit updates/supersession;
- ACL/policy enforcement;
- provenance;
- import/export;
- history;
- backup/restore;
- MCP reads and governed proposals;
- index rebuild.

### ZC-3 — Local inference is optional

If a user wants AI-assisted extraction, classification, summarization, or reranking, Morize supports user-owned local runtimes first.

Candidate adapters:

- Ollama;
- llama.cpp;
- embedded/local ONNX or Rust-native runtimes where qualified.

Models are separate artifacts with their own licenses, hashes, resource requirements, and trust records.

### ZC-4 — SQLite-first infrastructure

The first supported deployment cannot require Redis, Kafka, Neo4j, Postgres, Qdrant, Elasticsearch, Kubernetes, or Docker.

Those may become optional adapters or deployment choices later. The single-user local path uses ordinary files plus SQLite.

### ZC-5 — Vector search is optional

A vector extension or local vector engine may be enabled after benchmarks show value. Exact/FTS/graph/temporal retrieval remains available without it.

### ZC-6 — Network denial remains functional

`morize doctor --offline` must verify that a vault can perform core operations while external network access is unavailable.

Tests for the trusted core should be hermetic and must not silently download models or fixtures.

### ZC-7 — Open source build path

A developer can build Morize from source with public open-source toolchains. Release artifacts are convenience, not a proprietary gate.

### ZC-8 — Export has no paywall

Full user-owned canonical memory and supported evidence metadata can be exported without a subscription.

### ZC-9 — No hosted-only “real product”

Self-host/local operation is the product, not a crippled demo. Optional hosted services may improve convenience in the future but cannot be required to unlock canonical memory correctness.

### ZC-10 — CI independence

Public CI may use free hosted runners when available, but the project cannot depend on a paid CI feature for correctness. Required checks must also be runnable locally.

## Default local profile

```text
Profile: local-zero-cost
Network: denied by default
Canonical memory: Markdown vault
Operational DB: SQLite
Full text: SQLite FTS5
Graph: SQLite relations + bounded traversal
Vector: disabled
Model: disabled
Telemetry: disabled
Cloud sync: disabled
Authentication: local OS / local daemon profile
MCP: loopback/local transport with explicit auth where transport requires it
```

## Optional enhanced-local profile

```text
Profile: local-ai
Everything from local-zero-cost
+ local model runtime
+ optional local embeddings
+ optional local reranker
+ optional local vector projection
```

It still requires no per-request payment.

## Remote profile

Remote inference, sync, connectors, or hosted deployment are strictly optional and configured explicitly.

A remote adapter must expose:

- provider identity;
- endpoint;
- model identity;
- data classes allowed to leave the device;
- credential binding;
- cost/usage observability where available;
- fallback policy.

A local failure must never silently authorize a paid remote fallback.

## Zero-Cost Conformance Gate

Every release candidate must prove:

1. clean install/build from documented open-source tooling;
2. fresh vault initialization;
3. create/update/supersede/contradict/query operations;
4. FTS and graph retrieval;
5. temporal query;
6. export/import round trip;
7. delete and rebuild derived indexes;
8. restart/crash recovery fixtures;
9. MCP local smoke test;
10. at least one agent integration smoke test that uses no paid memory service;
11. full core test suite with external network denied;
12. no required environment variable representing a paid service credential.

Release evidence records the exact commands and versions used.

## Cost-budget policy

Optional features may expose resource cost estimates such as CPU time, RAM, disk, tokens, or remote spend. Morize never equates higher spend with higher trust.

If a future contributor proposes a mandatory hosted dependency, the change is architecture-breaking and must be rejected unless this contract is explicitly changed by project governance.

## Founder constraint

The initial project requirement is that maintaining and using Morize should not force the founder to pay for runtime services. Architecture and roadmap decisions must preserve that constraint.
