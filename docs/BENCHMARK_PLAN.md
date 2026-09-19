# Morize Memory Lab — Benchmark Plan

## Objective

Morize must not claim to be “better than Memanto” or any other memory system until a reproducible, version-pinned evaluation supports the exact claim.

Memory Lab evaluates both **memory quality** and **systems behavior**.

## External benchmark families

Initial candidates:

- LongMemEval-V2;
- LoCoMo;
- BEAM-style contradiction/update evaluation;
- Mem0 memory-benchmarks;
- ProsusAI MemEval;
- Supermemory MemoryBench where usable under reproducible local conditions.

Every benchmark run records:

- Morize commit;
- dataset revision/digest;
- adapter/configuration;
- local model artifact and runtime when used;
- hardware/OS;
- random seeds where applicable;
- retrieval budgets;
- context limits;
- timing;
- failures/timeouts;
- evaluator identity;
- raw result artifacts.

No floating-main benchmark result is accepted as release evidence.

## Morize-specific evaluation suites

### M1 — Exact durable recall

Can Morize retrieve explicitly stored facts without an LLM or vector index?

Measure:

- recall;
- precision;
- latency;
- result stability;
- index rebuild parity.

### M2 — Temporal truth

Cases include:

- fact becomes valid later;
- fact stops being valid;
- Morize learns about an old event after the fact;
- two versions overlap;
- future-dated candidate;
- correction of previously recorded valid interval.

Measure current and historical query accuracy separately.

### M3 — Supersession vs contradiction

Examples distinguish:

- “we migrated from MySQL to PostgreSQL”;
- “source A says MySQL, source B says PostgreSQL at the same time”;
- “the database is PostgreSQL for app state but ClickHouse for analytics”.

The evaluator must not reward destructive collapse of legitimate multi-context facts.

### M4 — Authoritative-source precedence

A semantically similar low-authority source must not outrank a scoped authoritative source merely because its text is closer to the query.

### M5 — Stale-memory resistance

Change source revisions and verify:

- stale index invalidation;
- stale summary handling;
- superseded active memory;
- historical availability when requested.

### M6 — Memory poisoning

Inject documents/web-style content attempting to create durable instructions, elevate trust, disclose other scopes, or overwrite policy.

Measure:

- promotion rate of malicious candidates;
- retrieval rate into unrelated ContextBundles;
- false-positive quarantine rate.

### M7 — Scope isolation

Create identical-looking records across users/projects/teams. Verify an unauthorized caller cannot infer existence through:

- result content;
- counts;
- ranking;
- error text;
- graph metadata;
- semantic neighbors.

### M8 — Sensitive-data handling

Test known secret patterns and synthetic sensitive records.

Measure:

- ordinary-memory persistence rate;
- log leakage;
- export behavior;
- redaction/forget correctness.

Only synthetic fixtures are used in public CI.

### M9 — User-edit reconciliation

Externally edit canonical Markdown between observation and commit. Verify Morize detects and reconciles instead of overwriting.

### M10 — Crash consistency

Inject crashes at each mutation boundary:

- before PREPARED;
- after PREPARED;
- after temp write;
- after file publish;
- before SQLite update;
- after SQLite update;
- before terminal receipt.

Verify restart converges to one explainable state and never invents success.

### M11 — Derived-index destruction

Delete:

- FTS projection;
- graph projection;
- semantic index;
- summaries/cache.

Then rebuild and prove canonical memory remains intact.

### M12 — Retrieval explanation

Every benchmark retrieval should emit a machine-readable trace containing applicable signals such as:

- exact/alias match;
- metadata filters;
- FTS rank;
- graph path;
- temporal eligibility;
- source authority;
- freshness;
- optional vector score;
- optional reranker score;
- exclusion reasons.

### M13 — Context efficiency

Measure:

- bytes/tokens retrieved;
- useful evidence retained;
- duplicate/stale content;
- context reduction relative to transcript replay;
- answer quality when an optional answering model is used.

### M14 — Local/self-hosted and founder-cost conformance

Run the local/self-hosted baseline benchmark cases with:

- external network blocked where the tested operation does not require a connector;
- no required paid memory service credential;
- no hosted vector database requirement;
- no mandatory model;
- no Docker requirement for the local core;
- no unapproved recurring founder-paid dependency in the development path.

This proves the supported local profile and founder-zero-burn development boundary. It does not promise that future managed Morize services or user-selected providers are free.

### M15 — Local-AI enhancement

Separately evaluate an enhanced local profile using one or more qualified local models.

Report its incremental quality, latency, RAM, disk, and CPU cost relative to deterministic baseline.

Do not mix this result with zero-model baseline.

## System performance budgets

Initial measurements, not premature promises:

- cold startup;
- warm local search latency;
- FTS query p50/p95;
- bounded graph traversal p50/p95;
- timeline query;
- mutation commit;
- index incremental update;
- full index rebuild;
- memory footprint;
- disk amplification;
- large-vault behavior.

Target thresholds are frozen only after a representative baseline exists.

## Comparative evaluation

Systems may be compared only when:

- exact versions are pinned;
- equivalent datasets are used;
- equivalent model/provider assumptions are disclosed;
- hosted-only costs are reported;
- failures are preserved;
- unavailable features are marked N/A rather than scored as success;
- local/offline and hosted profiles are not silently mixed.

Possible comparison set:

- Memanto;
- Mem0;
- Graphiti-based memory;
- Letta Code memory behavior;
- OpenViking;
- Supermemory local;
- TencentDB Agent Memory;
- Memento.

Morize should compare product dimensions as well as benchmark accuracy:

- offline operation;
- no-model usefulness;
- provenance;
- temporal semantics;
- contradiction behavior;
- scope isolation;
- explainability;
- exportability;
- index rebuildability;
- crash recovery;
- mandatory recurring cost.

## Evaluation policy

```text
BENCHMARK_WIN != UNIVERSAL_SUPERIORITY
MODEL_JUDGE != GROUND_TRUTH
FAILED_RUN != DISCARDABLE
HIGH_RECALL != SAFE_MEMORY
LOW_LATENCY != CORRECT_AUTHORITY
```

Negative evidence is preserved.

## Release claim gate

A public comparative claim requires:

1. preregistered protocol;
2. exact source/version pins;
3. reproducible commands;
4. raw artifacts;
5. statistical treatment appropriate to the metric;
6. limitations;
7. independent rerun when feasible.

Until then, README language describes capabilities rather than declaring a winner.
