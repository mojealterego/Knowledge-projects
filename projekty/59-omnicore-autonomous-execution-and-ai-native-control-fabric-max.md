# Project 59 — OmniCore Autonomous Execution & AI-Native Control Fabric MAX

## Purpose

Unify the newly supplied research on autonomous project execution, HTA/WBS task decomposition, multi-agent orchestration, AI-native computing, learned scheduling, AI Foundry, perceptual interfaces, organizational competencies and defensive influence analysis into one governed OmniCore control fabric.

This project is an architectural synthesis and research blueprint. Claims in the supplied sources about future operating systems, performance improvements or full autonomy remain hypotheses unless independently measured.

## Source-derived architecture

The supplied autonomous-project material models outcome quality as a function of structured context and fixed model capability:

`Y = f(X, W_frozen)`

The controllable engineering surface is therefore the construction of `X`: domain evidence, instructions, decomposition, tool context and feedback.

The task-execution material adds HTA: recursively decompose a goal until every executable operation has explicit input, process, output and acceptance criteria.

The AI-native computing material adds a vertical stack:

`NPS Kernel -> AI-Generated Hardware Abstraction -> PUI`

The OmniCore synthesis turns these into a single governed pipeline.

## Master execution pipeline

```text
USER / OPERATOR
      |
      v
INTENT + BUSINESS OBJECTIVE
      |
      v
CONTEXT COMPILER
      |
      +--> DOMAIN EVIDENCE / RAG
      +--> PROJECT STATE
      +--> ORGANIZATIONAL CAPABILITIES
      +--> POLICIES / AUTHORIZATION
      +--> ENVIRONMENT TELEMETRY
      |
      v
HTA / WBS DECOMPOSER
      |
      v
TYPED TASK DAG
      |
      v
SPECIALIST AGENT ROUTER
      |
      v
PLAN / HYPOTHESIS / ARTIFACT SPEC
      |
      v
CAPABILITY BROKER
      |
      v
DETERMINISTIC FLOW RUNTIME
      |
      +--> MCP / APIs
      +--> Files / Data
      +--> Code / Build
      +--> Simulation
      +--> External Services
      |
      v
INDEPENDENT VERIFICATION
      |
      +--> POSTCONDITION
      +--> POLICY
      +--> EVIDENCE
      +--> SECURITY
      +--> HUMAN APPROVAL
      |
      v
AUTHORITATIVE STATE COMMIT
      |
      v
OBSERVABILITY / EVAL / AUDIT
      |
      v
MEASURED OUTCOME -> ADAPTIVE REPLANNING
```

## 1. Autonomous project execution

### HTA contract

Each task becomes a first-class object:

```text
TaskSpec
- id
- parent_id
- objective
- inputs
- preconditions
- capabilities
- procedure
- expected_output
- success_criteria
- stopping_rule
- risk_class
- approval_requirement
- rollback
- postcondition
- provenance
```

No task is executable merely because an LLM produced a plausible instruction.

### WBS contract

WBS is treated as an executable planning graph rather than a static project document. The supplied source emphasizes outcome-oriented work packages, mutually exclusive decomposition, complete work coverage and explicit acceptance criteria. The material also describes multi-level decomposition and dependency mapping.

### Agent roles

Recommended specialization:

- **Planner:** creates and validates task graph.
- **Researcher:** collects evidence under source/provenance rules.
- **Builder:** creates artifacts in a sandbox.
- **Critic/Verifier:** independently tests claims and artifacts.
- **Operator:** executes approved consequential actions.
- **Release Agent:** promotes only verified artifacts.

The roles are capability partitions, not merely personas.

## 2. Crew vs Flow

The supplied OmniCore Omega study distinguishes collaborative agent groups from deterministic execution flows.

For this project:

- **Crew-like layer:** explores high-entropy problems and delegates cognitive work.
- **Flow-like layer:** controls state transitions, ordering, retries and side effects.

This distinction is retained because probabilistic planning and deterministic execution have different failure modes.

## 3. AI-native computing

The supplied architecture describes three layers:

### Kernel — Neural Process Scheduling

Telemetry is transformed into a latent workload representation and used to predict future resource demand. The source discusses runtime uncertainty calibration and predictive scheduling.

OmniCore imposes a hard boundary:

`Model recommendation != kernel authority`

Watchdogs, resource ceilings, isolation, starvation prevention and recovery remain deterministic.

### AI Foundry — generated low-level artifacts

The source proposes ingesting hardware specifications, generating driver code, testing it and iteratively repairing failures.

OmniCore routes this through the existing verified-code factory:

`SPEC -> GENERATE -> STATIC CHECK -> EFFECT/CAPABILITY CHECK -> TEST -> SYMBOLIC/FORMAL CHECK -> BUILD -> SANDBOX/HIL -> SIGN -> PROMOTE`

