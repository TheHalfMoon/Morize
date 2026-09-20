# Morize Source Ledger

**Research snapshot:** 2026-09-19
**Status:** planning evidence; no third-party component is automatically admitted into the trusted core.

## Admission vocabulary

- **COPY_SELECTIVE** — selected source may be copied after exact path/revision/rights/dependency review.
- **ADAPT** — preserve a useful mechanism while implementing Morize-owned contracts.
- **DEPEND_OPTIONAL** — optional dependency behind a replaceable adapter.
- **REFERENCE** — study behavior/architecture; no source transfer implied.
- **BENCHMARK** — evaluation input only.
- **PROCESS_REFERENCE** — engineering/governance discipline rather than runtime code.
- **REJECT_CORE_DEPENDENCY** — may be studied but must not become a required Morize runtime dependency.

Founder permission makes authorized sources eligible for consideration; engineering quality, security, attribution, redistribution rights, dependency closure, and the founder-cost/commercial boundary remain independent gates.

## Tier A — Direct memory-system references

| Source | Research pin/state | Primary value | Initial posture |
|---|---|---|---|
| `moorcheh-ai/memanto` | `ce38df5070cbaf57613f7a404fef6779ee3bef44` | memory extraction/read/write/policy services, namespaces, scheduling, migration/export, MCP and agent integrations, CLI/UI, TypeScript SDK | **COPY_SELECTIVE / ADAPT**; direct donor explicitly authorized by founder |
| `rcarmo/memento` | `0b0b8f94dd8b0410a0e3c0fd547e995d2b739b41` | Markdown-in-Git canonical concepts, separate control/derived SQLite, authenticated MCP, proposals/review, writer lease, rebuildable indexes | **ADAPT / COPY_SELECTIVE** after exact component review |
| `getzep/graphiti` | `de8eb5b896c05ed1b5b329d4cb52015446d65e21` | temporal knowledge graphs, fact validity windows, provenance, ontology, historical queries | **REFERENCE / ADAPT** |
| `mem0ai/mem0` | `a39a802bbc93e85b820078cd3c4dbaf53af25dbe` | memory extraction/update patterns, provider abstraction, metadata, operational ecosystem | **REFERENCE / ADAPT**; never canonical truth dependency |
| `letta-ai/letta-code` | `8994497c7f5bc29da97fdad8703e4531152ca631` | persistent named agents, cross-session recall, experience, forks, memory evolution, UX | **REFERENCE / ADAPT**; self-modification never becomes trusted automatically |
| `volcengine/OpenViking` | `0b3ecd9a6a5b1b9c036f5f0a8089aaa03994034f` | event-driven memory evolution, user/peer scopes, directory-aware context, cases/trajectories/experiences, Codex integration | **REFERENCE / ADAPT** |
| `supermemoryai/supermemory` | `57b430b5b6a19106a989651f4cde853c05147682` | memory + RAG + profiles + connectors, local runtime, agent plugins, context reduction patterns | **REFERENCE / ADAPT**; no hosted billing dependency |
| `TencentCloud/TencentDB-Agent-Memory` | `41dee1f9f8cd2b7e87f3e5c073966dc58d296a16` | L0/L1/L2/L3 memory hierarchy, knowledge/asset metadata, user/team/agent/task relationships, service boundaries | **REFERENCE / ADAPT** |
| `Graphify-Labs/graphify` | `b9cd9570728a5ff3485d2a1e36fe9a1272a368ae` | deterministic AST graphing, explained edges, path/query/explain, no-vector graph approach | **ADAPT / COPY_SELECTIVE** after path review |
| `vitali87/code-graph-rag` | `3cd90fe854c94a773125b555dc106a12c113a798` | code/evidence graph retrieval and deeper structural relationships | **REFERENCE / OPTIONAL_ADAPTER** |
| `asg017/sqlite-vec` | `04d28bd21773981e2d266bbf6aa4efbd011eb4f6` | single-file local vector extension aligned with SQLite architecture | **DEPEND_OPTIONAL** only after measured need |

## Tier B — Internal architecture sources

The linked GitHub account was surveyed across accessible public and private repositories. The following repositories contain directly reusable design or process evidence.

