# 26 — OmniCore Trustworthy Kernel & Agentic Safety Lab MAX

## Status
Architecture-ready defensive research project.

## Objective
Turn the current OmniCore concept into a measurable engineering program for a trustworthy AI-native runtime: deterministic kernel boundaries, typed capabilities, isolated AI execution, verified driver synthesis, multimodal/prompt-injection defenses, and repeatable adversarial evaluation.

This project is deliberately defensive. Supplied security reports on jailbreaks, indirect injection, safety settings, fine-tuning and model control are converted into threat models, policy-surface mapping, detection, containment and regression evaluation rather than bypass tooling.

## Source synthesis

The supplied security material describes a layered safety surface involving model training/alignment, system instructions and safety classifiers. fileciteturn51file5L220-L253 The correct architectural conclusion for OmniCore is that security must be enforced outside the natural-language interface as well as inside the model lifecycle.

The feasibility material defines an integrated stack of seL4, WebAssembly/WASI, LLM/RAG and 3DGS, with the principal integration risk being GPU access from the microkernel. Its proposed mature path is a seL4 VMM plus a Linux driver domain with IOMMU/SMMU passthrough; shared memory is recommended for large transfers. fileciteturn234file0L45-L79

## Core architecture

```text
                         OPERATOR
                            │
                     INTENT / REQUEST
                            │
                +-----------▼------------+
                | TRUST CONTROL PLANE    |
                | policy / authz         |
                | provenance             |
                | risk classification    |
                | eval routing           |
                | policy-surface map     |
                +-----+-------------+----+
                      │             │
                +-----▼----+   +----▼------+
                | AI / LLM |   | Tools/MCP |
                | isolated |   | capability|
                | domain   |   | gateway   |
                +-----+----+   +----+------+
                      │             │
                      +------+------+
                             │
                       VERIFIED PLAN
                             │
                  +----------▼----------+
                  | DETERMINISTIC       |
                  | EXECUTOR / BROKER   |
                  +----------+----------+
                             │
                  EFFECT / STATE CHANGE
                             │
                     READBACK / VERIFY
                             │
                     TELEMETRY / EVAL
                             │
                       EVIDENCE LEDGER
```

## 1. Policy Surface Mapper

The supplied "unlocking AI" material is treated as a map of possible control layers, not a recipe for disabling them. It distinguishes model-level alignment, system instructions and safety classifiers. fileciteturn51file5L235-L252

OmniCore models the surface explicitly:

```yaml
PolicySurface:
  model_behavior:
  system_instruction:
  developer_policy:
  application_policy:
  capability_policy:
  data_policy:
  network_policy:
  user_consent:
  runtime_sandbox:
  release_gate:
```

For every consequential operation the system records **which layer denied or allowed it**. This makes policy failure diagnosable instead of collapsing everything into a generic refusal.

## 2. Alignment / policy regression harness

Each model/runtime combination is evaluated against a frozen corpus plus newly discovered regressions:

```text
MODEL VERSION
   ↓
SAFETY / CAPABILITY TEST SUITE
   ↓
POLICY DECISION TRACE
   ↓
COMPARE WITH BASELINE
   ↓
REGRESSION / IMPROVEMENT / UNKNOWN
```

Key rule:

`MODEL BEHAVIOR != APPLICATION AUTHORIZATION`

Even a permissive model must not gain a capability merely because the model generated a valid-looking tool call.

## 3. Representation-aware security

The supplied security literature points to multilingual, encoding, multimodal, structural and multi-turn attack surfaces. fileciteturn51file7L362-L396

Canonicalization pipeline:

```text
RAW INPUT
 ↓
UNICODE / ENCODING NORMALIZATION
 ↓
LANGUAGE IDENTIFICATION
 ↓
STRUCTURE PARSE
 ↓
OCR / AUDIO / VIDEO EXTRACTION
 ↓
PROVENANCE
 ↓
UNTRUSTED DATA OBJECT
 ↓
POLICY CLASSIFICATION
```