### PUI — Perceptual User Interface

The supplied architecture treats vision, gesture, speech and context as first-class interaction modalities. It also identifies accessibility benefits of multimodal interaction.

OmniCore adds:

`Perceived intent -> confidence/ambiguity analysis -> explicit confirmation when consequential -> authorized action`

Perception is not authorization.

## 4. Organizational capability layer

The supplied Siwak article frames core organizational competencies as intangible strategic resources, including knowledge and intellectual assets, that can be configured to create economic value and may be difficult for competitors to copy.

OmniCore represents organizational capability as a graph:

```text
Capability
  -> knowledge
  -> people/roles
  -> intellectual assets
  -> processes
  -> technology
  -> evidence of competence
  -> strategic applicability
  -> measurable outcome
```

This connects agent routing to actual organizational competence rather than assigning tasks solely by model availability.

## 5. Influence-security integration

The supplied psychology material catalogues manipulation mechanisms such as self-esteem targeting, ingratiation, cognitive dissonance, thinking biases, group pressure, authority cues, psychological resistance, guilt, Machiavellianism and text/group manipulation.

OmniCore treats these as defensive detection classes.

The system may detect:

- hidden objectives;
- coercive framing;
- artificial urgency;
- authority substitution for evidence;
- commitment locking;
- popularity/social-proof pressure;
- omission of alternatives;
- semantic redefinition;
- dark-pattern opt-out friction.

It must not optimize these mechanisms for covert influence.

## 6. Semantic integrity

The supplied linguistic article demonstrates how a technical term can be moved into public discourse and rhetorically redefined. The control fabric therefore distinguishes:

`TERM -> technical definition -> contextual usage -> speaker framing -> evidence -> semantic drift`

This is useful for misinformation resistance, requirements analysis and policy interpretation.

## 7. AETHER / APEX interface pattern

The supplied AETHER and APEX artifacts contain a browser-based NEXUS-OMEGA visual prototype with Three.js, WebGL post-processing, a central wireframe core, particles, a data-stream tunnel, synthesized audio and terminal interaction.

The strongest reusable architecture is:

`Runtime State -> Visual State Projection -> Operator Interaction -> Verified Runtime State`

The visual layer is explicitly non-authoritative.

Target/IP and decryption-like branches in the prototype are retained only as simulation/UI patterns. Production implementations must use the existing Zero-Trust OSINT rules: explicit scope, authorization, passive-first collection, rate limiting, provenance and auditability.

## 8. Monitorability-aware execution

This project inherits the portfolio's monitorability doctrine:

- reasoning traces are evidence, not authorization;
- missing/weak traces do not lower security requirements;
- action and authoritative-state monitors operate independently;
- consequential actions require stronger external verification when observability is degraded;
- human review thresholds rise with consequence and uncertainty.

## 9. Capability graph

```text
ORGANIZATIONAL CAPABILITY
        |
        v
AGENT SPECIALIZATION
        |
        v
TASK CAPABILITY REQUIREMENT
        |
        v
CAPABILITY BROKER
        |
        +--> AUTHORIZATION
        +--> SCOPE
        +--> RESOURCE
        +--> EXPIRY
        +--> RATE LIMIT
        +--> PROVENANCE
        +--> APPROVAL
        |
        v
EXECUTION
        |
        v
POSTCONDITION
```

## 10. Failure model

The architecture explicitly handles:

- ambiguous intent;
- incomplete WBS;
- invalid preconditions;
- stale state;
- hallucinated tool results;
- tool failure;
- agent disagreement;
- poisoned context;
- insufficient monitorability;
- generated-code defects;
- unauthorized side effects;
- misleading visual state;
- semantic drift;
- organizational capability mismatch.

Every failure class maps to a detection, containment or recovery mechanism.

## 11. Evaluation matrix

| Layer | Primary metric | Independent check |
|---|---|---|
| Intent | ambiguity / scope completeness | human or policy review |
| HTA/WBS | coverage / dependency integrity | graph validator |
| Agent plan | task validity | critic / second solver |
| Tool call | schema correctness | capability broker |
| Execution | runtime success | authoritative readback |
| Artifact | functional correctness | tests / static analysis |
| Evidence | provenance / freshness | source validator |
| Recommendation | calibration / uncertainty | evaluation harness |
| Influence | agency risk | defensive classifier + review |
| PUI | intent accuracy / accessibility | multimodal test suite |
| Kernel | latency / safety envelope | deterministic watchdog |
| Release | reproducibility | signed artifact + replay |

## 12. Hard invariants

