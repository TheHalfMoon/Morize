# Morize API, Compatibility, and Evolution Policy

## 1. Surfaces

Morize has several versioned public surfaces:

- canonical vault format;
- portable export/import format;
- Rust library API;
- local daemon HTTP/IPC API;
- MCP tools/resources/prompts;
- CLI commands, flags, output, and exit codes;
- Python SDK;
- TypeScript SDK;
- connector contracts;
- hosted-service API when introduced.

A surface becoming convenient does not silently make it stable. Stability status must be explicit.

## 2. Compatibility classes

Every exposed surface is classified as:

- `INTERNAL` — no compatibility promise outside the owning crate/module;
- `EXPERIMENTAL` — may change with release notes/migration guidance;
- `STABLE` — compatibility contract applies;
- `DEPRECATED` — still supported for a declared window;
- `REMOVED` — no longer supported.

## 3. Versioning

Use semantic-versioning principles for released software, but persistent data deserves stricter treatment than ordinary source APIs.

A change is breaking when it can cause a supported client/vault to:

- fail to parse;
- misinterpret meaning;
- lose data;
- change authorization behavior;
- change temporal semantics;
- change deletion semantics;
- silently alter retrieval contract;
- require an unavailable dependency.

## 4. Persistent-format rule

The Morize Vault Format and portable export format carry independent schema versions.

Before a writer emits a new incompatible representation:

1. reader support exists;
2. migration behavior is implemented;
3. backup/rollback behavior is defined;
4. compatibility fixtures exist;
5. recovery is tested;
6. Diffcipline risk is at least R2 and R3 when migration can damage existing durable state.

## 5. Read compatibility

A reader must define one of these behaviors for unknown/newer data:

- reject clearly;
- preserve opaque data where semantically safe;
- enter read-only compatibility mode.

It must not guess.

## 6. Write compatibility

A process must not write a vault when:

- the vault format is newer than the writer understands;
- migration preconditions are not satisfied;
- reconciliation state is unresolved;
- required policy/source identities are unsupported.

Read-only diagnosis/export may remain possible when safe.

## 7. API negotiation

Daemon/SDK interfaces should expose:

- API version;
- server capabilities;
- optional feature flags;
- compatibility status.

Clients must not infer support from server version strings alone when feature negotiation is available.

## 8. CLI contract

Stable CLI commands define:

- argument/flag syntax;
- exit-code meaning;
- machine-readable JSON schema when offered;
- stdout/stderr responsibilities;
- destructive-action confirmation/non-interactive rules.

Human prose output may evolve more freely than documented machine JSON.

## 9. MCP contract

MCP tool names and argument/result schemas are versioned Morize contracts.

Rules:

- do not repurpose a stable tool name with incompatible semantics;
- administrative tools remain separated from ordinary memory tools;
- client-advertised capability does not expand Morize authority;
- removed tools follow deprecation guidance when practical.

## 10. SDK parity

Rust is the semantic reference implementation.

Python/TypeScript SDKs may differ idiomatically but must preserve:

- identity semantics;
- lifecycle semantics;
- error categories;
- authorization results;
- temporal query meaning;
- pagination/bounds;
- idempotency behavior.

Generated schemas are preferred where they reduce drift.

## 11. Error taxonomy

Public APIs should distinguish at least:

- invalid input;
- unsupported version;
- authentication failure;
- authorization denial;
- conflict/stale precondition;
- review required;
- quarantine;
- not found without protected-existence leakage;
- resource/budget exceeded;
- transient provider failure;
- permanent connector failure;
- unresolved/unknown write outcome;
- internal corruption/recovery required.

Transport status codes do not replace domain errors.

## 12. Idempotency

Consequential API operations require stable idempotency semantics.

A retry after timeout must not create duplicate durable changes merely because the client did not receive the first response.

## 13. Pagination and bounds

List/search APIs define:

- maximum page size;
- stable continuation semantics;
- ordering;
- snapshot/freshness behavior;
- result truncation.

No public API returns unbounded vault/graph contents by default.

## 14. Deprecation policy

Before removing a STABLE surface when feasible:

1. mark it deprecated;
2. document replacement;
3. provide migration guidance;
4. maintain a declared support window;
5. add compatibility tests for the window;
6. remove only in an allowed breaking release.

Security issues may require faster removal; the exception must be documented.

## 15. Connector compatibility

Each connector binds:

- connector type/version;
- external API/protocol version;
- Morize adapter version;
- mapping/configuration digest.

External provider changes do not silently rewrite adapter semantics.

## 16. Hosted-service compatibility

Future Morize Cloud may evolve operationally faster than the open-source core, but the service must publish the compatibility versions it supports for:

- clients;
- vault/export formats;
- SDKs;
- connectors.

Billing plan changes do not silently alter data meaning.

## 17. Compatibility test matrix

Before v1 stable release, build fixtures for:

- current writer -> current reader;
- supported older writer data -> current reader;
- current writer -> supported previous reader where backward compatibility is promised;
- upgrade from every declared supported prior persisted version;
- interrupted migration recovery;
- import/export round-trip;
- cross-platform canonicalization;
- SDK/client compatibility against server versions.

## 18. Breaking-change governance

Any proposal that breaks a stable contract must be a separate SpecGrain whose outcome includes:

- necessity;
- affected populations;
- migration strategy;
- rollback;
- documentation;
- release/version consequence;
- support window;
- founder/customer operational cost.

Breaking changes are never incidental refactors.
