# Project 90 — OmniSOP Operational Knowledge & Procedure Compiler MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build a provenance-aware system that converts observed human work into validated, versioned and executable Standard Operating Procedures (SOPs), then continuously detects process drift and proposes controlled procedure updates.

The project is not a generic document generator. Its canonical object is an **operational procedure** with prerequisites, ordered actions, evidence, exceptions, risks, validation status and version history.

## Source-derived foundation
The supplied SOP material describes recording a real process, imposing a stable structure, using one action per step, adding screenshots and warnings, and validating the result with a user unfamiliar with the task. fileciteturn489file2L10-L37

The larger report frames SOP as a formalized organizational algorithm connecting strategy and execution and combines process engineering, cognitive considerations, documented information, multimodal AI and agent orchestration. fileciteturn489file3L18-L28

## Canonical pipeline
```text
WORK / SCREEN / VOICE OBSERVATION
        ↓
EVENT + ACTION EXTRACTION
        ↓
CONTEXT / INTENT INTERVIEW
        ↓
PROCESS GRAPH
        ↓
SOP IR
        ↓
DRAFT + SCREENSHOT / EVIDENCE LINKS
        ↓
NAIVE-USER VALIDATION
        ↓
RISK / COMPLIANCE REVIEW
        ↓
VERSIONED SOP RELEASE
        ↓
PROCESS DRIFT DETECTION
        ↓
CHANGE PROPOSAL
        ↺
```

## SOP Intermediate Representation
```yaml
SOP:
  id:
  version:
  purpose:
  scope:
  owner:
  prerequisites: []
  steps:
    - id:
      action:
      actor:
      input:
      expected_state:
      evidence_ref:
      warning:
      exception_paths: []
  outputs: []
  risks: []
  controls: []
  validation:
    naive_user_test:
    reviewer:
    date:
  provenance: []
  supersedes:
```

## Observation model
The recorder must distinguish visible UI actions from inferred intent. A click is evidence of a click; it is not automatically evidence of why the operator clicked it.

```yaml
Observation:
  timestamp:
  modality: screen|voice|text|camera
  actor:
  action:
  visible_state:
  source_ref:
  confidence:
  inferred_intent:
  inference_basis:
```

## Process graph
Each procedure is represented as a directed graph rather than only a linear document. This permits branching, exception handling, prerequisites, loops and recovery paths.

```text
START
 ↓
PRECONDITION
 ↓
ACTION
 ↓
EXPECTED STATE?
 ├── YES → NEXT STEP
 └── NO  → TROUBLESHOOT / ESCALATE
```

## Validation gates
1. **G0 — Source completeness:** every step has an observation or explicitly declared source.
2. **G1 — Structural completeness:** purpose, scope, prerequisites, procedure and troubleshooting exist.
3. **G2 — Action granularity:** one operational action per step where practical.
4. **G3 — Evidence linkage:** important UI states have screenshots/video/source references.
5. **G4 — Naive-user test:** an unfamiliar operator can complete the task without hidden coaching.
6. **G5 — Risk review:** ambiguous, irreversible or privileged actions are explicitly marked.
7. **G6 — Version release:** owner, version and provenance are recorded.

## Continuous change detection
The system compares a released SOP with later process observations:

```text
CURRENT SOP
    ↕
OBSERVED PROCESS
    ↓
DIFF
    ↓
SEMANTIC IMPACT
    ↓
CHANGE PROPOSAL
    ↓
REVALIDATION
    ↓
NEW VERSION
```

No observed deviation automatically overwrites the canonical procedure. The system must determine whether the deviation is operator error, legitimate process change, environmental variation or evidence of an obsolete SOP.

## Agent topology
- **Observer** — records multimodal process evidence.
- **Process Analyst** — constructs the process graph.
- **SOP Compiler** — emits the SOP IR and human-readable procedure.
- **Evidence Linker** — attaches screenshots, recordings and source references.
- **Risk Reviewer** — detects unsafe, irreversible and authorization-sensitive actions.
- **Validation Agent** — orchestrates naive-user and regression tests.
- **Drift Monitor** — compares future observations against released SOP versions.
- **Human Approver** — owns release of consequential procedure changes.

## Security and privacy
Screen recordings and process traces may contain credentials, personal information or proprietary material. The architecture therefore requires local redaction where practical, retention policies, access controls, provenance and explicit publication boundaries.

Credentials must never become durable SOP content. A procedure may say **where a credential is required** without recording the secret itself.