The system must preserve the original evidence while creating normalized views for analysis.

## 4. Prompt injection / indirect-instruction boundary

External content is data by default:

```text
USER / WEB / RAG / FILE / TOOL RESULT
                  ↓
             UNTRUSTED DATA
                  ↓
        CONTEXT COMPILER / FILTER
                  ↓
         MODEL + EXPLICIT POLICY
                  ↓
          TYPED CAPABILITY PLAN
                  ↓
            CAPABILITY BROKER
```

A retrieved instruction cannot alter authorization merely because it appears in a highly relevant document.

## 5. Capability Firewall

Every privileged operation remains typed:

```text
CapabilityId
SubjectId
Resource
Operation
Scope
Expiry
RateLimit
Provenance
ApprovalState
```

No natural-language output directly authorizes an actuator.

## 6. Memory poisoning resistance

The state boundary remains:

```text
EPHEMERAL CONTEXT
LONG-TERM MEMORY
POLICY STATE
CREDENTIAL STATE
AUDIT LOG
```

Only typed memory-write operations can mutate durable memory. Security-relevant claims require provenance and, where consequential, independent corroboration.

## 7. Verified generation boundary

AI-generated drivers, services, policies and code move through:

```text
GENERATE
 → PARSE
 → TYPE / EFFECT CHECK
 → STATIC ANALYSIS
 → INVARIANT CHECK
 → SYMBOLIC / SMT CHECK
 → BUILD
 → FUZZ / SANDBOX TEST
 → HIL WHERE APPLICABLE
 → SIGN
 → STAGED DEPLOYMENT
 → HEALTH CHECK
 → PROMOTE / ROLLBACK
```

No generated artifact self-promotes.

## 8. GPU / 3DGS trust partition

The GPU/renderer domain remains lower trust than the kernel/security control domain. Untrusted media, shaders and rendering workloads are isolated; cross-domain buffers use validated contracts.

## 9. OMEGA-derived all-source fusion — defensive form

The OMEGA family combines cyber recon, public business records, geospatial information, image/EXIF analysis and an LLM synthesis step. fileciteturn61file1L53-L104

OmniCore converts this into a defensive **exposure-and-evidence fusion** pipeline:

```text
AUTHORIZED ASSETS / PUBLIC SOURCES
          ↓
COLLECTOR CAPABILITIES
          ↓
NORMALIZE + PROVENANCE
          ↓
ENTITY / ASSET RESOLUTION
          ↓
EVIDENCE GRAPH
          ↓
EXPOSURE HYPOTHESES
          ↓
RISK / COUNTEREVIDENCE
          ↓
REMEDIATION PRIORITIES
```

The system may correlate public KRS/entity information, infrastructure observations and image metadata only within explicit legal and authorization boundaries. Offensive exploitation and credential attacks are not product objectives.

## 10. KRS / public-registry adapter

The supplied code demonstrates direct querying of the Polish KRS API and extraction of entity name, address, registration data and representation. fileciteturn175file8L5-L41

The production adapter must add:

- source timestamp;
- raw response hash;
- schema version;
- entity-resolution confidence;
- field-level provenance;
- rate limit and retry policy;
- jurisdiction metadata;
- retention policy.

A public registry result is evidence, not an automatic inference about intent, ownership risk or threat level.

## 11. Defensive OMEGA collector contract

```yaml
Collector:
  id:
  version:
  source:
  mode: passive|authorized_active
  target_scope:
  input_schema:
  output_schema:
  required_capabilities:
  provenance_fields:
  rate_limit:
  timeout:
  evidence_quality:
  failure_modes:
```

Any active network operation requires explicit scope, authorization, allowlist, rate limit and audit logging.

## 12. Evidence fusion model

