# Project 72 — OmniCore Cross-Framework Agent Assurance & Runtime Verification Fabric MAX

## Status

**PROPOSED → ARCHITECTURE BASELINE**

This project is created as an explicit operationalization layer for capabilities already distributed across the portfolio. It does **not** replace Projects 47, 58, 60, 68 or 69.

The project exists because the repository already contains mature principles for capability governance, monitorability-aware oversight, adversarial continuity and constitutional runtime control, while Project 59 defines the execution lifecycle. The missing reusable layer is a framework-neutral assurance fabric that can observe, replay, verify and compare those controls across concrete agent runtimes.

## 1. Problem

Agent systems increasingly combine model-directed planning, tools, memory, workflows, external systems and long-running state. Framework-specific traces and lifecycle semantics make it difficult to answer consistently:

- what the agent was allowed to do;
- what capability it actually invoked;
- which state transition occurred;
- whether an external side effect really happened;
- whether the claimed evidence supports the action;
- whether the same trajectory reproduces the failure;
- whether production behavior has drifted from evaluated behavior;
- whether reduced observability should increase verification requirements.

The portfolio contains these principles individually. Project 72 turns them into a reusable assurance boundary.

## 2. Design objective

```text
FRAMEWORK / MODEL / TOOL IMPLEMENTATION
                ↓
        ASSURANCE ADAPTER
                ↓
        CANONICAL EVENT MODEL
                ↓
       POLICY / CAPABILITY CHECKS
                ↓
      STATE / TRAJECTORY REPLAY
                ↓
  INDEPENDENT POSTCONDITION CHECK
                ↓
       EVIDENCE + RISK RECORD
                ↓
       EVAL / DRIFT / INCIDENT LOOP
```

The system is provider- and framework-neutral at the assurance layer.

## 3. Non-goals

Project 72 does not:

- replace an agent framework;
- become the authoritative business database;
- treat chain-of-thought as a required or trusted security primitive;
- grant capabilities merely because an adapter observes them;
- silently alter production behavior without explicit policy;
- claim formal verification for arbitrary agent behavior.

## 4. Core architecture

### Assurance planes

**Collection plane**

Normalizes framework-specific telemetry, tool events, policy checks, state transitions and external confirmations.

**Verification plane**

Performs schema checks, authorization checks, invariants, trajectory consistency, evidence validation and postcondition verification.

**Replay/evaluation plane**

Stores reproducible test cases and event sequences for regression, adversarial replay, drift analysis and incident reconstruction.

**Governance plane**

Maps assurance findings to project identity, policy version, capability version, agent version and evidence provenance.

## 5. Canonical event model

The minimum event contract is:

```yaml
AssuranceEvent:
  event_id:
  timestamp:
  trace_id:
  run_id:
  actor:
  framework:
  agent_id:
  agent_version:
  task_id:
  state_before_hash:
  state_after_hash:
  action_type:
  capability_id:
  authorization_decision:
  policy_version:
  input_ref:
  output_ref:
  evidence_refs: []
  side_effect:
    requested: false
    confirmed: false
    confirmation_ref:
  verification:
    status:
    postcondition_ref:
  risk_class:
  monitorability:
  provenance:
```

The event model records facts and references. It does not require private reasoning traces.

## 6. Assurance contract

Every consequential action is evaluated through:

```text
DISCOVER
  ↓
IDENTIFY CAPABILITY
  ↓
CHECK AUTHORIZATION
  ↓
CHECK PRECONDITIONS
  ↓
EXECUTE / OBSERVE
  ↓
AUTHORITATIVE READBACK
  ↓
CHECK POSTCONDITION
  ↓
RECORD EVIDENCE
```

Read-only retrieval may use a lighter path. Mutations, privileged operations and external side effects require the full path.

## 7. Runtime invariants

1. Observation does not grant authority.
2. Capability identity is explicit.
3. Authorization is evaluated independently from model output.
4. Every consequential state transition has a verifiable postcondition.
5. A reported tool result is not equivalent to authoritative external completion.
6. Stale or conflicting state cannot silently overwrite newer state.
7. Missing observability never lowers the security requirement.
8. Evidence provenance remains distinguishable from inference.
9. Replay artifacts preserve enough context to reproduce the relevant decision path without requiring private chain-of-thought.
10. Framework adapters cannot weaken global assurance policy.

## 8. Verification classes

### Contract verification

Validate tool schemas, event schemas, version compatibility and required fields.

### Authorization verification

Compare requested capability, effective authorization, scope, expiry, resource limits and approval state.

### State verification

Validate legal state transitions, version monotonicity, stale-write protection and terminal-state conditions.

### Outcome verification

Confirm the external system or authoritative state changed as expected.

### Evidence verification

Check provenance, freshness, source identity, contradiction signals and evidence-to-claim linkage.

### Security verification

Replay prompt/tool injection, memory poisoning, encoded or multilingual attacks, capability confusion and trajectory-level attack patterns.

### Monitorability verification

Measure whether critical observables are available, trustworthy and independent. Degraded monitorability increases required external checks.

## 9. Drift detection

Compare evaluation and production distributions over:

```text
failure modes
capability usage
policy denials
verification failures
latency
cost
trajectory length
human approvals
retries
external side-effect discrepancies
```

Drift is an alert condition, not automatic proof of degradation or compromise.

## 10. Replay model

A replayable case consists of:

```yaml
ReplayCase:
  case_id:
  agent_version:
  policy_version:
  environment_class:
  input_refs: []
  event_refs: []
  expected_invariants: []
  expected_postconditions: []
  attack_class:
  outcome_class:
```

Replay should support deterministic fixtures, controlled nondeterminism and explicit seed capture where applicable.

## 11. Integration with existing projects

| Project | Integration |
|---|---|
| 47 | Canonical project/capability identity and lineage |
| 58 | Monitorability-aware oversight and verification escalation |
| 60 | Adversarial multimodal and resilience testing |
| 68 | Constitutional runtime and control-plane boundaries |
| 69 | Temporal / mobile / endpoint continuity defense |
| 13 / 28 | Verified code and release gates |
| 29 / 30 / 32 | Evidence provenance and Zero-Trust data operations |
| 35 / 36 / 43 | Intent, influence and human-agency signals |
| 40 / 41 | Repository and multimodal action telemetry |
| 59 | HTA/WBS, deterministic execution and authoritative commit flow |

## 12. Implementation architecture

A practical first implementation can remain a modular monolith:

```text
assurance-core/
├── domain/
│   ├── events
│   ├── policies
│   ├── capabilities
│   ├── state
│   └── verification
├── adapters/
│   ├── framework-a
│   ├── framework-b
│   └── generic-http
├── replay/
├── evaluation/
├── drift/
├── evidence/
├── audit/
└── cli/
```

A distributed architecture is justified only when throughput, isolation or organizational boundaries require it.

## 13. Security model

The assurance service is itself a security-sensitive component.

Required controls:

- least privilege;
- signed/verified configuration where appropriate;
- secret isolation;
- tenant/run isolation;
- immutable audit records for consequential findings;
- rate limiting;
- input size/resource limits;
- strict adapter sandboxing for untrusted telemetry;
- no arbitrary network execution from an event parser;
- provenance-preserving normalization;
- explicit retention controls.

## 14. Evaluation suite

Minimum initial suites:

```text
contract-regression
authorization-boundary
state-transition
postcondition-verification
prompt-injection
memory-poisoning
trajectory-replay
monitorability-degradation
production-eval-drift
adapter-compatibility
```

Each suite must contain both positive and negative cases.

## 15. Measurable outcomes

The project is successful when it can demonstrate, with reproducible tests:

- normalized traces from at least two concrete execution surfaces;
- deterministic capability/authorization decisions;
- detection of stale or inconsistent state transitions;
- independently verified consequential postconditions;
- replayable trajectory-level security regressions;
- measurable monitorability degradation;
- production/evaluation drift signals;
- evidence packets that allow incident reconstruction without relying on model self-reporting.

No fixed accuracy, latency or coverage number is assumed before benchmarking.

## 16. Roadmap

### Phase A — contract foundation

Implement canonical schemas, event validation, capability identifiers and invariant engine.

### Phase B — adapter layer

Implement two materially different runtime adapters and normalize their traces into one contract.

### Phase C — verification engine

Add authorization, state, evidence and postcondition verification.

### Phase D — replay/evaluation

Add deterministic fixtures, attack replay and regression storage.

### Phase E — drift/monitorability

Add production-vs-evaluation comparison and observability quality metrics.

### Phase F — formalization

Apply formal methods selectively to critical state machines and authorization invariants after concrete runtime behavior is available.

## 17. Evidence classification

**SOURCE_DERIVED:** Project 59 defines deterministic execution, authoritative readback, postcondition verification and monitorability-aware behavior; Projects 58, 60, 68 and 69 provide adjacent oversight/security/control requirements.

**INFERRED:** A reusable cross-framework assurance layer is a natural operational gap between these capabilities and concrete runtime implementations.

**PROPOSED:** External market demand, framework coverage and quantitative superiority must be independently validated before the project is treated as a validated product opportunity.

## Definition of Done

- canonical event contract implemented;
- at least two runtime adapters;
- authorization checks independently evaluated;
- state-transition invariants executable;
- authoritative postcondition verification implemented;
- replayable security regression cases;
- monitorability quality metrics;
- drift detection with false-positive characterization;
- evidence-linked audit records;
- documentation synchronized with implementation;
- reproducible test commands and results;
- no secret material in source or fixtures.

---

## Knowledge evolution — batch 15 / 2026-10-08: externally reviewed tool admission before execution

The five `Analiza wykonalności nowych integracji i funkcji a...` reports propose Zero-Shot Tool Synthesis, hot `importlib` loading and **temporary execution of generated Python**. Neither a transient file nor a Python import boundary is a security sandbox. Protecting MCP, Computer Use, ROS hardware and credit/payment contexts requires an external, typed permission boundary **before** any generated code or tool can run.

Implemented proof: `tools/agent_tool_admission_gate.py`, a **non-executing** prototype that checks artifact SHA-256, externally supplied approvals by hash, capability allowlist, expiry, short timeout, read-only/no-network sandbox declaration and disabled side effects; unit tests include self-declared approvals, hash mismatch and privileged capability attempts. Passing the gate does **not** create an actual sandbox/cryptographic attestation and cannot replace human signature verification, OS-level process isolation or network egress firewall.

`TrustDecision` output is never promoted to real tool execution without independent sandbox, trusted issuer, telemetry, review and rollback. The token-manipulation source adds client-side state/API as untrusted inputs; an agent cannot grant itself credits or user permission.
