# Project 115 — OmniAutonomous Engineering Intelligence Orchestrator MAX

**Status:** PROPOSED  
**Maturity:** ARCHITECTURE_BASELINE

## Mission

A model-agnostic orchestration layer that turns an AI development environment into a bounded autonomous engineering lifecycle: persistent memory, simulation, planning, implementation, execution, verification, reflection, skill learning, regression and reversible promotion.

## Boundary

P115 owns lifecycle orchestration. It does not replace:

- **P100** — NeXus AI Code OMEGA-X development environment;
- **P111** — enterprise agent platform/control plane;
- **P114** — cognitive memory and verification substrate;
- **P108** — LLM/agent security validation workbench;
- **P80** — self-evolving computing substrate;
- **P94** — adaptive problem-solving/discovery fabric.

## Canonical loop

```text
INTENT
  ↓
TASK GRAPH
  ↓
MEMORY + PROVENANCE
  ↓
SIMULATION / TRACE PREDICTION
  ↓
MULTI-PATH PLANNING
  ↓
MUTATION / GENERATION
  ↓
SANDBOX EXECUTION
  ↓
OBSERVATION + TESTS
  ↓
FORMAL / NEURO-SYMBOLIC VERIFICATION
  ↓
REFLECTION
  ↓
SKILL + MEMORY UPDATE
  ↓
HOLDOUT REGRESSION
  ↓
PROMOTION / ROLLBACK
```

## Architecture

### 1. State and memory

Use P114 as the substrate for episodic, semantic, procedural and test-time memory. Every durable mutation carries provenance, version and conflict-resolution metadata.

### 2. Planning and simulation

A CWM-compatible adapter predicts relevant execution-state changes before physical execution. A bounded MCTS or equivalent planner can explore candidate trajectories where simulation fidelity is sufficient.

Simulation output is advisory. It is never execution evidence and cannot satisfy a postcondition by itself.

### 3. Engineering mutation

Generators may produce code, configuration, tests or skills. Every generated artifact is untrusted until it passes the applicable execution and verification gates.

### 4. Verification

Combine conventional tests with symbolic invariants and counterexamples where applicable. A failed symbolic obligation blocks promotion rather than being silently ignored.

### 5. Reflection

MARS-like reflection converts failure traces into candidate lessons. Lessons are not automatically trusted: they enter a candidate memory/skill state and must pass regression before promotion.

### 6. Evolution

AlphaEvolve-like mutation/selection can optimize bounded engineering objectives. Population-based search, fast filters and full evaluation are optional mechanisms, not guarantees of improvement.

### 7. MCP environment control

Adapters can expose authorized capabilities for Cursor, Unity, Unreal Engine and other development systems. Capability exposure does not grant authority to perform consequential actions.

### 8. Evidence

Every autonomous cycle should be replayable from an evidence packet containing task identity, model/runtime versions, retrieved memory references, generated artifacts, tool calls, observations, test results, verification results, decision and rollback information.

## Audit-grade contracts — Iteration 36

P115 now treats the lifecycle as a set of explicit state transitions rather than an informal agent loop.

### Authoritative state

The orchestrator must distinguish:

- requested intent;
- compiled task graph;
- retrieved memory snapshot;
- simulated trajectory;
- generated candidate;
- sandbox execution state;
- observed runtime state;
- verification verdict;
- candidate lesson;
- promoted state;
- rollback state.

Only authoritative readback may establish the resulting runtime state. UI state, model claims, simulation predictions and tool responses are observations or proposals until validated.

### Evidence packet minimum

Each autonomous cycle should persist a deterministic correlation identifier and:

```text
cycle_id
parent_cycle_id
project_id
intent_hash
task_graph_hash
memory_snapshot_hash
model_runtime_identity
tool_capability_snapshot
candidate_artifact_hash
sandbox_identity
execution_trace_hash
test_result_hash
verification_result_hash
reflection_hash
promotion_decision
postcondition_result
rollback_reference
```

The packet is evidence of the process and its observations; it is not itself proof that an engineering claim is true.

### Promotion gate

Promotion is allowed only when all applicable gates succeed:

`authorization ∧ execution_success ∧ tests_pass ∧ verification_pass ∧ holdout_pass ∧ postcondition_pass ∧ evidence_complete`.

Unknown authorization, incomplete evidence, failed verification or ambiguous postconditions fail closed.

### Memory and skill promotion

Reflection may propose a lesson, but a lesson becomes durable only after:

`candidate → provenance check → contradiction check → holdout regression → versioned promotion`.

A promoted lesson must be reversible and attributable to the cycle that created it.

## Safety and assurance

- model capability ≠ authorization;
- MCP capability ≠ authorization;
- memory ≠ policy;
- simulation ≠ execution evidence;
- generated artifact ≠ trusted artifact;
- reflection ≠ truth;
- benchmark claim ≠ reproduced result;
- promotion requires evidence;
- rollback must be deterministic and auditable.

## Research track

Titans, R3Mem, CWM, MCTS, AlphaEvolve and MARS are integrated as adapter/research concepts. Their source-reported numerical gains require independent reproduction before being used as engineering requirements.

## Security

Refusal-vector, CAST and ablation material from the source is restricted to defensive model-behavior research and P108/P72 assurance. P115 does not provide a production mechanism for disabling safety controls.

## Verification roadmap