```yaml
EvidenceNode:
  id:
  domain: cyber|business|geo|image|web|document|runtime
  observation:
  source:
  captured_at:
  content_hash:
  transformation_chain:
  confidence:
  freshness:
  corroboration:

EvidenceEdge:
  from:
  to:
  relation:
  confidence:
  provenance:
```

The graph supports multi-source correlation without collapsing inference into fact.

## 13. Human-agent decision gate

```text
EVIDENCE
 ↓
MODEL SYNTHESIS
 ↓
RISK CLASSIFICATION
 ↓
REMEDIATION OPTIONS
 ↓
HUMAN REVIEW FOR HIGH CONSEQUENCE
 ↓
AUTHORIZED ACTION
 ↓
AUTHORITATIVE READBACK
```

## 14. Evaluation

### Model / policy
- jailbreak regression rate;
- policy-drift rate;
- unauthorized-capability rate;
- false refusal rate;
- multilingual consistency.

### Injection / provenance
- indirect-instruction acceptance rate;
- provenance coverage;
- memory-poison acceptance rate;
- stale-state acceptance rate.

### Infrastructure
- cross-domain fault propagation;
- rollback success;
- credential exposure;
- sandbox escape rate.

### Intelligence fusion
- entity-resolution accuracy;
- false-correlation rate;
- evidence coverage;
- counterevidence discovery rate;
- time-to-remediation.

## 15. Continuous Attack-Trajectory Defense

Extend the project from single-input security to temporal graph analysis:

```text
INPUT → MEMORY → TOOL → ACTION → STATE CHANGE
   ↖          ↘       ↗        ↘
      TRAJECTORY / DEPENDENCY GRAPH
```

The detector tracks semantic drift, repeated policy probing, capability escalation, memory poisoning, multimodal conflicts and action/claim mismatch across the full interaction trajectory. This is the defensive counterpart of Chain-of-Attack research.

## 16. Endpoint Trust-Boundary Evaluation

Mobile security controls become laboratory scenarios for boundary testing:

`CODE SIGNING → SANDBOX → PERMISSION → IPC → NETWORK → PERSISTENCE → REVOCATION`.

The goal is measurement of containment and recovery, not reproduction of surveillance malware.

## 17. Definition of Done additions

10. Attack trajectories are represented as versioned temporal graphs.
11. Memory, tool and state transitions are inspected jointly.
12. Multilingual and encoded transformations are regression-tested.
13. Endpoint-style trust boundaries have simulated containment tests.
14. Security recovery works even when the model is uncooperative.

---

## 2026-10-08 — batch 13: agent-produced kernel build assurance

Source: `--AI w Tworzeniu Systemów Operacyjnych-- (1).pdf` (9 pages). It proposes a three-role Cursor/Windsurf/Devin agent factory, Rust `no_std`, RAG and QEMU. All are architecture/source claims; the PDF is not a compiled boot image.

### Engineering changes
- `KernelAgentPolicy`: role-to-permission mapping for code editing, user-space test execution, QEMU sandbox and reviewer sign-off. Model instructions such as `.cursorrules` constrain prompts but **do not enforce kernel security**.
- `SourceGrounding`: versioned, authoritative CPU/ABI/HAL references, RAG citation lineage and untrusted-input isolation for fetched documentation.
- `BootEvidence`: source SHA, toolchain + target, QEMU command and logs, exit/interrupt tests, deterministic reproduction and signed approval. Never infer success from an agent terminal narrative.
- `Ring0Boundary`: the AI supervisor may propose plans in a separable environment but cannot automatically apply probabilistic changes in the privileged kernel or disable existing controls.
- `AgentDisagreement`: divergent reviews become counterevidence, not a majority permission vote.
- `PatchPromotion`: static analysis, fuzz + memory tests, performance baseline, peer review, staged release and rollback.

**Acceptance:** QEMU boot test and reproducible build, privilege-denial tests, unsafe block justification, source-trace and independent postcondition readback. This batch did not run a kernel, drivers or cloud agents.
