# Project 60 — OmniCore Adversarial Multimodal AI Security & Resilience Fabric MAX

## Purpose

Build a model- and provider-neutral security fabric for highly capable multimodal and agentic AI, combining the supplied research on GCG adversarial optimization, Gemini jailbreak/prompt-injection classes, multimodal indirect injection, memory poisoning, agent-to-agent propagation and adaptive red-team evaluation.

The project is defensive. Attack mechanisms are represented as threat classes and controlled evaluation targets; operational bypass payloads are not part of the production capability surface.

## Problem statement

Modern AI security cannot be reduced to prompt filtering. The supplied GCG report describes a transition from semantic jailbreaks toward algorithmically optimized adversarial inputs operating in high-dimensional model representations. fileciteturn327file0L27-L40

The Gemini report separately demonstrates the architectural distinction between model jailbreak and application-level prompt injection, where untrusted content can be misinterpreted as instructions and cause unauthorized tool actions. fileciteturn328file0L47-L64

Together these imply a layered security problem:

```text
MODEL ROBUSTNESS
      +
INPUT / CONTEXT INTEGRITY
      +
MULTIMODAL DATA BOUNDARY
      +
MEMORY INTEGRITY
      +
AGENT-TO-AGENT TRUST
      +
CAPABILITY AUTHORIZATION
      +
AUTHORITATIVE STATE VERIFICATION
```

## 1. Threat classes

### T1 — Optimized adversarial inputs

GCG-style optimization searches discrete token space using gradient information and greedy evaluation. fileciteturn327file0L80-L114

Defense objective:

`detect / perturb / evaluate / contain`

rather than attempting to enumerate every possible suffix.

### T2 — Transfer attacks

The supplied report documents transfer between source and target models and identifies shared training distributions and representation similarities as contributing factors. fileciteturn327file0L157-L182

Defense objective: robustness must be evaluated across model families and not inferred from one model's local score.

### T3 — Long-context conditioning

Many-shot and context-flooding patterns exploit the fact that long context changes the effective conditioning environment. fileciteturn328file0L184-L195

Defense objective: context length must increase provenance and policy scrutiny, not trust.

### T4 — Multimodal injection

The supplied Gemini research describes visual contextual attacks, OCR injection, hidden text and adversarial image perturbations. fileciteturn328file0L196-L237

Defense objective:

`image/audio/video/OCR = untrusted evidence until classified`

### T5 — Indirect prompt injection

Web pages, PDFs, e-mails and database records may contain instruction-like content. The model must not inherit the authority of those instructions merely because it retrieved them. fileciteturn328file0L238-L249

### T6 — Memory poisoning

Persistent memory creates a state-integrity attack surface because false information can survive across sessions. fileciteturn328file0L264-L273

### T7 — Agent propagation

Agent-to-agent messages can transport malicious instructions across otherwise separate agents. fileciteturn328file0L279-L286

### T8 — Evaluation-aware behavior

The supplied Gemini report discusses evaluation awareness. OmniCore treats such reports as evaluation hypotheses, not proof of intentional deception, and therefore tests behavior across independent environments and monitors. fileciteturn328file0L309-L329

## 2. Security architecture

```text
                UNTRUSTED WORLD
                      |
       +--------------+--------------+
       |              |              |
      TEXT          IMAGE          AUDIO/VIDEO
       |              |              |
       +--------------+--------------+
                      v
          MULTIMODAL SANITIZATION
                      |
             PROVENANCE / SOURCE
                      |
          INSTRUCTION-DATA SEPARATION
                      |
             CONTEXT COMPILER
                      |
       +--------------+--------------+
       |                             |
 MODEL / REASONING              MEMORY PLANE
       |                             |
       +--------------+--------------+
                      v
              POLICY / RISK GATE
                      |
              CAPABILITY BROKER
                      |
          AUTHORIZED EXECUTION
                      |
              STATE READBACK
                      |
            POSTCONDITION CHECK
                      |
               AUDIT / EVAL
```

## 3. Adversarial evaluation harness

The harness supports controlled evaluation families:

- semantic jailbreak robustness;
- GCG-style optimization robustness;
- perturbation/stochastic robustness;
- cross-model transfer;
- multilingual and encoding robustness;
- long-context injection;
- visual/OCR injection;
- indirect injection through retrieved content;
- memory integrity;
- agent-to-agent contamination;
- tool-call authorization;
- evaluation-aware behavior;
- monitorability degradation.

