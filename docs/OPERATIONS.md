# Morize Operations, Diagnostics, and Observability

## 1. Principle

Operational visibility must help users and operators understand Morize without turning private memory into telemetry.

Local diagnostics are first-class. Hosted observability is optional by deployment profile.

## 2. Structured operational events

Morize should emit bounded structured events for:

- service start/stop;
- vault open/close;
- migration start/result;
- recovery/reconciliation result;
- index build/invalidation;
- connector health;
- request category/result;
- authorization decision category;
- resource limit events;
- background job state;
- backup/restore;
- sync state;
- provider failures;
- release/update diagnostics.

Events use stable categories and avoid raw memory content by default.

## 3. Logging

Logging levels and destinations are explicit.

Rules:

- credentials and tokens are redacted;
- memory bodies are not ordinary log fields;
- source locators may require redaction according to sensitivity;
- authorization denials should not reveal protected resource existence;
- correlation IDs are operational identities, not user identity claims;
- production log retention is profile-specific.

## 4. Metrics

Candidate metrics include:

### Core

- vault open latency;
- mutation latency;
- FTS query p50/p95/p99;
- graph traversal latency;
- ContextBundle assembly latency;
- index backlog;
- reconciliation count/status;
- migration duration;
- derived index size;
- canonical storage size.

### Reliability

- failed mutation rate;
- unknown-outcome count;
- recovery count;
- corrupt-state detections;
- backup success/failure;
- restore verification failures.

### Security

Use aggregate event categories where useful:

- authorization denials;
- quarantined candidates;
- revoked connector use attempts;
- rate/resource-limit events.

Do not expose secret/sensitive payloads through metric labels.

### Commercial/managed

Future service metrics may include:

- tenant usage units;
- billable provider usage;
- quota consumption;
- connector runs;
- sync transfer/storage.

Commercial metrics are separate from memory truth.

## 5. Tracing

Distributed tracing is optional for networked profiles.

Trace spans should record operation identity and coarse metadata, not memory plaintext.

Sensitive request arguments require explicit safe representations.

## 6. Health

Health endpoints/commands distinguish:

- process alive;
- ready to serve reads;
- ready to serve writes;
- migration required;
- reconciliation required;
- degraded derivative index;
- connector degraded;
- provider degraded;
- resource exhaustion.

“Process is running” is not equivalent to “vault is writable and consistent.”

## 7. Doctor command

`morize doctor` should inspect local facts and produce actionable diagnostics.

Potential checks:

- vault format/version;
- canonical directory integrity;
- SQLite integrity;
- pending mutation/reconciliation;
- projection freshness;
- missing/corrupt blobs;
- connector binding state;
- model/vector optional dependencies;
- permissions;
- backup state;
- unsupported configuration;
- disk space warnings.

`morize doctor --offline` must not require external network calls.

## 8. Privacy and telemetry consent

No first-party hosted product analytics is required for local/self-hosted operation.

If product telemetry is introduced:

- default and consent behavior must be documented;
- event schema must be inspectable;
- memory content is excluded by default;
- sensitive metadata is minimized;
- disabling telemetry must not disable core functionality;
- telemetry endpoint identity is explicit;
- retention is documented.

Crash reporting follows the same privacy constraints.

## 9. Error diagnostics

Errors have stable domain categories and may include:

- operation/correlation ID;
- safe affected component;
- recovery suggestion;
- documentation reference;
- whether retry is safe;
- whether human review is required.

Avoid dumping arbitrary source/model/provider payloads into errors.

## 10. Performance baselines

Before setting marketing SLOs, establish reproducible baselines for representative vault sizes.

Benchmark dimensions:

- number of memories;
- number of versions;
- number of relations;
- source/blob volume;
- FTS corpus size;
- active scopes;
- concurrent local clients;
- cold/warm cache;
- hardware class.

Report p50/p95/p99 where meaningful and preserve outliers/failures.

## 11. Resource budgets

Every potentially adversarial loop has explicit caps:

- input bytes;
- result count;
- graph fan-out/depth;
- index batch;
- concurrent jobs;
- connector retries;
- model calls;
- download/upload size;
- import/export size;
- sync batch/in-flight bytes.

Exceeding a budget produces explicit degradation/failure without corrupting state.

## 12. Background jobs

Background work has:

- job identity;
- owning vault/scope;
- bounded retry policy;
- cancellation semantics;
- persisted status when required;
- recovery on restart;
- idempotency where consequential.

Examples:

- indexing;
- compaction;
- expiry;
- backup;
- sync;
- connector ingestion.

“Background” does not mean unaccounted or unbounded.

## 13. Backup monitoring

A configured backup is not considered healthy merely because scheduling exists.

Track:

- last attempted backup;
- last successful backup;
- artifact identity/digest;
- restore test status;
- retention;
- encryption/key availability.

Managed services may publish RPO/RTO only after measured operational evidence.

## 14. Managed service SLOs

No uptime, latency, durability, RPO, or RTO promise is frozen during open-source foundation planning.

Before a commercial SLA:

1. define measurement boundary;
2. instrument service;
3. establish baseline;
4. test failure modes;
5. define exclusions;
6. model cost;
7. approve support/on-call obligations.

## 15. Incident response

Before managed customer launch, document:

- severity levels;
- detection;
- triage ownership;
- containment;
- customer communication;
- evidence preservation;
- recovery;
- post-incident review;
- security disclosure path.

Open-source vulnerability reporting remains in `SECURITY.md`.

## 16. Cost observability

Founder/project-operated paid infrastructure must be visible.

Track:

- monthly recurring cost;
- usage-sensitive cost;
- provider;
- purpose;
- owning profile;
- budget cap;
- customer revenue linkage;
- anomaly alerts where available.

A reliability retry loop may not become an uncontrolled billing loop.

## 17. Operational readiness gate

A profile is production-supported only when it has:

- health/readiness semantics;
- diagnostics;
- backup/restore story;
- upgrade/migration story;
- resource bounds;
- error taxonomy;
- security logging rules;
- operational documentation;
- applicable platform/deployment tests.

Passing core unit tests alone does not establish operational readiness.
