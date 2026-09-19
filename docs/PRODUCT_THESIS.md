# Morize Product Thesis

## Category

Morize is a **Memory Operating System for AI**.

It is not merely a vector database, chat-history store, RAG framework, agent harness, user-profile service, or hosted memory API. Morize owns the lifecycle between an observation and durable, retrievable, governed memory.

## Core job

For every candidate piece of knowledge, Morize should be able to answer:

1. What exactly is being claimed?
2. What source or observation supports it?
3. Who or what observed it?
4. When was it observed?
5. During what interval was it valid?
6. Which scope owns it?
7. How sensitive is it?
8. Does it conflict with existing knowledge?
9. Does it supersede prior knowledge?
10. Should it be remembered at all?
11. Who may retrieve it?
12. Why did a retrieval choose it?
13. Can the user inspect, correct, export, redact, forget, or rewind it?
14. Can the system operate if every model/API/network integration is disabled?

## Product principles

### P1 — Memory is governed state

A model may propose memory. A model does not gain unlimited authority to persist arbitrary text.

`MEMORY_CANDIDATE != DURABLE_MEMORY`

### P2 — Retrieval relevance is not source authority

A highly similar item may be stale, untrusted, cross-project, secret-derived, or contradicted.

`RETRIEVAL_SCORE != SOURCE_AUTHORITY`

### P3 — Confidence is not authority

Inspired by machine-native typed-decision systems, Morize represents uncertainty explicitly and lets deterministic policy choose autonomous versus reviewed paths.

`HIGH_CONFIDENCE != AUTHORIZED_MUTATION`

### P4 — Canonical truth survives loss of indexes

FTS, vector indexes, graph metrics, caches, embeddings, summaries, and reranking state are projections. Delete them and Morize must still retain canonical knowledge.

### P5 — Local-first is architectural

The core must remain useful with:

- no network;
- no API key;
- no cloud account;
- no vector server;
- no GPU;
- no hosted model;
- no Morize account.

### P6 — Evidence is retained across change

Superseding a fact does not silently destroy its history. Contradictions remain representable. Forget/redact removes active content according to policy without claiming that already-exported external copies disappeared.

### P7 — Users own memory

Users can inspect, search, edit through governed workflows, export, import, pin, expire, redact, forget, and back up durable memory.

### P8 — Scope is part of meaning

The same statement may be valid in one project and invalid in another. Run, session, agent, project, user, team, organization, and public/reference scopes must not collapse into one flat namespace.

### P9 — Security precedes convenience

Retrieved memory is untrusted input to an agent. Durable memory is a prompt-injection and data-exfiltration surface. Permissions are evaluated before retrieval output, not after ranking.

### P10 — Claims require reproducible evidence

Morize should publish failed or negative benchmark results rather than hide them. Product claims bind an exact version, dataset, configuration, hardware class, and evaluation protocol.

## Product surfaces

Morize should expose one governed engine through multiple surfaces:

- embedded Rust library;
- local daemon;
- CLI;
- MCP server;
- REST API;
- Rust SDK;
- Python SDK;
- TypeScript SDK;
- coding-agent plugins/hooks;
- desktop/web inspector over the local daemon;
- optional self-hosted team server.

No surface may bypass canonical policy and mutation contracts.

## Target users

### Individual AI users

Share durable memory across multiple AI assistants without handing it to a hosted memory provider.

### Developers

Add a typed memory API with provenance, temporal semantics, explainable retrieval, and migration tools.

### Coding agents

Give repositories project memory, decisions, failures, lessons, repository context, and cross-session continuity.

### Teams

Share reviewed organization/project memory while preserving private user and agent scopes.

### Research

Inspect and evaluate memory behavior as a system rather than trusting a proprietary API.

### High-sensitivity domains

Provide the substrate for healthcare, legal, research, or enterprise systems while keeping domain-specific compliance and decision safety separate from the memory engine itself.

## North-star experience

A user installs Morize locally:

```bash
morize init
morize daemon
morize connect codex
morize connect claude-code
```

Later, either agent can ask:

```text
What database is canonical for this project, why was it chosen,
what did it replace, and which evidence supports that answer?
```

Morize returns a compact answer plus inspectable provenance and temporal history, without requiring a paid API.

## Non-goals for the first release

- training a foundation model;
- replacing general-purpose databases;
- becoming an autonomous agent platform;
- hosting arbitrary code execution inside the trusted memory core;
- making medical/legal/financial truth judgments;
- mandatory cloud sync;
- mandatory graph database infrastructure;
- mandatory dense-vector retrieval;
- opaque self-modification of policies or memory schemas.

## Success definition

Morize v1 succeeds when a new user can run it offline, connect at least two agent harnesses, persist governed project/user memories, resolve temporal updates and contradictions, inspect why recall happened, export everything, rebuild every derivative index, and run the full core test suite without paying for any external service.
