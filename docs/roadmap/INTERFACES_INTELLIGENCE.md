# Roadmap B — Interfaces, Portability, and Optional Intelligence

## P6 — CLI, daemon, MCP, and stable APIs

Outcome: humans and agents use one governed engine.

Work:
- local daemon lifecycle and health;
- stable local API schemas;
- CLI for init, remember, recall, search, timeline, explain, export, import, doctor, and bench;
- authenticated MCP transport profile;
- least-authority read/search/explain/timeline tools;
- proposal-based durable changes;
- separate administrative policy/ACL surface;
- authenticated request principal binding;
- stale integration-binding tests;
- `morize doctor --offline`.

Gate:
- two independent local clients can share permitted memory without receiving administrative authority.

## P7 — Import, export, and agent integrations

Outcome: adoption is easy and lock-in is low.

Work:
- versioned Morize portable export;
- plain Markdown export;
- Memanto/OKF compatibility where technically sound;
- Mem0 migration adapter;
- Letta migration adapter;
- Supermemory/OpenViking migration research and adapters where public formats permit;
- Codex integration;
- Claude Code integration;
- OpenCode integration;
- Gemini CLI integration;
- Cursor-compatible integration;
- Hermes integration;
- generic MCP and Agent Skills path;
- stable project identity derived from repository identity instead of transient working directory.

Gate:
- at least two major agent harnesses share the same permitted project memory;
- export/import round trips require no paid service.

## P8 — Optional local intelligence

Outcome: probabilistic intelligence can improve memory without owning authority.

Work:
- replaceable DecisionEngine interface;
- deterministic rule engine baseline;
- optional Ollama adapter;
- optional llama.cpp or qualified embedded runtime;
- typed extraction schemas;
- entity-link candidates;
- contradiction/supersession classification candidates;
- optional source-linked summaries;
- confidence calibration;
- abstention policy;
- exact model/runtime artifact identity;
- model license and digest manifest;
- local-only evaluation profile.

Gate:
- any claimed quality improvement is measured;
- model removal does not corrupt or orphan canonical memory;
- model output cannot bypass deterministic policy.

## P9 — Optional semantic retrieval and advanced artifact graph

Outcome: add complexity only after measured deterministic baseline gaps.

Work:
- freeze P5 retrieval baseline first;
- evaluate sqlite-vec and local embedding candidates;
- optional local semantic projection;
- explicit hybrid score decomposition;
- projection invalidation and rebuild;
- evaluate Graphify patterns for deterministic artifact/code graphs;
- evaluate code-graph-rag patterns for deeper structural retrieval;
- compare SQLite graph with any proposed external graph backend;
- reject infrastructure whose measurable benefit does not justify operational complexity.

Gate:
- semantic/vector features remain removable;
- canonical memory and deterministic retrieval work unchanged without them;
- no paid embedding/vector service is required.

## Dependency spine

```text
P5 -> P6 -> P7
          |
          +-> P8 -> P9
```

P8 and P9 improve the product after the deterministic path is already complete; they never become prerequisites for local core correctness.