| Internal source | Visibility | Candidate contribution |
|---|---|---|
| `TheHalfMoon/Golam` | public | governed memory writer, candidate-vs-durable separation, provenance/taint, memory operations, crash reconciliation, strict-local rules, Source Foundry |
| `TheHalfMoon/kernux` | public | ContextSource/ContextItem/ContextBundle model, search hierarchy, permission-filtered context, memory poisoning defenses, project/user/org scopes |
| `TheHalfMoon/Himsat` | public | Rust-first local-first architecture, evidence-linked capture, encrypted vault/recovery patterns, no hidden network fallback |
| `TheHalfMoon/MedScale` | public | provenance, privacy gates, source qualification, graph/data workspace research, abstention/evidence patterns |
| `TheHalfMoon/SpecGrain` | public | recursive implementation planning, bounded context, evidence packets, independent verification, benchmark discipline |
| `TheHalfMoon/Diffcipline` | public | proof-before-done, exact-diff evidence, reproducible claims, deterministic PASS/REVIEW/FAIL contracts |
| `TheHalfMoon/Sentrdel` | public | security/policy/control-plane patterns |
| `TheHalfMoon/Tarif` | public | default-deny action authority, secret isolation, receipts |
| `TheHalfMoon/Ecra` | public | browser/search/agent capability routing and execution receipts |
| `TheHalfMoon/Qdrat` | public | teams, roles, membership, approvals, tasks, reporting/audit patterns |
| `TheHalfMoon/Kodac` | public | donor admission, sandbox/evidence discipline, runtime identity and supply-chain patterns |
| `TheHalfMoon/Ascout` | public | review/test/security orchestration and evidence surfaces |
| `TheHalfMoon/MESC` | public | reproducibility, model/runtime qualification, evidence-first governance |
| `TheHalfMoon/HarnessMind` | private | candidate harness/evidence patterns; no private source text is published by this planning pass |
| `TheHalfMoon/ProtocolWISE` | private | candidate provenance/verification patterns; public transfer requires per-component review |
| `TheHalfMoon/Fanatir` | private | reviewed as an internal candidate pool; no automatic transfer |
| `TheHalfMoon/Hikma` | private | reviewed as an internal candidate pool; no automatic transfer |
| `TheHalfMoon/Coddev` | private | reviewed as an internal candidate pool; no automatic transfer |
| `TheHalfMoon/Paina` | private | reviewed as an internal candidate pool; no automatic transfer |
| `TheHalfMoon/kodac-phase-b-gate` | private | candidate qualification evidence; no automatic transfer |
| `TheHalfMoon/Golam-research` | public | reconstructed agent/runtime behavior, capability boundaries, MCP and local execution evidence |
| `TheHalfMoon/Wispral` | public | voice/command/context and permission UX patterns |
| `TheHalfMoon/Delethos` | public | planning/deletion/evidence discipline where relevant |
| `TheHalfMoon/Flake` | public | structured canonical planning and source-admission gates |
| `TheHalfMoon/Winds` | public | agent/runtime engineering evidence where relevant |
| `TheHalfMoon/commandF` | public | repository/tool/context patterns where relevant |
| `TheHalfMoon/commandMed` | public | provenance, proposal-only model semantics, verification patterns |
| `TheHalfMoon/MSTR` | public | evidence/governance patterns where relevant |
| `TheHalfMoon/wepld` | public | research/source-gap methodology |
| `TheHalfMoon/Signthos` | public | source/provenance patterns where relevant |
| `TheHalfMoon/Zyara` | public | scoped entity/relationship design references where relevant |
| `TheHalfMoon/Balott` | public | no current core-memory adoption; retain in surveyed inventory |
| `TheHalfMoon/Trcel` | public | empty at survey time; no current adoption |
| `TheHalfMoon/Morize` | public | target repository |

Private repositories are not copied wholesale into public Morize. Exact files/components require a transfer record that confirms the founder-authorized rights, intended public disclosure, attribution, dependency closure, and security review.

## Tier C — Evaluation sources

| Source | Pin/state | Purpose |
|---|---|---|
| `xiaowu0162/LongMemEval-V2` | `2cc8c540bdb87fe6761629b585e727e1c4704520` | long-horizon memory benchmark |
| `mem0ai/memory-benchmarks` | current research candidate | LoCoMo/LongMemEval/BEAM-style evaluation harness |
| `ProsusAI/MemEval` | current research candidate | cross-system memory evaluation |
| Supermemory MemoryBench | current research candidate | additional conversational memory/RAG evaluation |

Morize adds its own cases for stale memory, authoritative-source precedence, temporal validity, contradictions, poisoning, scope leakage, abstention, user edits, FORGET/REDACT, index rebuild, and offline operation.

## Tier D — Protocol/runtime/tool references

- `alibaba/open-code-review` at `85cecfe5f935da2b2aae8f91ce4fee8ed343a681` — **PROCESS_REFERENCE / REVIEW_ONLY**; deterministic review file selection/rule resolution and delegation-mode coverage. It is not a Morize runtime dependency and does not replace SpecGrain readiness or Diffcipline proof.