1. No executable task without explicit preconditions and postconditions.
2. WBS completeness is validated before autonomous execution.
3. Agent specialization never bypasses capability authorization.
4. MCP exposes capabilities; it does not grant authority.
5. Crew-style collaboration cannot directly commit privileged state.
6. Deterministic flows govern consequential side effects.
7. AI-generated low-level code remains untrusted until independently verified.
8. Perceived user intent is not authorization.
9. Visual or audio feedback is never authoritative system state.
10. Organizational competence claims require evidence.
11. Semantic redefinition is treated as an epistemic risk.
12. Manipulation research is used for detection and resilience, not covert optimization.
13. Reduced monitorability increases external verification requirements.
14. Simulation is never silently represented as real-world evidence.
15. Targeted network operations require explicit scope and authorization.
16. Human approval is a resumable state transition.
17. State commits require authoritative readback.
18. Unknown remains distinct from false, true and verified.

## 13. Relationship to the existing portfolio

Primary integration points:

- Project 04 / 09 — AI-native OS and learned-kernel research.
- Project 13 / 28 — verified agentic software factory.
- Project 24 — prompt compiler and DSL.
- Project 25 / 47 — portfolio governance and project identity.
- Project 31 — Agent OS, Skills and MCP.
- Project 32 — Zero-Trust OSINT.
- Project 35 / 36 / 43 — intent security and human-agency defense.
- Project 40 / 41 — repository and multimodal action intelligence.
- Project 49 — cognitive sovereignty and adaptive intelligence.
- Project 54 / 58 — epistemic control and monitorability-aware oversight.

## Research status

The project is an architectural synthesis of the supplied corpus. Specific performance claims, production readiness of AI-native kernel scheduling, and complete replacement of static operating-system code are not accepted as established facts without independent measurement.

## Design objective

`Probabilistic intelligence for planning and adaptation + deterministic authority for execution and safety.`

---

## Knowledge evolution — batch 16 / 2026-10-08

Feasibility report (5) claims recursive Swarm Self-Replication. Bounded execution must enforce max_depth, max_workers, max_jobs, deadline, queue_capacity, total token/cloud cost and backpressure. Model-created workers cannot grant themselves further privileges. The P126 IBM NPC proposal should not hold DB writes/locks during model inference and must have timeout/fallback. No external action or live cloud orchestration was performed.

---

## 2026-10-09 — batch 20: ODYN / Hermes / Nous ecosystem

**Sources:** Hermes CLI/gateway, messaging, cron, skills and paid Tool Gateway docs. Proposed `HermesProcessLifecycle`: OS/platform classifier, foreground vs allowed background mode, process supervisor, deadline, durable state, callback auth, rate+budget cap, restart policy, media retrieval and proof of actual tool execution. Android Termux foreground `hermes gateway run` does **not** imply systemd/persistent service; Android may terminate background processes. The official Termux APT docs currently warn broken; fail closed rather than spawn processes via desktop/glibc install scripts. `Nous Tool Gateway` is paid, do not schedule purchases or browser sessions by model authority. No cron/gateway jobs, Linux sandboxes or account credentials started in this batch.

---

## 2026-10-09 — batch 21: Zed Guild, Railway bounties, WordPress premium

**No-progress tool-call loop regression as reproducible acceptance target.** Zed issue [#65199](https://github.com/zed-industries/zed/issues/65199) reports an agent read-only loop of 171 identical `read_file` calls, 195 calls and ~10.3M tokens with no edit. Treat figures as issue reporter's evidence, not a locally reproduced benchmark. New `tools/agent_tool_loop_guard.py` is a deterministic non-executing policy: it halts when same tool+argument digest repeats >N times without independently verified progress, or total call/token budgets are exceeded. `trusted_progress_version` must be supplied by the host's real postcondition and cannot be accepted from an LLM message. Needed next: integrate with a real agent executor (inside this repo when built), measure legitimate retries/false positives, add telemetry and a trusted explicit reset. It is **not** installed in Zed or ODYN.

---

## 2026-10-09 — batch 23: seven-page GitHub MCP Registry discovery

The GitHub MCP listing includes potentially effectful tool surfaces **Desktop Commander**, **Remote Desktop Commander**, **SentinelX**, **Apple Tools**, **MCP Emails**, **Omnidim**, social posting, SaaS automation and agents provisioning infrastructure. These are public vendor *claims*, not active runtime `tools/list` or granted capabilities. Use deny-by-default `McpToolEffect`: source proof, tool signature, bounded user intent, policy scope, independent authorization, trusted-progress receipt, cost quota, rollback and irreversible-action review. Combine existing `agent_tool_loop_guard.py` no-progress termination with `tools/mcp_registry_catalog.py` metadata eligibility; **neither is a process sandbox or actual enforcement unless integrated with an authorized host**. External GitHub repositories remain read-only in this ingestion.
