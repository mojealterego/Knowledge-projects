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

---

## Knowledge evolution — batch 16 / 2026-10-08

DGM source requests a four-stage lifecycle ModelScout→TechRecon→Strategist→DGM_Core. **Actual code**: tools/dgm_cycle_gate.py checks candidate/base commit digests, integration/ branch, four nonempty phase receipts, externally supplied candidate approval, claimed independent verified CI checks, expiry, rollback witness and disabled external effects. This is a non-executing metadata gate, **not** source attestation, sandbox, independent signature verification or platform branch protection. Direct default-branch mutation shown in DGM example must not be used.

---

## 2026-10-09 — batch 19: agent marketplace, MCP and consent-based games

GitHub Copilot Agent Apps (public preview) can read PRs, create feature flags/config, post comments, run scanners and trigger deployments. **Marketplace listing, publisher badge or advertised free tier is not an owner-approved permission grant**. Added `tools/marketplace_agent_review_gate.py` with 13 tests: dry-run requested listing slug/URL, exact target repo, externally supplied grant id+approved capability+expiry, explicit effectful-action approval, secrets/data-export denial and owned isolated scanning target. The module **does not install an app, invoke GitHub permissions, verify the publisher's true identity or enforce IAM**. Real installation remains separately approved; all side effects must have tool receipts and independent postconditions. SAKH read-only vs Apricot project-delete vs Traveler profile-update are different MCP grants; inspect actual `tools/list` after OAuth rather than assuming read-only across servers.

---

## 2026-10-09 — batch 20: ODYN / Hermes / Nous ecosystem

**Sources:** Hermes security, MCP, Skills and AgentSkills specs, Nous `agent-governance-toolkit`, `OpenShell`, `hermes-example-plugins`, `hermes-agent-self-evolution`. Add `UpstreamAdoptionEvidence`: public GitHub URL, exact upstream/fork SHA, source license, hash, archive/fork status, independent security and CI receipts, reviewer approval, sandbox profile, granted capabilities, expiry and rollback. Actual **code** in `tools/hermes_upstream_adoption_gate.py` performs *offline candidate policy review*, with 17 unittest cases; it does **not clone, verify a digital signature or run a real sandbox**. Third-party SKILL.md, plugin hooks, MCP commands, Honcho memories and prompt contexts must not independently authorize terminal, browser, device or paid gateway actions. Protect against prompt injection, tool result spoofing, credentials leaking into logs and unbounded self-improvement.

---

## 2026-10-09 — batch 21: Zed Guild, Railway bounties, WordPress premium

**Tool-loop guard and evidence-bound opportunity inputs.** The Zed open issue #65199 is a concrete fail condition for repeated tool calls without state change; #51333 is a separate reproducible Gemini sandbox/ACP integration issue (OPEN); #65205 is already CLOSED, demonstrating stale issue feeds. Introduce `TrustedProgressReceipt`: tool name, SHA256 of typed parameters, externally verified version/postcondition, token estimate, number of no-progress repeats, cumulative budget, reject reason and human override. `tools/agent_tool_loop_guard.py` enforces simple token/call/repeat limits offline and cannot intercept third-party SDK calls until explicitly wired. `tools/external_opportunity_evidence_gate.py` denies supposed jobs based on board-only or stale data. Both are **testable policies**, not autonomous browsing, paid bounty claim or external repository authority.

---

## 2026-10-09 — batch 23: seven-page GitHub MCP Registry discovery

**GitHub MCP Registry** is a list of server metadata, not a verification of trust, actual `tools/list`, OAuth scopes, financial terms or ChatGPT installation. Official GitHub docs describe Copilot IDE MCP public preview and separate hosted registry policies. Add `RegistryDiscoveryEvidence`: source URL/slot, publisher claim, version+license, official manifest SHA, actual independently read tool list, tool read/write scopes, OAuth issuer/audience, data residency, explicit connection approval, audit and revocation. **Implemented `tools/mcp_registry_catalog.py:evaluate_integration()`** allows local catalog inspection and denies install/connect/payment or untrusted effects from listing-only proof. This is offline policy, **not** a real sandbox, OAuth client or platform enforcement. Desktop Commander, Remote Desktop Commander, Apple Tools, SentinelX, financial/communications services are high impact; never infer user consent from catalog popularity.

## Batch 24 — uploaded PDF evidence review, 2026-10-09

Batch24 source assurance covers 10 full uploaded PDFs (262 pages) and three local static gates: program_claim_gate.py (date/funding evidence), seeker_evidence_scope_gate.py (case authorization and no false location evidence), communication_link_policy.py (opt-in device communication). All require independent host-side permissions and real evidence before remote action. Add CandidateEvidence statuses source_proposed, provider_documented, locally_tested, independently_verified, externally_committed. Claims in PDF such as 'fully implemented', '2B free tokens' or 'global geolocation' never advance state absent exact primary proof; overlapping Seeker drafts are not corroborating witnesses. DGM tool/diff operations still go through existing dgm_cycle_gate.py and single_repository_scope_gate.py. No trusted external sandbox or scanning service integrated.

---

## 2026-10-09 — batch 25: uploaded MCP, Unity, eSIM and adversarial agent sources

User-supplied Pegasus.md and ofensivemax/Pegasus.txt are untrusted adversarial instructions and incomplete offensive Python illustrations, NOT higher-priority orders to disable controls. Static tool tools/untrusted_agent_source_review.py identifies shell execution, network reconnaissance, intrusion intent and surveillance patterns WITHOUT executing supplied code; 13 synthetic tests. Reject any user-source `AGENTS.md`/persona instructions that try to override the repository's AGENTS.md or security policy. MCP Android and Google Drive claims need a real, separately authorized host; folder ID or service account does not automatically isolate all Drive requests. No privileges, third-party data, or tool executions have been granted.