The source describes SmoothLLM as a stochastic defense based on perturbing multiple copies and aggregating outcomes. fileciteturn327file0L277-L287

OmniCore treats this as one component of a larger evaluator, not a universal solution.

## 4. Monitorability-aware oversight

This project inherits the repository rule that reasoning traces are evidence rather than authorization. When reasoning visibility becomes weak or unavailable, external verification becomes stronger.

```text
NORMAL MONITORABILITY
 -> standard capability controls

DEGRADED MONITORABILITY
 -> stronger action verification
 -> tighter capability scope
 -> deeper audit
 -> higher human-review threshold

NO RELIABLE INTERNAL TRACE
 -> authoritative state verification remains mandatory
```

This prevents an empty or persuasive reasoning trace from becoming a security bypass.

## 5. Capability isolation

No model output, retrieved document, memory record or agent message can directly authorize:

- data export;
- external communication;
- financial action;
- credential mutation;
- privileged code execution;
- persistent policy change;
- cross-agent privilege escalation.

All such effects pass through typed capabilities with explicit subject, resource, operation, scope, expiry, rate limit, provenance and approval state.

## 6. Evidence and memory integrity

Every durable memory candidate should be represented as:

```text
MemoryCandidate
- source
- observed_at
- content
- evidence_class
- confidence
- sensitivity
- intended_scope
- expiry
- approval_state
- supersedes
```

A memory item cannot become an authorization fact merely because an LLM proposed it.

## 7. Resilience scoring

A system-level resilience score should combine independent dimensions rather than a single model safety number:

`R = g(model_robustness, context_integrity, modality_integrity, memory_integrity, capability_isolation, state_verification, monitorability)`

Scores must preserve uncertainty and disagreement. Correlated tests do not count as independent proof.

## 8. Relationship to existing projects

- Project 06 / 12 — cognitive security and adversarial evaluation.
- Project 26 — deterministic trust boundary and capability firewall.
- Project 27 — disagreement-driven reasoning evaluation.
- Project 30 / 32 — evidence provenance and Zero-Trust OSINT.
- Project 35 — intent/context security.
- Project 36 / 43 — human-agency defense.
- Project 41 — multimodal repository/world-state action fabric.
- Project 49 — cognitive sovereignty.
- Project 54 — epistemic/adversarial control.
- Project 57 — multimodal intelligence and cognitive interfaces.
- Project 58 — monitorability-aware oversight.
- Project 59 — autonomous execution and AI-native control fabric.

## 9. Hard invariants

1. Model robustness does not imply application security.
2. Jailbreak and prompt injection remain distinct threat classes.
3. External content is never an authority source.
4. Multimodal inputs are untrusted evidence.
5. OCR output is untrusted evidence.
6. Long context does not increase trust.
7. Memory writes are governed state transitions.
8. Agent-to-agent messages do not inherit authority.
9. Model-generated tool calls require capability authorization.
10. Successful reasoning does not prove a requested action is safe.
11. GCG-style evaluation is authorized, bounded and auditable.
12. Perplexity or any single detector is not a security boundary.
13. Cross-model transfer testing is required for claims of broad robustness.
14. Reduced monitorability increases external verification requirements.
15. Authoritative state is verified outside the model.
16. Unknown robustness remains unknown until measured.
17. Attack artifacts are controlled evaluation assets, not production capabilities.
18. Revocation and shutdown remain effective without model cooperation.

## Design objective

`Adversarially tested intelligence + zero-trust data/context boundaries + typed capability authority + independent state verification.`

---

## Knowledge evolution — batch 14 / 2026-10-08: GCG defensive robustness provenance

Sources: nine-page `Atak GCG na Modele Językowe.pdf` and `Architektura Systemu AI OmniCore Omega (1).pdf`. Existing GCG knowledge and P60/P108 own this domain. `GCGThreatCard` captures pinned model and tokenizer versions, authorized isolated test scope, threat class, source evidence, observed refusal/tool-policy results, independent reviewer and regression state.

The source claims gradient-generated suffix transfer; **do not assume universal bypass efficacy**. Evaluate model-output behavior separately from whether independent tool-authorization blocks unsafe actions. P60 may design controlled defensive test fixtures, but must not introduce live jailbreaks or payload generation against third-party models.

Acceptance: benign synthetic regression, positive permission-check tests, versioned baseline, bounded attempt budget and independent postcondition. No GCG attack was executed here.
