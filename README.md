# Morize

> **Models reason. Agents act. Morize remembers.**

Morize is an open-source, local-first memory operating system for AI agents, applications, and humans.

It is designed to answer more than “what text looks similar?” Morize tracks **what is known, where it came from, when it was true, what changed, what conflicts, who may see it, and why a memory was retrieved**.

## Product promise

- **Founder-zero-burn development.** The pre-revenue engineering path must not force the founder into mandatory recurring service fees. This is not a promise that future managed Morize services will be free to users.
- **Useful without an LLM.** Exact lookup, FTS/BM25, temporal filters, graph traversal, policy, provenance, versioning, import/export, and deterministic retrieval work locally without model inference.
- **Local-first and offline-capable.** Networking is an explicit optional capability, never a hidden fallback.
- **User-owned truth.** Canonical durable memory is inspectable, exportable, versioned, and recoverable without a Morize cloud.
- **Typed decisions, not free-form authority.** Memory mutations are constrained actions such as STORE, UPDATE, SUPERSEDE, CONTRADICT, MERGE, EXPIRE, FORGET, REDACT, QUARANTINE, IGNORE, and REQUIRE_REVIEW.
- **Evidence before confidence.** Confidence may guide escalation, but never grants authority.
- **One memory plane, many agents.** Codex, Claude Code, Cursor, Gemini CLI, OpenCode, Hermes, custom agents, applications, and MCP clients can share governed memory without sharing unrestricted authority.
- **Derived indexes are disposable.** Vector, graph, keyword, and summary indexes can be rebuilt. They never silently become canonical truth.
- **Open source.** The project is intended to remain usable as a complete local product without a proprietary hosted dependency.

## Why Morize

Morize owns the lifecycle between an observation and durable, retrievable, governed memory.

```text
Observation
    |
    v
Privacy / taint / provenance gate
    |
    v
Memory Candidate
    |
    v
Typed Decision Engine
    |
    +--> IGNORE / QUARANTINE / REQUIRE_REVIEW
    |
    v
Governed Memory Writer
    |
    +--> Canonical Memory Vault
    +--> Immutable Evidence Ledger
    |
    v
Rebuildable Projections
    +--> FTS / BM25
    +--> Temporal graph
    +--> Optional vectors
    +--> Optional summaries
    |
    v
Context Compiler
    |
    v
Agent / App / Human
```

## Signature capabilities

- **Temporal Truth Ledger** — separate valid time from observed/committed time.
- **Evidence Graph** — propositions remain linked to support, contradiction, supersession, and source lineage.
- **Memory Firewall** — secrets, sensitive data, untrusted content, and cross-scope data are classified before promotion.
- **Memory Branches** — fork, diff, merge, snapshot, and rewind active memory state without rewriting evidence history.
- **Explainable Recall** — show why each result was selected.
- **Memory Lab** — reproducible evaluation for recall, temporal accuracy, contradictions, stale-memory resistance, provenance, poisoning, forgetting/redaction, latency, and offline operation.

## Founder-zero-burn development stack

The planned default stack is intentionally portable:

- Rust core and CLI;
- SQLite for operational metadata, FTS, temporal/graph projections, and local indexes;
- human-readable Markdown for canonical durable memory;
- local filesystem blobs for large immutable artifacts;
- optional Git mirror/version export;
- optional local inference through Ollama, llama.cpp, or qualified embedded runtimes;
- optional local vector search only after measured need.

Remote providers remain explicit adapters rather than hidden correctness dependencies. Future Morize-managed cloud, team, enterprise, inference, synchronization, support, and operational services may be commercial and charged to users.

## Commercial boundary

The repository is open source under Apache-2.0. Morize may later charge for managed cloud, hosted memory, synchronization, team/enterprise governance, managed inference/search, premium connectors, support, SLA, migration, and other operated services. The current cost constraint is on founder burn before revenue, not on future user pricing.

## Planning documents

- [Product thesis](docs/PRODUCT_THESIS.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Canonical data model](docs/DATA_MODEL.md)
- [v1 requirements](docs/REQUIREMENTS.md)
- [Typed memory decisions](docs/TYPED_DECISION_MODEL.md)
- [API and compatibility policy](docs/API_COMPATIBILITY.md)
- [Deployment profiles](docs/DEPLOYMENT_PROFILES.md)
- [Synchronization model](docs/SYNC_MODEL.md)
- [Operations and observability](docs/OPERATIONS.md)
- [Founder cost and commercial boundary](docs/ZERO_COST_CONTRACT.md)
- [Commercial service architecture](docs/COMMERCIAL_MODEL.md)
- [Planning governance: SpecGrain + Diffcipline](docs/PLANNING_GOVERNANCE.md)
- [Execution master plan](docs/EXECUTION_MASTER_PLAN.md)
- [Risk register](docs/RISK_REGISTER.md)
- [Master plan gap review](docs/GAP_REVIEW.md)
- [Source ledger](docs/SOURCE_LEDGER.md)
- [Donor and provenance policy](docs/DONOR_AND_PROVENANCE.md)
- [Threat model](docs/THREAT_MODEL.md)
- [Benchmark plan](docs/BENCHMARK_PLAN.md)
- [Implementation roadmap](docs/ROADMAP.md)
- [Open-source governance](GOVERNANCE.md)

## Status

Morize is in **foundation planning**. The repository starts by freezing contracts, evidence rules, source provenance, founder-cost constraints, commercial boundaries, and delivery governance before implementation.

No benchmark-superiority claim is accepted without reproducible evidence.

## License

Morize is released under the [Apache License 2.0](LICENSE). The Apache-2.0 source license does not prevent future paid Morize-hosted or managed services. Third-party code remains subject to its original obligations and Morize's source-admission process.
