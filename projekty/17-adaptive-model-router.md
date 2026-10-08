# 17 — Adaptive Model Router

## Objective
Create a cross-provider routing layer that chooses the least expensive / lowest-latency execution profile capable of satisfying an evaluated quality threshold, with escalation when uncertainty or verification failure rises.

## Source-derived basis
The supplied Gemini performance material emphasizes context caching, batch processing, streaming, asynchronous execution and RAG/tuning as system-level performance levers. The Claude-vs-Gemini comparison is retained only as historical evidence that model specialization can differ by task; it must not be treated as a permanent provider ranking.

## Architecture

```text
Request
  ↓
Task / Intent Classifier
  ↓
Policy Engine
  ├── latency target
  ├── quality target
  ├── modality
  ├── privacy boundary
  ├── consequence level
  ├── local resource profile
  └── budget
  ↓
Execution Profile Router
  ├── edge model
  ├── local model
  ├── fast cloud model
  ├── reasoning model
  └── specialist model
  ↓
Verifier
  ├── schema
  ├── tests
  ├── evidence
  └── confidence
  ↓
Escalate / Finalize
```

## Routing policy

1. Classify the task and consequence level.
2. Resolve privacy and data-egress constraints before selecting a provider.
3. Search for the minimum capability set and eligible execution profiles.
4. Prefer edge/local execution for privacy-sensitive, low-latency or offline work when evaluation permits it.
5. Allocate a bounded reasoning budget.
6. Verify the result.
7. Escalate only when the quality gate fails, uncertainty exceeds policy, or the selected execution profile lacks a required capability.
8. Record routing decisions for later optimization.

## Execution-profile model

A route is not only a model choice. It is:

```yaml
profile:
  location: edge | local | cloud
  provider:
  model:
  reasoning_budget:
  context_budget:
  allowed_tools: []
  privacy_class:
  cost_budget:
  latency_budget:
  verification_level:
```

This makes the router compatible with Project 37's sovereign edge fabric.

## Context economics

Context is treated as a bounded resource:

```text
STATIC KNOWLEDGE
+ TASK CONTEXT
+ RETRIEVED EVIDENCE
+ TOOL RESULTS
+ CONVERSATION STATE
      ↓
CONTEXT BUDGET
```

Prefer cache reuse, retrieval and task-focused context before blindly increasing model size. The performance source explicitly highlights caching and efficient execution as optimization levers.

## Model adapters

All providers implement a common adapter:

`generate`, `stream`, `structured_output`, `tool_call`, `usage`, `estimate_cost`, `health`, `cancel`.

A provider may be local, edge or remote. Provider substitution must not change application policy or authorization.

## Reliability modes

Support explicit execution modes:

`FAST | PRIVATE | OFFLINE | VERIFIED | HIGH_ASSURANCE`

Examples:

- `PRIVATE + HIGH_ASSURANCE` → local model, local retrieval, deterministic verification, no external MCP.
- `FAST + LOW_CONSEQUENCE` → edge inference with lightweight verification.
- `HIGH_ASSURANCE + complex` → compound reasoning, multiple checks and possible human review.

## Metrics

Track:

- verified quality by task class;
- cost per successful task;
- p50/p95 latency;
- cache hit rate;
- retrieval precision/coverage;
- escalation rate;
- tool error rate;
- recovery success;
- external data egress;
- local completion rate;
- user correction rate.

## Safety

Routing never bypasses policy or safety controls. A cheaper or less constrained provider cannot be selected merely because it produces a result more readily. Provider-specific safety posture remains explicit, versioned and auditable.

## 2026-09-09 evolution — compound reasoning execution profiles

The new Pro Loop material makes compound inference a first-class routing profile rather than merely a property of one model.

A compound route is now conceptually:

```yaml
compound_profile:
  base_execution:
  cache:
    mode: explicit | implicit | none
    version:
    ttl:
  fanout:
    max_paths:
    concurrency_limit:
    jitter:
  aggregation:
    mode: majority | weighted | semantic | critic
    early_exit:
  verification_level:
  cost_budget:
  latency_budget:
```

Routing should select this profile when the expected value of additional bounded inference exceeds the cost/latency budget and when the task benefits from disagreement analysis. Explicit cache compatibility is part of provider capability discovery; it is not assumed globally.

The router must also distinguish:

`MODEL ESCALATION` from `COMPUTE ESCALATION`.

A request can remain on the same model while increasing the number of bounded branches, or it can move to a stronger model when compound inference fails its quality gate.

Operational telemetry should include cache hit/miss, branch completion/failure, aggregate disagreement, early-exit reason and cost per verified result.

Historical provider prices in the source report are not used as current routing constants; live pricing and benchmark data remain required.

## Definition of done

The router is complete when model/provider choice is replaceable, execution profile is explicit, context/cost/latency are measurable, verification is mandatory at the configured assurance level, compound fan-out is rate-limited and observable, and no routing decision can weaken authorization or security policy.

---

## 2026-10-08 — batch 18: public web source evolution

Gemma 4 official 2026-04-02 announcement describes E2B/E4B/26B-MoE/31B-Dense variants. Add LocalOrCloudRoutingPolicy inputs: task, modality, source privacy, available local RAM/NPU/runtime, licensing+weight SHA, expected token/latency/cost budget and independent result verification. `gemini-embedding-2` is for retrieval/embedding, not chat completion. No device performance, GGUF packaging, remote Gemini pricing or access has been benchmarked here.
