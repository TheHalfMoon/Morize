# Founder Cost and Commercial Boundary

> Historical filename: `ZERO_COST_CONTRACT.md`. The binding meaning is **zero mandatory founder burn**, not a promise of free service to every user.

## 1. Intent

Morize must be buildable, testable, developed, and dogfooded by the founder without creating a mandatory recurring infrastructure bill before the project has revenue or separately approved funding.

This is a **founder operating-cost constraint**.

It is not:

- a promise that every Morize product or service will always be free;
- a restriction on charging users;
- a restriction on commercial hosting;
- a restriction on enterprise offerings;
- a restriction on using paid infrastructure once revenue/funding justifies it;
- a restriction on users choosing paid model or infrastructure providers.

## 2. Open-source and commercial model

The Morize source repository is licensed under Apache License 2.0.

The open-source license and future commercial strategy are separate concerns.

Morize may later offer paid products and services such as:

- Morize Cloud;
- managed synchronization and backup;
- hosted memory infrastructure;
- managed embeddings, reranking, or inference;
- team and organization administration;
- enterprise identity, governance, compliance, audit, and policy features;
- premium connectors;
- managed high-availability deployments;
- observability and analytics;
- support, SLA, onboarding, migration, and consulting;
- usage-based or seat-based plans.

Nothing in this founder-cost contract promises that hosted or managed services will be free.

## 3. Founder-zero-burn baseline

Before revenue or explicit budget authorization, the founder must be able to perform the core engineering loop without paid services:

1. clone and build Morize;
2. run the deterministic core;
3. run unit/integration/property/fuzz tests that belong to ordinary development;
4. initialize local vaults;
5. exercise local search, temporal memory, evidence graph, import/export, and recovery;
6. develop and test MCP/local client integrations;
7. run local benchmarks on bounded fixtures;
8. inspect and review changes;
9. package local development builds;
10. maintain project planning and evidence.

The project should prefer:

- local developer hardware;
- open-source dependencies;
- repository-hosted artifacts;
- free public-repository CI capacity where sufficient;
- deterministic synthetic fixtures;
- optional local models;
- local SQLite/filesystem infrastructure.

## 4. No accidental founder bill

Before a paid service is introduced into a required development or release path, the owning SpecGrain must identify:

- the service;
- why the free/local path is insufficient;
- expected monthly and per-unit cost;
- cost trigger and upper bound;
- who pays;
- whether customer revenue covers it;
- a local/self-hosted fallback when practical;
- shutdown/degradation behavior if the service is unavailable or budget is exhausted;
- data/privacy implications;
- vendor lock-in and migration plan.

A dependency that can generate an unbounded bill requires an explicit budget guardrail.

## 5. Commercially aware architecture

Morize should preserve a clean product boundary:

```text
Apache-2.0 Open-Source Core
  |
  +-- Local/self-hosted product
  |
  +-- Public SDKs / MCP / APIs
  |
  +-- Optional provider adapters
  |
  +-- Optional managed commercial services
         |
         +-- Cloud sync
         +-- Hosted memory
         +-- Team/enterprise control plane
         +-- Managed inference/search
         +-- Premium operations/support
```

The open-source core must not be deliberately crippled merely to force a hosted subscription. Commercial value should come from convenience, scale, operations, collaboration, governance, service levels, and managed infrastructure.

This principle does not prohibit differentiated managed capabilities whose operation genuinely requires hosted infrastructure.

## 6. User cost is a product decision

Users may choose among profiles:

### Local / self-hosted

Users operate Morize on infrastructure they control. Their costs are their own hardware/infrastructure/model costs.

### Bring-your-own-provider

Users may connect paid model, storage, graph, vector, or cloud providers and pay those providers directly.

### Managed Morize

A future Morize-operated service may charge users under published pricing and usage limits.

### Enterprise

A future enterprise product may use negotiated pricing, support, deployment, compliance, or SLA terms.

The architecture must not hard-code a permanent pricing model.

## 7. Core technical independence

Even with future commercial services, these correctness properties remain owned by Morize contracts rather than by billing status:

- durable record identity/version semantics;
- provenance representation;
- temporal truth semantics;
- conflict/supersession semantics;
- export format;
- schema validation;
- bounded input handling;
- authorization contracts;
- deterministic recovery rules.

Paid tiers may provide scale and operations; they must not silently redefine what a Morize memory means.

## 8. Optional paid infrastructure

A paid service may become a supported or recommended production option when justified.

It must remain explicit in:

- configuration;
- cost ownership;
- credential handling;
- data boundary;
- failure behavior;
- telemetry;
- retention;
- migration/export.

Local failure must not silently cause a chargeable fallback.

## 9. CI and release economics

Public CI may use free hosted capacity while available. If CI scale later exceeds free capacity, the project may:

- optimize test partitioning;
- move expensive suites to scheduled/manual runs;
- use self-hosted runners;
- fund CI from revenue/sponsorship;
- adopt paid CI through an explicit budget decision.

Correctness must not depend on hiding required checks merely to remain free.

## 10. Founder cost evidence

Each release planning cycle should record material recurring project-operated costs, even when the value is zero.

Suggested ledger:

```text
service
purpose
required_or_optional
current_monthly_cost
forecast_monthly_cost
billing_owner
budget_cap
revenue_backed
fallback
exit_plan
```

## 11. Invariants

```text
FOUNDER_ZERO_BURN != USER_FREE_FOREVER
OPEN_SOURCE != NO_COMMERCIAL_MODEL
PAID_SERVICE != ARCHITECTURAL_AUTHORITY
LOCAL_FAILURE != SILENT_PAID_FALLBACK
USER_REVENUE_MAY_FUND_INFRASTRUCTURE
COST_CHANGE_REQUIRES_EXPLICIT_OWNERSHIP
```

## 12. Current planning constraint

At the current pre-revenue planning stage:

- no mandatory paid runtime service is authorized for project development;
- no paid hosted dependency is required by the deterministic implementation spine;
- future paid product/service design remains explicitly allowed;
- any material founder-paid recurring dependency requires a separately justified SpecGrain and documented budget authority.
