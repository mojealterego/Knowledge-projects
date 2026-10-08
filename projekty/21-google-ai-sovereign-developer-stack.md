# 21 — Google AI Sovereign Developer Stack

## Status
Architecture-ready reference project.

## Objective
Unify the strongest ideas from the supplied Google AI materials into a provider-aware development stack: local/edge inference where practical, cloud reasoning when justified, agentic orchestration, multimodal computation, structured outputs, evaluation, and human approval.

## Source-derived foundation
The supplied Google AI report describes a transition from generative systems to agentic systems and combines Gemini models, Project IDX, JAX/TensorFlow, Android AI Core, Chrome Built-in AI and Google Antigravity. fileciteturn219file3L14-L21

Its strategy explicitly recommends local prototyping, agentic construction, verification, and hybrid deployment. fileciteturn216file4L173-L183

## Reference architecture

```text
                 DEVELOPER / USER
                        │
                 Intent + Artifacts
                        │
                 TASK CLASSIFIER
                        │
           ┌────────────┼────────────┐
           │            │            │
        EDGE/LOCAL    CLOUD      SANDBOX
        fast/private  reasoning  code/data
           │            │            │
           └────────────┼────────────┘
                        │
                  AGENT RUNTIME
             ┌──────────┼───────────┐
             │          │           │
            MCP       A2A*       Tools
             │          │           │
             └──────────┼───────────┘
                        │
                 VERIFICATION
                        │
               EVALS / APPROVAL
                        │
                   DEPLOYMENT
```

`*` A2A is retained here as a source-derived architecture concept; implementation must use the currently supported protocol/version rather than assuming historical API details.

## Model routing

Routing dimensions:

- task class;
- required reasoning depth;
- modality;
- privacy classification;
- latency target;
- cost budget;
- availability;
- verification requirement.

Reasoning depth is dynamic. The Gemini material describes configurable thinking levels and budgets and recommends matching them to task complexity. fileciteturn219file0L41-L70

## Multimodal compute path

When an image, diagram, chart or video contains measurable information:

1. preserve original media;
2. use model perception to identify relevant regions;
3. execute deterministic code where measurable quantities matter;
4. compare computed result to model interpretation;
5. attach provenance.

The supplied material specifically describes programmatic visual analysis using Python/OpenCV/NumPy and numerical extraction from charts. fileciteturn217file2L91-L107

## Agentic IDE pattern

Antigravity-like workflows are represented as staged agent runs:

`inspect → plan → modify → execute tests → inspect outputs → review → approve → finalize`

Generated changes are never promoted merely because the model reports success.

## MCP integration

Use MCP as the capability boundary for private data sources and operational tools. The supplied material identifies examples such as Drive, PostgreSQL and BigQuery and describes the resulting transformation from generic assistant to domain-specific agent. fileciteturn217file5L214-L227

The project inherits the repository's existing MCP doctrine: schemas, authz, approvals, annotations, structured results, observability and server-side validation.

## Local-first / edge-first policy

Prefer local or edge execution for:

- privacy-sensitive preprocessing;
- classification;
- simple extraction;
- offline interactions;
- latency-critical UI paths.

Escalate to cloud models for:

- difficult reasoning;
- high-context synthesis;
- expensive multimodal interpretation;
- tasks whose local model fails evaluation.

No assumption of zero operating cost is made without measuring hardware, storage, bandwidth, observability and support costs.

## Evaluation

Required benchmark matrix:

| Axis | Example metric |
|---|---|
| Quality | task success / expert score |
| Reasoning | verified solution rate |
| Multimodal | measurement agreement |
| Latency | p50 / p95 |
| Cost | cost per successful task |
| Privacy | data-exposure events |
| Reliability | successful recovery rate |
| Safety | policy violations blocked |

## Definition of done

The stack is considered production-ready only when routing, agent execution, MCP access, sandboxing, verification, evaluation, audit and rollback operate as one coherent system.

---

## 2026-10-08 — batch 18: public web source evolution

Official Gemini Enterprise Agent Platform, Gemini CLI, Antigravity, Embedding 2, Gemma 4 and cloud media APIs form an **optional provider integration set**. Typed capabilities: PUBLIC_DOC, AUTHENTICATED_METADATA, BUDGET_APPROVED_INFERENCE, APPROVED_MUTATION and PAYMENT. User-provided console `authuser`, model or project URL parameters are not credentials/IAM grants. Implemented offline-only `tools/multimodal_embedding_intake_gate.py` and `tools/cloud_operation_budget_gate.py`: remote input limits/sensitive-content preauthorization and independent scope/action/spend plan approval. These are **not production IAM/security enforcement**, no Google API connection, model downloads, enabled APIs, VM/bucket or charges. Source synthesis: `docs/knowledge-base/2026-10-08-google-gemma-embedding-agent-platform-android-security.md`.

---

## 2026-10-09 — batch 19: agent marketplace, MCP and consent-based games

The official Gemini Drops page is a versioned/changing feature showcase rather than a stable SDK/API specification; keep a dated provider feature matrix with plan/region entitlement status, cross-verified against official model docs. GitHub Copilot Agent Apps are another vendor ecosystem, not automatically Gemini or ChatGPT plugins. No Google or GitHub Marketplace app installed; P21 keeps provider-specific cost/permissions/release validation separate from model-expressed intent.
