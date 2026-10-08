# P121 — InfraSentinel-Agentic

**Status:** CONCEPT / ARCHITECTURE-READY / PoC CLAIMS REQUIRE VERIFICATION  
**Domain:** autonomous SRE, cloud infrastructure, DevOps, incident remediation  
**Primary stack described by source:** Nebius Token Factory + NVIDIA Nemotron + Tavily + NVIDIA NeMo Guardrails

## Product definition

InfraSentinel-Agentic is a governed autonomous SRE system for diagnosing infrastructure incidents, researching current technical documentation, constructing remediation plans and executing bounded changes inside an isolated environment.

Core loop:

`ALERT → AUTHENTICATE → OBSERVE → RESEARCH → PLAN → AUTHORIZE → VALIDATE TOOL CALL → SANDBOX ACT → READBACK → VERIFY → AUDIT`

## Architecture

### Cognitive layer

A model hosted through Nebius is proposed as the planning/orchestration engine. The model should produce structured tool calls rather than unrestricted shell text.

### Epistemic layer

Tavily is used to retrieve current documentation and incident-relevant information. Source filtering and context limits are useful, but retrieval remains evidence acquisition, not truth certification.

### Protective layer

NeMo Guardrails/Colang provides programmable policy checks around inputs, tool selection, execution arguments and outputs. It complements, but does not replace, explicit authorization and infrastructure security controls.

### Execution layer

Infrastructure tools execute only inside a sandbox/test environment until the complete authorization and verification path has been proven.

## Reference architecture

```text
PROMETHEUS / ALERT SOURCE
          ↓
  INCIDENT NORMALIZER
          ↓
 AUTHENTICITY + SCHEMA
          ↓
    POLICY / RISK GATE
          ↓
  NEMOTRON ORCHESTRATOR
       ↙         ↘
 TAVILY          PLAN
 RESEARCH         ↓
       ↘    TOOL VALIDATION
             ↓
       SANDBOX EXECUTOR
             ↓
          READBACK
             ↓
      POSTCONDITION TEST
        ↙           ↘
   VERIFIED       ESCALATE
        ↓
       AUDIT
```

## Example incident

A Kubernetes `CrashLoopBackOff` associated with RBAC configuration can trigger:

1. collect cluster/log evidence;
2. search current Kubernetes documentation;
3. identify candidate causes;
4. construct a minimal remediation plan;
5. validate the proposed tool and arguments;
6. execute only in an isolated environment or after explicit authorization;
7. read back cluster state;
8. verify the expected postcondition;
9. record evidence and decision lineage.

The system must never treat a generated `kubectl` command as self-authorizing.

## Tool contract

The source describes two primary tool categories:

- `search_documentation_tavily(query)`;
- `execute_infrastructure_command(command)`.

Production implementation should version schemas, validate arguments independently of the LLM, enforce capability/risk policies, and reject unknown tools or malformed arguments.

## Guardrail model

The source proposes:

`INPUT FILTER → TOOL SELECTION CHECK → EXECUTION-TIME CHECK → POST-ACTION AUDIT`

P121 extends this with an explicit authorization boundary:

`POLICY → AUTHORIZE → TOOL VALIDATE → ACT`

and with independent verification:

`ACT → READBACK → POSTCONDITION TEST`.

## Research provenance

Every remediation recommendation should retain:

`INCIDENT → QUERY → SOURCE → EXTRACTED CLAIM → PLAN → ACTION → RESULT → VERIFICATION`

Official documentation should be preferred for API/configuration claims, but even official documentation can be version-specific. The agent must bind retrieved guidance to the detected software/environment version.

## Risk tiers

Suggested action classes:

- **R0 — Read-only:** logs, status, metadata.
- **R1 — Reversible sandbox mutation:** isolated test changes.
- **R2 — Reversible production change:** requires explicit policy authorization and rollback plan.
- **R3 — High-impact mutation:** production networking, identity, databases or cluster-wide resources; human approval and stronger verification.
- **R4 — Destructive:** deletion, irreversible migration or broad credential revocation; default deny.

## Reliability

Required controls:

- idempotency keys for mutating workflows;
- durable execution state;
- retry with bounded backoff;
- cancellation and timeout;
- rollback/checkpoint support;
- immutable audit records;
- environment isolation;
- least privilege;
- independent postcondition checks.

## Fine-tuning / LoRA

Organization-specific incident data may justify LoRA adaptation. Training configurations from the source are treated as hypotheses requiring empirical evaluation. The evaluation set must remain held out and include safety/regression cases; successful training loss is insufficient evidence of operational improvement.

## Business metrics

P121 should measure:

- mean time to acknowledge (MTTA);
- mean time to remediate (MTTR);
- remediation success rate;
- rollback rate;
- false-remediation rate;
- escalation rate;
- unauthorized-action block rate;
- tool-call/schema error rate;
- cost per resolved incident;
- availability impact avoided.

`METRIC ≠ CAUSALITY` and `AUTOMATION ≠ BUSINESS ROI`.

## Portfolio integration

P121 consumes primitives from P100 rather than recreating them:

- capability/provider/tool registry;
- authorization/policy;
- MCP boundaries;
- DevSecOps and CodeQL;
- artifact provenance;
- sandbox/build verification;
- agent workflow/DAG infrastructure.

P114 contributes memory/verification patterns, P119 contributes device/runtime readback patterns where relevant, and the voice-agent knowledge layer contributes agent/tool orchestration patterns.

## Security invariants

- `MODEL ≠ AUTHORITY`
- `TAVILY RESULT ≠ VERIFIED TRUTH`
- `GUARDRAIL ≠ COMPLETE SECURITY`
- `TOOL CALL ≠ AUTHORIZATION`
- `SANDBOX SUCCESS ≠ PRODUCTION SAFETY`
- `COMMAND SUCCESS ≠ REMEDIATION SUCCESS`
- `REMEDIATION SUCCESS ≠ POSTCONDITION VERIFICATION`
- `TEMPERATURE ≠ DETERMINISM`
- `FINE-TUNE ≠ GUARANTEED IMPROVEMENT`
- `AUTONOMY ≠ UNBOUNDED ACCESS`

## Implementation gate

The first production-grade milestone is a single read-only incident diagnosis followed by a sandbox remediation with complete evidence lineage and independently verified postconditions. Production mutations should remain disabled until this closed loop passes adversarial tests.

---

## 2026-10-08 — batch 18: public web source evolution

Google Cloud Compute Engine, Cloud Storage and Enterprise Agent Platform source pages become infra qualification inputs: GcpChangeApproval {verified_account_IAM_scope,project_and_region,VM+GPU_quota,storage_lifecycle,egress_cost,monthly_spend_cap,explicit_side_effect_permission,rollback_and_audit_receipts}. `tools/cloud_operation_budget_gate.py` checks metadata against independent approvals without calling Cloud APIs; it is not a real IAM permission broker, billing lock or deployer. No compute VM, bucket, model endpoint or payment created.
