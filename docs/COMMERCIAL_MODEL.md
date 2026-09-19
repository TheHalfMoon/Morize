# Morize Commercial Model and Service Architecture

## 1. Position

Morize is an Apache-2.0 open-source project with room for commercial managed services.

The preferred monetization thesis is:

> Charge for operating Morize well at scale, not for obscuring the meaning or ownership of memory.

This document does not set prices.

## 2. Open-source core

The public core owns the semantic contracts:

- canonical memory identities/versions;
- temporal truth;
- provenance;
- contradiction/supersession;
- lifecycle actions;
- policy semantics;
- local/self-hosted persistence;
- retrieval contracts;
- public API/MCP schemas;
- import/export;
- backup/recovery semantics;
- source-admission and evidence rules.

The core is not intentionally crippled to make local/self-hosted use non-viable.

## 3. Potential paid surfaces

Future revenue may come from:

### Morize Cloud

- managed hosted memory;
- automatic upgrades;
- backup/disaster recovery;
- managed search/indexing;
- managed connectors;
- managed inference;
- synchronization.

### Team / Business

- shared organizations/projects;
- administrative governance;
- audit;
- collaboration/review workflows;
- higher service limits;
- managed integrations.

### Enterprise

- SSO/SCIM;
- policy administration;
- deployment assistance;
- private networking;
- region/data-residency options;
- advanced audit/export;
- support/SLA;
- enterprise procurement/security support.

### Services

- migration;
- onboarding;
- architecture consulting;
- integration work;
- premium support.

Exact features remain subject to future market evidence and legal/operational review.

## 4. Entitlement boundary

An entitlement decides whether a commercial service operation is available.

It does **not** change:

- what a MemoryVersion means;
- provenance semantics;
- temporal semantics;
- conflict semantics;
- export serialization;
- recovery truth.

Conceptual flow:

```text
request
 -> authenticate
 -> authorize data scope
 -> evaluate service entitlement/quota
 -> execute service operation
 -> canonical Morize semantics
```

Authorization and entitlement are separate decisions.

A customer may be authorized to read their data while not entitled to a premium operation. The system must return that distinction safely.

## 5. Metering boundary

Future usage-based billing may meter:

- stored bytes;
- managed vector/embedding work;
- inference tokens/compute;
- connector runs;
- synchronization transfer;
- API operations;
- organization seats;
- premium retention/backup tiers.

Metering events are operational/commercial records. They are not memory evidence.

Metering must not ingest private memory content merely to calculate usage when metadata/units are sufficient.

## 6. Quotas and cost protection

Managed services require:

- hard/soft quota semantics;
- per-tenant cost budgets;
- provider rate limits;
- customer-visible usage where practical;
- founder/operator global budget alerts/caps;
- no unbounded automatic retries against billable providers.

Quota exhaustion must not corrupt canonical state.

## 7. Billing failure

Billing system failure must be isolated from memory durability.

Examples:

- payment processor unavailable -> do not corrupt vaults;
- entitlement service unavailable -> fail protected premium operations safely;
- invoice failure -> apply published grace/restriction policy;
- cancellation -> preserve export/retention behavior promised by policy.

Destructive deletion is never triggered by an ambiguous billing callback.

## 8. Pricing independence

Do not hard-code plan names such as Free/Pro/Enterprise into canonical data schemas.

Use generic capability/entitlement identifiers so commercial packaging can change without data migration.

## 9. Provider economics

A managed feature that depends on an external paid provider must record:

```text
provider
unit cost
expected usage
customer price relationship
margin/risk assumption
budget cap
retry policy
fallback
exit plan
data boundary
```

Provider substitution cannot silently change privacy or memory semantics.

## 10. Free/self-hosted usage

Because the source is Apache-2.0, third parties may use and redistribute it subject to the license.

The business plan therefore should not depend on preventing self-hosting.

Commercial differentiation should favor:

- convenience;
- reliability;
- managed operations;
- collaboration;
- security administration;
- enterprise integration;
- support;
- performance at scale.

## 11. Hosted architecture separation

Keep service-specific concerns outside the deterministic memory core where possible:

```text
Morize semantic core
   ^
   |
service APIs
   |
+-----------------------------+
| auth / tenant control       |
| entitlements / quotas       |
| billing/metering            |
| managed infrastructure      |
| support/ops                 |
+-----------------------------+
```

A local build should not need payment/billing code to understand a vault.

## 12. Commercial launch gate

Before charging the first user, create a dedicated controlled SpecGrain covering:

- target customer and value proposition;
- pricing metric and plans;
- entitlement matrix;
- metering;
- payment processor;
- tax/VAT responsibilities;
- trial/free-tier policy if any;
- refund/credit policy;
- abuse/fraud handling;
- customer support;
- service limits;
- retention/export/cancellation;
- privacy/terms/subprocessors;
- security incident process;
- uptime/support promises;
- cost model and margin sensitivity;
- billing failure scenarios;
- end-to-end test environment.

No current planning document claims that commercial launch is already authorized.

## 13. Open-source trust

Commercialization must not create misleading repository claims.

Clearly distinguish:

- open-source features;
- self-hosted features;
- managed-only operations;
- experimental features;
- paid service entitlements.

Documentation should not imply that “open source” means the hosted service is free.

## 14. Strategic flexibility

Morize may evolve toward:

- fully service-based monetization;
- paid hosted tiers;
- enterprise contracts;
- sponsorship;
- dual distribution of operational services;
- paid support.

Changing business packaging should not require rewriting memory truth.
