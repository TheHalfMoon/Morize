# Roadmap D — Team Mode and v1 Release

## P12 — Team and self-hosted mode

Outcome: shared memory works without a mandatory hosted control plane.

Work:
- team and organization scopes;
- authenticated multi-user server;
- role and ACL administration;
- proposal and review flow;
- encrypted backups;
- optional peer/device synchronization through the dedicated sync contract;
- typed concurrent-edit/update-vs-forget/update-vs-redact conflict handling;
- peer revocation, stale-device behavior, and bounded reconciliation;
- audit and export boundaries;
- tenant-isolation and side-channel tests;
- per-tenant resource quotas;
- local structured operations/diagnostics;
- keep single-user local mode independent.

Gate:
- multi-user isolation and recovery tests pass;
- no hosted Morize account is required.

## P13 — Memory Lab, hardening, and v1

Outcome: public claims match exact evidence.

Work:
- LongMemEval-V2 harness;
- selected LoCoMo and contradiction/update evaluation;
- Morize temporal, scope, poisoning, and recovery suites;
- founder-zero-burn development conformance and local/self-hosted profile conformance;
- reproducible performance suite;
- dependency, source, Apache-2.0, NOTICE, and third-party license audit;
- fuzzing and security review;
- cross-platform packaging;
- SBOM;
- checksums and provenance attestations;
- migration, upgrade, backup, and restore proof;
- public API/SDK/MCP compatibility matrix;
- supported-version/platform/security-support statement;
- founder recurring-cost ledger review;
- independent semantic review;
- exact-head release qualification;
- preserve failed and negative results.

## v1 completion criteria

Morize v1 is complete only when evidence proves:

1. offline no-model core is useful;
2. canonical memory is human-readable and recoverable;
3. temporal truth and explicit conflicts work;
4. derived indexes are rebuildable;
5. provenance survives durable changes;
6. cross-scope isolation holds;
7. restart and recovery behavior is deterministic;
8. MCP remains least-authority;
9. at least two major client integrations interoperate;
10. import/export avoids lock-in;
11. optional local intelligence cannot bypass policy;
12. the pre-revenue development path has no unapproved mandatory founder-paid recurring dependency, and the supported local/self-hosted profile is independently tested;
13. source, license, and security gates close;
14. reproducible release artifacts ship.

## Release rule

Comparative claims must name the exact Morize revision, benchmark revision, runtime/model profile, hardware class when material, and known limitations.
