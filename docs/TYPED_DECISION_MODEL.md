# Typed Memory Decision Model

## Purpose

Morize uses structured decisions with explicit uncertainty. Models may analyze evidence, but deterministic Morize policy owns authorization and durable side effects.

The design is informed by the useful philosophy of machine-native decision systems such as Jev: return decisions in a known schema, expose uncertainty, and keep thresholds/composition in ordinary code.

Morize implements this philosophy independently. Jev is not a required runtime dependency.

## Closed decision vocabulary

The initial durable-memory action vocabulary is:

- `STORE`
- `UPDATE`
- `MERGE`
- `SUPERSEDE`
- `CONTRADICT`
- `EXPIRE`
- `FORGET`
- `REDACT`
- `QUARANTINE`
- `IGNORE`
- `REQUIRE_REVIEW`

The vocabulary is versioned.

Unknown actions fail validation rather than being interpreted from prose.

## Decision envelope

A decision binds:

```text
decision_id
candidate_ref
action_set_version
selected_action
action_scores?
confidence?
abstained
reason_codes[]
evidence_refs[]
relevant_memory_versions[]
decision_engine_identity
policy_revision_id
created_at
```

The exact serialized schema is refined in P1/P4.

## Engine classes

### Deterministic rule engine

Preferred when exact evidence is sufficient.

Examples:

- exact duplicate;
- user-explicit operation;
- expiry;
- stale expected version;
- exact source deletion;
- deterministic validation failure.

### Local inference engine

Optional model/runtime on user or Morize-controlled infrastructure.

Its identity includes material model/runtime/configuration state.

Laya is a current P8 evaluation candidate for this class because its current model card describes bounded typed answers and calibrated probabilities instead of generated prose. Morize treats those properties as upstream claims until independently reproduced. Morize does not pre-authorize Laya as a dependency: admission requires an immutable artifact/runtime identity, Morize-specific calibration and abstention evidence, resource/cost qualification, and a replaceable adapter. Its confidence is evidence only and cannot grant mutation authority.

### Remote inference engine

Optional provider adapter.

Remote inference is explicit in data boundary and cost ownership. It is never a hidden fallback.

### Ensemble

Multiple independent signals may be combined through a versioned aggregation algorithm.

## Confidence

Confidence is evidence about a classifier/decision process.

It can influence:

- automatic progression threshold;
- review routing;
- abstention;
- benchmark calibration.

It cannot influence:

- principal identity;
- access rights;
- scope visibility;
- protected-policy bypass;
- stale-version overwrite;
- mutation authorization.

`CONFIDENCE != AUTHORITY`

## Reason codes

Stable reason codes are machine-consumable.

Examples:

- `EXACT_DUPLICATE`
- `NEWER_SOURCE`
- `VALID_TIME_CHANGED`
- `SOURCE_CONFLICT`
- `LOW_SOURCE_AUTHORITY`
- `CROSS_SCOPE_CONFLICT`
- `STALE_SOURCE`
- `EXPLICIT_USER_OPERATION`
- `INSUFFICIENT_EVIDENCE`
- `AMBIGUOUS_ENTITY`
- `POLICY_REVIEW_REQUIRED`

Free-form explanation may improve UX, but policy never parses model prose to discover authority.

## Abstention

Abstention is a first-class valid result.

When evidence is insufficient, Morize may:

- preserve only the observation;
- preserve competing propositions;
- quarantine the candidate;
- request review;
- request more evidence;
- return no durable change.

Invented certainty is worse than explicit uncertainty.

## Threshold ownership

Thresholds are versioned policy/configuration, not prompt text.

A threshold decision can depend on:

- action type;
- source authority;
- sensitivity;
- scope;
- reversal cost;
- current conflict state;
- calibrated confidence;
- required evidence count/class.

A threshold cannot bypass hard authorization constraints.

## Replay boundary

For every committed model-assisted mutation, preserve enough evidence to inspect:

- exact candidate;
- relevant prior memory versions;
- evidence references;
- engine identity;
- structured engine output;
- policy revision;
- threshold configuration;
- authorization outcome;
- resulting mutation/version.

A nondeterministic external model call is not falsely presented as bit-for-bit reproducible. Historical input/output identity is retained; deterministic downstream policy remains reproducible.

## Model-change rule

Changing any material decision-engine behavior may require a new engine identity, including:

- model/provider;
- model revision;
- prompt/template;
- tokenizer;
- quantization when material;
- classifier code;
- aggregation weights;
- structured-output schema;
- calibration mapping.

Cached decisions are not automatically valid under a new engine.

## Evaluation

Memory Lab evaluates:

- calibration error;
- selective accuracy at thresholds;
- abstention quality;
- false automatic durable-change rate;
- update/supersession/conflict classification;
- source-authority mistakes;
- review volume;
- cost/latency for optional inference;
- deterministic policy replay.

High benchmark accuracy does not grant runtime authority.

## Commercial/provider neutrality

Morize may support paid hosted inference in future managed services.

The typed-decision contract is provider-neutral so that:

- local inference;
- bring-your-own provider;
- Morize-managed inference;
- deterministic-only mode;

can share the same downstream governance semantics.

Pricing status never changes what a decision means.
