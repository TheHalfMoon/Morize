# Contributing to Morize

Morize welcomes contributions that improve correctness, privacy, portability, performance, interoperability, or developer experience without weakening the core invariants.

## Before changing code

Read:

1. `README.md`
2. `docs/PRODUCT_THESIS.md`
3. `docs/ARCHITECTURE.md`
4. `docs/FOUNDER_COST_AND_COMMERCIAL_BOUNDARY.md` (founder-cost and commercial boundary)
5. `docs/THREAT_MODEL.md`
6. `docs/DONOR_AND_PROVENANCE.md`
7. the applicable roadmap packet.

## Core contribution rules

- Do not introduce a mandatory recurring founder-paid service without an explicit cost owner, cap, justification, and approved SpecGrain.
- Do not add hidden network fallbacks.
- Do not make vector search, model inference, or an external graph database a canonical-memory dependency.
- Treat model output as untrusted candidate data.
- Preserve provenance for durable knowledge.
- Authorize before disclosure, not after ranking.
- Keep derived indexes rebuildable.
- Add finite limits for externally controlled input.
- Preserve negative/failing evaluation evidence.
- Do not make comparative superiority claims without reproducible evidence.

## Contribution attestation

Morize uses the Developer Certificate of Origin 1.1 in `DCO`.

Contributors should certify commits with a `Signed-off-by:` trailer using `git commit -s` or an equivalent valid sign-off.

The current project does not require a CLA.

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
- founder-cost, commercial-service, and local/self-hosted impact;
- known limitations.

## Community

Be technically direct and respectful. Critique designs and evidence rather than people.


## Planning and proof

Implementation contributions must originate from a bounded SpecGrain or an explicitly approved equivalent repository task.

Completion uses Diffcipline proof semantics:

- exact diff and scope;
- risk profile;
- dependency/lockfile evidence;
- actually executed verification;
- policy provenance.

`NOT RUN` is never represented as `PASS`.

See `docs/PLANNING_GOVERNANCE.md`.
