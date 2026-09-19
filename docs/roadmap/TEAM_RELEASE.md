# Roadmap D — Team Mode and v1 Release

## P12 — Team and self-hosted mode

Outcome: shared memory works without a mandatory hosted control plane.

Work:
- team and organization scopes;
- authenticated multi-user server;
- role and ACL administration;
- proposal and review flow;
- encrypted backups;
- optional peer/device synchronization;
- conflict-safe synchronization;
- audit and export boundaries;
- tenant-isolation tests;
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
- zero-cost conformance;
- reproducible performance suite;
- dependency, source, and license audit;
- fuzzing and security review;
- cross-platform packaging;
- SBOM;
- checksums and provenance attestations;
- migration, backup, and restore proof;
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
12. zero required service-fee operation is tested;
13. source, license, and security gates close;
14. reproducible release artifacts ship.

## Release rule

Comparative claims must name the exact Morize revision, benchmark revision, runtime/model profile, hardware class when material, and known limitations.
