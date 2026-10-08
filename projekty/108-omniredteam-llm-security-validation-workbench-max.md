# Project 108 — OmniRedTeam LLM Security Validation Workbench MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build a dedicated defensive workbench for evaluating LLM and agent security under controlled, reproducible conditions. The workbench turns adversarial-security hypotheses into test cases, isolated executions, telemetry, evidence and regression results without becoming an offensive intrusion platform.

## Why this is distinct
P54 owns epistemic/adversarial control. P60 owns broad adversarial multimodal AI security. P72 owns reusable cross-framework assurance and runtime verification. P100 owns the broader NeXus agentic development environment. P108 owns the **operator-facing security-validation product boundary**: controlled LLM red-team evaluation, replay, telemetry, regression and evidence.

## Source foundation
The supplied Omni Red-Team Platform is a concrete Tauri + Rust + React/TypeScript desktop architecture with local GGUF model lifecycle, streaming token telemetry, attack/evaluation levels, SQLite logs and event-driven security status. The source also exposes prototype-risk defaults such as unrestricted Tauri allowlisting, missing CSP, embedded test secrets and mocked inference; these are treated as hardening fixtures, not production patterns.

The supplied OmniCore Pathfinder adds native Rust AM-MAS orchestration, DAG execution, Event Sourcing, sqlite-vec semantic memory, llama.cpp inference, token telemetry and Linux namespace isolation. Its absolute-security claims and direct shell-execution example remain unverified and are converted into verification requirements rather than trusted guarantees.

## Canonical architecture

```text
TEST CASE / MODEL / CONFIG
          ↓
CASE NORMALIZER + PROVENANCE
          ↓
MODEL / RUNTIME ADAPTER
          ↓
ISOLATED TEST EXECUTION
          ↓
TOKEN / LATENCY / TOOL TELEMETRY
          ↓
SECURITY DETECTORS + INVARIANTS
          ↓
AUTHORITATIVE TEST OUTCOME
          ↓
EVIDENCE PACKET + REPLAY RECORD
          ↓
REGRESSION / DRIFT ANALYSIS
          ↓
P54 / P72 ASSURANCE GATES
```

## Core capabilities

### 1. Test corpus
Versioned cases for prompt injection, context manipulation, tool/capability confusion, memory poisoning, policy conflict, unsafe output handling and trajectory-level regressions. Cases are bounded to authorized test environments.

### 2. Model/runtime adapters
Support local GGUF/llama.cpp and other approved runtimes through a common adapter contract. Runtime identity, model version, configuration and policy version are recorded with every run.

### 3. Telemetry
Record token latency, throughput, model lifecycle events, tool events, sandbox status, detector results and authoritative test outcomes. Requested configuration is never treated as proof of actual execution.

### 4. Replay
Persist sufficient run state to reproduce a security finding without requiring private chain-of-thought. Deterministic fixtures use fixed seeds where applicable; nondeterministic behavior records the relevant uncertainty and environment metadata.

### 5. Evidence
Every finding links to the source case, runtime/model version, execution trace, detector version, observed outcome and verification status. `observed`, `inferred`, `hypothesized` and `verified` remain separate.

### 6. Regression / drift
Compare model/runtime versions over failure classes, detector results, latency, tool usage, policy denials and verification failures. Drift is an alert condition, not proof of compromise.

## Security boundary

P108 never grants production capability. Test authorization is explicit and scoped to a laboratory target. No credential theft, authentication bypass, covert persistence, destructive exploitation or unauthorized device/account access is implemented.

The workbench may model exploit classes as **non-destructive defensive fixtures** so that detection and containment can be measured. Source references to bypasses or undocumented interfaces remain threat-model inputs only.

## Production-hardening requirements

The source code is not accepted as production-ready without:
- least-privilege Tauri capabilities;
- explicit CSP;
- secret-free source and fixtures;
- real model execution replacing mock output;
- restricted network egress;
- sandbox resource limits;
- authenticated and integrity-protected run records where required;
- adapter isolation;
- authoritative postcondition checks;
- reproducible build and test manifests;
- negative tests for detector failure and telemetry loss.

## Verification program

1. Schema and test-case conformance.
2. Local GGUF/model lifecycle validation.
3. Token/latency telemetry accuracy against independent timing.
4. Detector precision, recall and false-positive characterization.
5. Prompt/context/tool-injection regression suite.
6. Memory-poisoning and trajectory-replay fixtures.
7. Sandbox isolation and restricted-egress tests.
8. Event-Sourcing replay consistency.
9. Evidence provenance and integrity checks.
10. Version-to-version regression analysis.
11. Resource exhaustion and rate-limit tests.
12. UI/runtime state consistency.
13. No-secret/no-credential-leak tests.
14. Failure-injection tests for telemetry, storage and model lifecycle.

## Integration

- P54 — epistemic/adversarial control and evidence calibration.
- P60 — adversarial multimodal security corpus.
- P61 — native OmniCore/runtime patterns.
- P72 — canonical assurance events, authorization and postcondition verification.
- P100 — NeXus model hub, agent registry and developer-environment integration.
- P102 — cloud migration/compliance where cloud test infrastructure is used.

## Maturity
**PROPOSED / ARCHITECTURE_BASELINE.** The supplied code is an implementation witness, not independent evidence of production readiness or absolute security.

## Derived from
P54, P60, P61, P72, P100.

## Primary source witnesses
- `omni_redteam_architecture_blueprint_260603_120915.pdf`
- `omnicore_am_mas_blueprint_260603_123615.pdf`

---

## Knowledge evolution — batch 14 / 2026-10-08: controlled GCG evaluation run schema

```yaml
DefensiveAdversarialEvaluation:
  source_lineage: batch-14-GCG
  model_revision: null
  tokenizer_revision: null
  authorization_ticket: null
  fixture_class: synthetic_defensive
  attempt_budget: 0
  refusal_and_false_positive_metrics: null
  tool_access_denied_by_external_policy: true
  independent_evidence_ref: null
  status: NOT_RUN
```

Source mechanism descriptions are adversarial research, not permission to compromise hosted models or remove safety enforcement. Never save operational harmful suffix payloads in the public portfolio. P108 owns controlled test execution while P60 owns threat taxonomy. Report what genuinely ran; no live security evaluation in this batch.
