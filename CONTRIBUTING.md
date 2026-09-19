# Contributing to Morize

Morize welcomes contributions that improve correctness, privacy, portability, performance, interoperability, or developer experience without weakening the core invariants.

## Before changing code

Read:

1. `README.md`
2. `docs/PRODUCT_THESIS.md`
3. `docs/ARCHITECTURE.md`
4. `docs/ZERO_COST_CONTRACT.md`
5. `docs/THREAT_MODEL.md`
6. `docs/DONOR_AND_PROVENANCE.md`
7. the applicable roadmap packet.

## Core contribution rules

- Keep the normal local path free of required paid services.
- Do not add hidden network fallbacks.
- Do not make vector search, model inference, or an external graph database a canonical-memory dependency.
- Treat model output as untrusted candidate data.
- Preserve provenance for durable knowledge.
- Authorize before disclosure, not after ranking.
- Keep derived indexes rebuildable.
- Add finite limits for externally controlled input.
- Preserve negative/failing evaluation evidence.
- Do not make comparative superiority claims without reproducible evidence.

## Source reuse

If code is copied or adapted from another project, include an exact source record and satisfy `docs/DONOR_AND_PROVENANCE.md`.

A dependency or donor is not admitted merely because it is popular, permissively licensed, or already used elsewhere.

## Development workflow

The implementation is Rust-first. Exact commands will be frozen when P1 creates the workspace. Until then, do not add placeholder runtime dependencies.

Changes should be small enough to review independently and should include:

- tests;
- documentation changes when behavior changes;
- migration notes for persistent-format changes;
- security analysis for new trust boundaries;
- benchmark evidence for performance/quality claims.

## Evidence

A change is not complete because an agent or contributor says it is complete.

Required checks must run successfully against the exact change. CI and local verification should remain reproducible and should not require secret cloud credentials for the trusted core.

## Pull requests

PR descriptions should state:

- problem;
- scope;
- design;
- source provenance if applicable;
- tests/evidence;
- security impact;
- zero-cost/local-first impact;
- known limitations.

## Community

Be technically direct and respectful. Critique designs and evidence rather than people.
