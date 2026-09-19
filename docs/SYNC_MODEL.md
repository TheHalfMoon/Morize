# Morize Synchronization and Conflict Model

## 1. Scope

Synchronization is a future collaboration/device capability. It is not the same as backup, replication, or export.

The local v1 writer begins from a simple governed-writer model. Distributed synchronization must not be assumed to inherit those guarantees automatically.

## 2. Goals

A supported sync profile should eventually preserve:

- immutable memory-version identity;
- operation identity;
- provenance;
- valid/knowledge time;
- policy-relevant ownership;
- deletion/redaction intent;
- attachments/blobs;
- branch/snapshot lineage;
- explicit conflicts.

## 3. Non-goal

Morize does not select undocumented last-write-wins for durable memory.

A later bounded data type may explicitly choose LWW if:

- the value is low risk;
- semantics are documented;
- conflicts are not meaningful;
- the choice is independently tested.

## 4. Synchronization objects

A sync design is expected to exchange immutable or append-oriented operations/objects such as:

- source/observation identities where permitted;
- MemoryVersion objects;
- relations;
- mutation receipts;
- branch/snapshot frontiers;
- deletion/redaction markers;
- projection invalidation hints;
- blob references/content.

Derived indexes are rebuilt locally and do not need canonical replication.

## 5. Device / peer identity

Each synchronized writer/peer has stable identity and revocation state.

A peer identity is not the same as a human principal. Operations bind both where relevant.

Required planning fields:

```text
peer_id
principal_binding
device_or_service_identity
key_or_auth_binding
created_at
revoked_at?
last_known_frontier
```

## 6. Frontier

Synchronization needs an inspectable notion of what a peer has observed.

The exact mechanism—version vectors, operation frontiers, Merkle state, or another design—must be selected by experiment/controlled SpecGrain.

Requirements:

- no ambiguous “latest” derived only from wall-clock time;
- monotonic observation where possible;
- deterministic gap detection;
- bounded reconciliation;
- stale-peer handling.

## 7. Conflict classes

Conflicts are typed.

### C1 — Concurrent content update

Two peers create different successors from the same expected version.

Result:
- preserve both;
- mark conflict;
- deterministic policy may suggest merge;
- no silent overwrite.

### C2 — Update vs forget

One peer updates while another issues forget.

Result:
- protected deletion semantics require dedicated policy;
- an update must not casually resurrect forgotten content.

### C3 — Update vs redact

Redaction has privacy/safety precedence according to policy. Sync must avoid propagating prohibited plaintext after redaction becomes authoritative.

### C4 — Concurrent relation edits

Different graph relationships may coexist unless semantically exclusive.

### C5 — Policy divergence

A peer operating under stale/revoked policy cannot donate authority to shared canonical state merely because its operation is well formed.

### C6 — Scope/ownership change

Moving data across scopes is not an ordinary content merge; it requires authority evaluation.

## 8. Time

Wall-clock timestamps support UX and temporal semantics but do not alone resolve distributed causality.

Clock skew must not silently decide conflict winners.

## 9. Offline operation

A peer may work offline when its profile permits it.

Offline operations record:

- base/frontier;
- expected memory versions;
- policy identity;
- principal/peer identity;
- operation identity.

Reconnect performs reconciliation before dependent operations are treated as globally settled.

## 10. Forget and redaction propagation

This is a controlled/high-risk surface.

The design must define:

- deletion marker identity;
- which canonical plaintext is removed;
- how derived projections are invalidated;
- how offline peers learn the deletion;
- what happens when a stale peer tries to re-upload removed plaintext;
- backup/retention boundaries;
- external-export limitations.

A stale peer cannot automatically resurrect content after a recognized protected deletion.

## 11. Blob synchronization

Content-addressed blobs allow deduplication, but transfer rules still enforce:

- scope authorization;
- sensitivity;
- size limits;
- integrity digest;
- encryption;
- cancellation/resume;
- retention/deletion.

A digest match is not authorization.

## 12. Encryption

Transport encryption is required for network synchronization.

End-to-end encryption may be supported depending on product profile.

Before E2EE adoption, define:

- key ownership;
- device addition/revocation;
- recovery;
- multi-user sharing;
- server-side indexing limitations;
- metadata leakage;
- rotation.

Do not claim E2EE merely because transport uses TLS.

## 13. Managed sync server

A future Morize-managed sync service may coordinate frontiers and object transfer.

Server reachability does not grant memory authority.

The server enforces:

- tenant identity;
- quotas;
- entitlement;
- storage policy;
- rate limits;
- authenticated peer bindings.

Canonical semantic validation remains Morize-owned.

## 14. Sync vs backup

Backup:
- point-in-time or scheduled recovery copy;
- not necessarily bidirectional;
- conflict resolution not normally required.

Sync:
- bidirectional or multi-writer convergence;
- causality/conflict/deletion semantics required.

Never market one as the other.

## 15. Sync verification

Required test families before supported multi-device/team sync:

- concurrent same-base edits;
- update/delete races;
- redaction vs stale peer;
- offline long-lived peer;
- revoked peer;
- clock skew;
- dropped/reordered/duplicated messages;
- partial blob transfer;
- network partition;
- server restart;
- duplicated operation replay;
- corrupted operation;
- cross-tenant object attempt;
- schema version mismatch;
- branch merge conflicts.

## 16. Performance and bounds

Sync protocols require finite:

- batch size;
- object size;
- retries;
- in-flight bytes;
- reconciliation depth;
- retained operation metadata.

A malicious or very stale peer must not force unbounded history transmission.

## 17. Implementation decision gate

P12 may not implement sync until a dedicated SpecGrain freezes:

- operation/frontier model;
- conflict rules;
- deletion/redaction precedence;
- peer identity/auth;
- encryption profile;
- storage semantics;
- failure/retry model;
- migration/compatibility;
- test oracle.

The roadmap entry is not sufficient authority.