## Integration
- Project 08 — personal/agentic context and approval architecture.
- Project 19 — research and evidence orchestration.
- Project 24 — agent/tool infrastructure.
- Project 28 — verified code/document generation.
- Project 32 — evidence/provenance methodology.
- Project 47 — project identity and registry integrity.
- Project 61 — Omnis/CIRA/PUI convergence.
- Project 72 — runtime/agent assurance.

## Hard invariants
1. Observation is not intent.
2. Generated procedure is not validated procedure.
3. Semantic similarity cannot grant access to source material.
4. Credentials and secrets are never persisted as procedure content.
5. A process deviation cannot silently mutate the canonical SOP.
6. Human approval remains the authority for consequential release.
7. Every released SOP is versioned and rollback-capable.
8. Source evidence and generated prose remain distinguishable.

## Definition of Done
- multimodal process observation;
- process graph + SOP IR;
- provenance-linked generated instructions;
- screenshot/evidence binding;
- naive-user validation workflow;
- risk and authorization annotations;
- versioned release/rollback;
- process-drift detection;
- auditable change proposals;
- reusable agent/tool interface.

---

## 2026-10-08 — batch 12: SOP delegation and report-as-evidence contract

Source: `Tworzenie Raportów i Delegowanie AI.pdf` (14 pages); older SOP sources already owned by P90.

### `SOPDelegationPlan`
```yaml
SOPDelegationPlan:
  objective: null
  trigger: null
  scope_allowlist: []
  exclusions: []
  required_roles: []
  steps_and_decision_branches: []
  required_inputs: []
  expected_outputs: []
  tool_capabilities: []
  side_effect_classes: []
  approval_gates: []
  independent_verification: []
  evidence_artifacts: []
  rollback: null
  release_owner: null
```
The report's `AS-IS → TO-BE → DRAFT → REVIEW → PUBLISH → DRIFT` trajectory becomes a versioned, permission-checked workflow. A generated report must bind each completed action to **actual tool execution receipts**; a text model's claim that it sent an email, deployed an app or scanned a system is not such a receipt.

### Tests and operations
Human-in-the-loop gates for external messages/payments/infrastructure changes; idempotency on retries; privilege isolation by step; path through partial failure; secrets redaction in screenshot/recording evidence; versioned SOP rollback; experiment with novice-reader clarity and stale procedure detection. No external workflow execution or business-process automation is claimed from this source alone.

---

## 2026-10-08 — batch 13: executable AI instructions from task decomposition

Source: `AI_ Instrukcje i Automatyzacja Zadań.pdf` (15 pages), with `AI w Zarządzaniu Projektami- Automatyzacja i Instr....pdf` (11 pages). These extend existing HTA/WBS/SOP material rather than creating a second procedure compiler.

### SOP compilation increment
```text
HUMAN OUTCOME / AUTHORIZED SCOPE
 → HTA (subgoals and elemental operations)
 → WBS (dependencies / resources / delivery milestones)
 → TYPED TASK STATE MACHINE (preconditions, tools, owner, deadline)
 → PERMISSION BROKER + DRY-RUN
 → EXECUTION WITH REAL TOOL RECEIPTS
 → INDEPENDENT POSTCONDITION CHECK
 → ROLLBACK / DRIFT REPORT
```
`ActionSpec` must encode `id, expected_input, precondition, allowed_tool, side_effect_class, output_contract, acceptance_test, retry_budget, approval_gate, rollback`. A workflow cannot make a model responsible for granting itself tools. A prose report is **not** evidence that a task ran; no automated communication/payment/credential operations without separate authorization.

**Acceptance:** dependency cycles, missing approval, failure recovery, bounded retry, idempotency, rollback, source-injection and receipt-reference tests. No real external task execution in this batch.

---

## Knowledge evolution — batch 14 / 2026-10-08: SOP semantic equivalence and execution evidence

Source `Automatyzacja Projektu z AI_ Instrukcja i Wykonani....pdf` (11 pages). Already-owned P90 pipeline compiles source SOP into scope, typed inputs/outputs, preconditions, action capability, exclusions, approval gates, idempotency, retry, rollback and acceptance conditions.

New `ProcedureEquivalenceTest` compares the source's stated authorized task vs translated workflow behavior, including denial of omitted/out-of-scope actions. Text generated by an agent cannot by itself attest that any Make/n8n/CrewAI workflow actually ran; `ResultEvidence` requires a tool receipt + independent postcondition + source hash. Negative fixtures: unclear approval, tool failure, source injection, race/retry and stale provider capability. No external automation executed in this batch.