- `rust-fuzz/cargo-fuzz` release `0.13.2` (release commit `984c861c8dfea28055254c5f1d2659ab2cd63f76`) — **DEV_TOOL / FUZZ_ONLY**. Official libFuzzer Cargo frontend used only by the isolated P1 fuzz workspace and Linux fuzz-smoke qualification. The pinned x86_64 Linux-musl release asset has upstream SHA-256 `b5b704018b63e0f151c17a057ac53b5111e1db545d1b9f72fee79f08a545931c`. It is not a runtime, product, memory-authority, or root-workspace dependency.

- `rust-fuzz/libfuzzer` / `libfuzzer-sys 0.4.13` at release merge `719e4efb9b8857ebaa782ae59376c8cbb78fed0f` — **DEV_DEPENDENCY / FUZZ_ONLY**. Upstream declares `(MIT OR Apache-2.0) AND NCSA`; the bundled libFuzzer sources are NCSA-licensed and the wrapper is MIT/Apache-2.0. Admission therefore requires fuzz-only isolation, exact lockfile closure, preserved license/notice documentation, and no linkage into Morize runtime artifacts. Current upstream cargo-fuzz releases include Windows/MSVC artifacts as well as Linux/macOS support; Morize intentionally scopes this bounded fuzz-smoke harness to Ubuntu/Linux so normal Windows product CI remains independent of nightly/libFuzzer tooling. The pinned CI tool artifact is the upstream x86_64-unknown-linux-musl cargo-fuzz executable, while sanitizer-instrumented fuzz targets are explicitly compiled and run for x86_64-unknown-linux-gnu because AddressSanitizer is incompatible with statically linked musl libc.

- `convaiinnovations/laya` — Hugging Face model repository observed 2026-09-20; Apache-2.0; the current model card describes a 421M-parameter text-classification/typed-decision model with `choice`/`score`/`noul` outputs, calibrated probabilities, routing, and non-generative behavior. These are upstream claims and are not Morize-verified performance or calibration evidence. **REFERENCE / DEPEND_OPTIONAL** for P8 qualification only; an exact immutable Hub revision/artifact digest, runtime/configuration identity, local resource cost, calibration, abstention, license/notice, and safety evidence are required before admission. Laya output is evidence, never Morize authority.

- TinyFish / Monid web tooling — service behavior observed 2026-09-20 from `https://monid.ai/blog/tinyfish`; related public implementation references: `monid-ai/monid@bd152f68e9f1f54eea132602fed49da90b6ab379` and `tinyfish-io/tinyfish-cookbook@8615317f6db58ae776dd53817ac30668c1db5ef8` (both MIT at the observed revisions). Candidate value: live search, browser-rendered multi-URL fetch, recency-bounded acquisition, and explicit escalation from search/fetch to heavier browser automation. **REFERENCE / ADAPT / COPY_SELECTIVE** after exact component/path review. Any TinyFish/Monid integration is an optional remote connector with explicit credential, network, data-boundary, rate, availability, and cost semantics; current free Search/Fetch pricing is not a permanent Morize assumption and must never become a correctness dependency.

- official Model Context Protocol specification and Rust SDK;
- Agent Skills format;
- ripgrep-class bounded local search;
- Tree-sitter for optional structural extraction;
- Ollama / llama.cpp / qualified embedded local inference;
- RustSec, cargo-deny, cargo-vet, SLSA/GitHub attestations, SBOM tooling, and cargo-dist-class packaging.

None of these may become an unreviewed authority surface.

## Jev / TypeSafe AI philosophy

TypeSafe AI is a **philosophy reference**, not a runtime dependency and not a code donor under this plan.

Useful ideas:

- decisions instead of unconstrained strings;
- typed outputs known in advance;
- calibrated uncertainty;
- application-owned thresholds;
- compose decisions in code;
- escalate low-confidence cases.

Morize implements these ideas in its own open, local `DecisionEnvelope` contract. No Jev API call is required, because it must not become a mandatory founder-paid dependency or a semantic authority dependency. A future paid Morize service may still use paid providers explicitly.

## Source selection rule

For every candidate:

```text
PERMISSION != TECHNICAL_FIT
POPULARITY != QUALIFICATION
REFERENCE != CODE_ADMISSION
MODEL_OUTPUT != VERIFIED_FACT
HOSTED_FEATURE != REQUIRED_ARCHITECTURE
BENCHMARK_RESULT != AUTHORITY
```

The preferred order is:

1. adopt a protocol/behavioral idea;
2. implement a small Morize-native mechanism when simpler;
3. adapt selected code when it materially improves correctness or delivery;
4. depend on an external package only when it is replaceable and does not violate the required local/self-hosted contract or founder-cost boundary;
5. reject broad donor stacks that drag in hosted services or unnecessary authority.