1. deterministic replay harness;
2. memory provenance tests;
3. simulation-vs-execution calibration;
4. mutation sandbox;
5. holdout regression gate;
6. symbolic verification adapter;
7. reflection/lesson promotion gate;
8. skill versioning and rollback;
9. MCP authorization tests;
10. failure injection;
11. resource and latency budgets;
12. end-to-end autonomous-cycle audit;
13. evidence-packet schema conformance;
14. fail-closed testing for missing or ambiguous authority;
15. deterministic postcondition/readback tests;
16. cross-cycle stale-state and supersession tests.

## Exit criterion

P115 remains an architecture baseline until an independently reproducible implementation demonstrates that autonomous cycles can improve bounded engineering objectives without violating verification, authorization, provenance, rollback or security invariants.

---

## 2026-10-08 — batch 13: multi-agent WBS/HTA build-plane for OmniCore

Sources: two task-management PDFs and `--AI w Tworzeniu Systemów Operacyjnych-- (1).pdf`. Existing P115 engineering orchestrator owns this capability. Earlier P90 is the procedure compiler, P26 kernel assurance.

### Build-and-verify workflow
- `ProjectIntent`: objective, non-goals, budget, sponsor, acceptance metrics and risk class.
- `DecompositionGraph`: HTA atomic operations + WBS deliverables, typed dependencies and critical path, no circular capability escalation.
- `AgentAssignment`: optional Cursor/Windsurf/Devin roles as *replaceable interfaces*, provider availability dynamically verified; no hard-coded agent abilities taken from a 2025 source PDF.
- `ExecutionBroker`: least privilege, scoped tokens, blocked public upload of proprietary or personal materials, budget ceilings, expiration and human approval for consequential actions.
- `VerificationPacket`: source/commit SHA, actual tool receipt, test log, independent reviewer, before/after diff, rollback proof and completeness state.
- `ExitCriteria`: fail closed on hidden scope changes, tests missing, stale information, unverifiable receipts or external side effect without approval.

**Acceptance:** a fully simulated build scenario, deterministic failure/rollback test and a clean-room reproducibility dossier; no actual GCP/Ring-0 access or Devin/Cursor/Windsurf orchestration was executed.

---

## Knowledge evolution — batch 14 / 2026-10-08: identical-text OmniCore reports and agent build-verification

Two files `Automatyzacja Tworzenia Oprogramowania z AI...` are byte-distinct (different SHA-256) but have identical extracted text after whitespace normalization. They count as **one source lineage** for P115, not proof from two independent authors.

`BuildRoleContract` defines expected output, task graph, agent capabilities, resource/secret scope, independent reviewer and provider version, independent of speculative Cursor/Windsurf/Devin vendor descriptions. `BuildEvidencePacket`: actual commit hash, compiler outputs, QEMU log, tool receipts, failure evidence, rollback and permission gate. Source proposes autonomous Ring-0 changes and vendor-linked agents; P115 does not grant the agent those powers. Textual "system built" is never equivalent to an installed and tested kernel.

---

## Knowledge evolution — batch 15 / 2026-10-08: ODYN MAS staged capability admission / no model self-authorization

The five versioned feasibility PDFs suggest Skills, HermesClaw, bitemporal memory, Dual LLM, Zero-Shot Tool Synthesis, ROS Noetic AlterEgo, Jevbridge MCP and later latency budgets, strict input validation, Semantic Caching, Throwaway Scripts. Source text says these were coded, but **repo-level verification of ODYN-AI has NOT been performed in this batch**.

Proposed `ToolProposalLifecycle`: `model_output → typed candidate → digest/hash → independently approved scope → non-executing admission gate → isolated interpreter/container → time/memory/network limit → test receipt → reviewed promotion/rollback`. Dynamic `importlib.exec_module` and `subprocess.run(["python3", path])` shown in source cannot be promoted merely by writing a temporary file or naming a function sandbox. Physical robot commands require a separate safety-rated policy and human supervisor. Hermes JSON formatting does not guarantee correct tools or 100% behavior.

**Actual implementation:** `tools/agent_tool_admission_gate.py` (non-executing, narrow allowlist) with 14 test cases; does **not** run arbitrary Python or replace a real sandbox. Broker rejects model-declared approval, unrecognized capabilities and mutable/harmful side effects. Provider/cloud integrations are future work, not completed by this markdown addition.

---

## Knowledge evolution — batch 16 / 2026-10-08

Feasibility (5)–(9) and 3 DGM reports define ModelScout→TechRecon→Strategist→DGM_Core with Hugging Face/GitLab, speculative 33+33 apps and GameBuilder. Treat S[t+1]=Phi(S[t],R(Omega_recon ∪ Omega_models)) as **notation, not a consistency proof**. DGMRun requires model hash+license, research citations, typed roadmap, candidate integration branch, independent CI receipts, human/tool-owner approval, rollback and readback; disallow auto-main updates or secret fallback tokens. Actual tools/dgm_cycle_gate.py is a NON-EXECUTING admission check with 13 unit tests. External model download/GitLab evolution not executed.

---

## 2026-10-08 — batch 18: public web source evolution

Gemini CLI/Antigravity SDK/CLI and Gemini Enterprise Agent Platform offer candidate agent coding/workflow interfaces. GoogleAgentExecutionPacket binds separately supplied tool scope/IAM, prompt/source provenance, source+commit revisions, isolated runtime, rate/cost cap, actual tool+CI receipts, human approval and rollback; agents may never infer rights from `authuser` or model-studio links. Any hosted paid operation must pass independent approval **in addition** to `tools/cloud_operation_budget_gate.py`'s offline plan review. No external agent installed/connected.
