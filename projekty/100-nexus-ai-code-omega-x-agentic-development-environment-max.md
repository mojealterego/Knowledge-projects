# Project 100 — NeXus AI Code OMEGA-X Agentic Development Environment MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build the concrete NeXus AI Code developer environment that federates the repository's agentic development, multimodal interaction, model engineering, MCP, DGM-H/RSI and verification capabilities into one inspectable VS Code/Electron product.

## Why this is distinct
P40 owns the provider-neutral agentic development and live-world execution fabric. P61 owns OmniCore convergence. P80 owns the self-evolving computing substrate. P87 owns the Nexus hybrid language/compiler. P98 owns general-intelligence capability research and evaluation. P100 is the concrete product/integration shell that exposes these capabilities to an operator through one developer environment.

## Source-derived product surface
The supplied NeXus sources specify or report:
- NeXus AI Code branding and Polish-first UI;
- luxury obsidian/gold/copper/platinum visual system;
- multi-model chat including cloud providers and local GGUF;
- simultaneous primary + auxiliary GGUF loading with sampling/context/GPU-layer controls;
- OCN single-agent and swarm invocation;
- OCN 9-file agent registry with Meta/Main/Basic tiers;
- voice chat, camera/desktop vision, microphone/STT/TTS and GUI actuation;
- Rust + Zenoh low-latency sidecar architecture;
- MCP server for a workspace/disk;
- Tor browser integration as an optional capability;
- Unreal/Unity/Android automation targets;
- model training with llama.cpp/Unsloth-oriented tooling;
- DGM/RSI and FunSearch-style code evolution;
- quantum/system telemetry dashboard and task queues;
- Code ↔ Diff views and fitness visualization;
- multiple VS Code extensions federated under one product.

Reported implementation evidence in the source includes 13 installed extensions, successful TypeScript compilation/deployment for several components and a launched Electron instance. These reports are treated as source evidence, not independently reproduced verification.

## Canonical architecture

```text
                    OPERATOR
                       ↓
              NE XUS / POLISH UI
                       ↓
             INTENT + CONTEXT LAYER
                       ↓
        ┌──────────────┼───────────────┐
        ↓              ↓               ↓
   MODEL HUB       OCN SWARM       WORLD/VOICE
        ↓              ↓               ↓
   GGUF/CLOUD      AGENT REGISTRY    VISION/AUDIO/HID
        └──────────────┼───────────────┘
                       ↓
                CAPABILITY BROKER
                       ↓
              MCP / FILE / CLI / GUI
                       ↓
             SANDBOXED EXECUTION
                       ↓
          AUTHORITATIVE READBACK
                       ↓
        VERIFY / AUDIT / ARTIFACT LEDGER
                       ↓
             DGM-H CANDIDATE LOOP
                       ↓
        TEST → VERIFY → STAGE → ROLLBACK
```

## 1. Model Hub

Manage heterogeneous model providers and local GGUF files through a common adapter contract.

### Dual-model mode
- **Główny** — primary reasoning/generation model.
- **Pomocniczy** — secondary model for critique, routing, vision, coding or specialized assistance.

Configuration includes temperature, top-p, top-k, context length and GPU-layer allocation where supported. Actual runtime configuration must be read back from the active backend rather than inferred from UI labels.

## 2. OCN Agent Registry

Preserve the source's 9-file standard:

```text
definition.json
SKILLS.md
SUPER_SKILLS.md
ULTRA_SKILLS.md
SUPER_HOOKS.md
System_prompt.md
TOOLS.json
SUPER_TOOLS.json
ULTRA_TOOLS.json
```

Agent tiers:
- Basic: 11 / 22 / 33 skills and tools;
- Main: 33 / 66 / 99;
- Meta: 111 / 222 / 333.

These numbers are registry targets, not automatic proof of actual capability.

Support:
- direct agent invocation;
- swarm invocation;
- role-based routing;
- capability discovery;
- versioned agent definitions;
- bounded tool exposure.

## 3. Multimodal / embodied layer

The OMEGA-X source defines four functional interfaces:

`VISION → AUDIO IN → AUDIO OUT → ACTUATION`

The implementation boundary is:

`OBSERVE → PLAN → AUTHORIZE → ACT → READBACK → VERIFY`.

Vision and audio observations are provenance-bearing inputs. GUI actions are not considered successful solely because an input event was emitted.

## 4. MCP workspace fabric

Expose workspace capabilities through typed contracts for:
- read;
- write;
- list/search;
- move/copy/delete;
- execution;
- git operations;
- Python/runtime inspection;
- environment information;
- bounded HTTP access where explicitly authorized.

Every capability declares side effects, scope, approval requirement, quota and verification method.

## 5. DGM-H / RSI integration

DGM-H is integrated as a **candidate evolution subsystem**, not as an unrestricted privileged self-modifier.

```text
OBSERVE PERFORMANCE
 ↓
GENERATE CANDIDATES
 ↓
PARALLEL EVALUATION
 ↓
FITNESS / TESTS
 ↓
SECURITY + FORMAL VERIFICATION
 ↓
SANDBOX
 ↓
STAGED PROMOTION
 ↓
POST-RELEASE READBACK
 ↓
ROLLBACK IF REGRESSION
```

FunSearch-style population, tournament selection, crossover and mutation are supported as research/engineering patterns. Candidate code remains untrusted until verification.

## 6. God-Level/OCAA research adapters

P100 may expose experimental adapters for:
- Code World Models;
- MARS object/meta reasoning;
- hierarchical memory concepts;
- neuro-symbolic verification;
- AlphaEvolve-style search;
- bitemporal project memory;
- predictive resource/behavior estimation.

These adapters are explicitly marked experimental until reproducible validation exists.

## 7. UI / Design System

The source-defined visual contract uses obsidian, glass-like panels, platinum/rhodium borders, gold/copper accents, restrained typography and inline SVG icons. Polish labels are first-class product strings. The status indicator communicates model/runtime activity without pretending that UI state is authoritative.

Representative source tokens include `#0A0A0C`, `#D4AF37`, `#B87333`, platinum/rhodium border values and `JetBrains Mono` / `Inter`.

## 8. Extension federation

Target extension families from the supplied implementation record:

```text
nexus-agent-ocn
nexus-ai-hub
nexus-dgm
nexus-code-evolution
nexus-mcp
nexus-sensory
nexus-tor
nexus-trainer
nexus-voice-commander
nexus-quantum-dashboard
nexus-world-panels
nexus-luxury-theme
nexus-pl-language
```

The architecture treats extensions as modular capabilities under a single product control plane, not as independent competing agent authorities.

## 9. Security model

Trust hierarchy:

```text
MODEL OUTPUT                 = UNTRUSTED PROPOSAL
RETRIEVED DATA               = UNTRUSTED UNTIL VALIDATED
UI STATE                     = PROJECTION
TOOL RESULT                  = OBSERVATION
CAPABILITY BROKER            = AUTHORIZATION BOUNDARY
SANDBOX                      = EXECUTION ISOLATION
VERIFIER                     = PROMOTION BOUNDARY
AUTHORITATIVE READBACK      = FACTUAL STATE
AUDIT LEDGER                 = EVIDENCE
```

Required controls:
- least privilege;
- capability allowlists;
- short-lived credentials where applicable;
- network egress restrictions;
- sandboxing;
- postcondition verification;
- turn supersession;
- cost/time/tool budgets;
- immutable audit correlation;
- staged release and rollback.

## 10. Integration map

- **P24** — intent/prompt compilation.
- **P27** — compound reasoning and disagreement analysis.
- **P28** — verified code-generation boundary.
- **P37** — sovereign local/edge/cloud runtime.
- **P40** — agentic development and live-state execution fabric.
- **P54** — cognitive/adversarial security.
- **P57** — multimodal intelligence and interfaces.
- **P59** — autonomous execution/control fabric.
- **P61** — OmniCore convergence and PUI/runtime substrate.
- **P65** — autonomous systems/agentic OS factory.
- **P72** — cross-framework assurance.
- **P80** — self-evolving computing substrate.
- **P87** — Nexus hybrid language/compiler.
- **P90** — SOP/process compilation.
- **P91** — persona/style controls.
- **P97** — multimedia production.
- **P98** — general-intelligence research/evaluation.

## 11. Verification program

1. Full extension compile matrix.
2. Clean-install and upgrade/rollback tests.
3. Model-provider adapter conformance tests.
4. Dual-GGUF load/switch/recovery tests.
5. OCN registry schema and capability-count validation.
6. Single-agent vs swarm orchestration tests.
7. MCP scope/authorization/postcondition tests.
8. Vision/audio/GUI action replay with authoritative readback.
9. DGM-H candidate isolation and rollback tests.
10. Telemetry accuracy and actual-execution metering.
11. UI localization and SVG rendering regression tests.
12. Cross-platform Electron packaging tests.
13. End-to-end task replay: intent → plan → execution → verification → artifact.

## 12. Definition of Done

P100 reaches the next maturity gate only when:
- the product builds reproducibly;
- all enabled extensions pass integration tests;
- model and capability state are authoritative and inspectable;
- consequential actions have policy + postcondition verification;
- DGM-H candidates cannot bypass verification;
- failures are recoverable and auditable;
- reported performance is independently reproduced;
- the NeXus UI remains a projection of authoritative system state.

## Iteration 26 — GGUF Agent Studio / AI App Builder convergence

The supplied GGUF application report provides a concrete local-runtime implementation witness that strengthens P100. Its base architecture is Python + FastAPI + `llama-cpp-python`, with direct GGUF loading, WebSocket streaming, agent definitions, delegation and an Agents Builder. The initial safe workspace exposes `list_files`, `read_file` and `write_file` only inside a bounded workspace, deliberately omitting default shell execution.

The later implementation record reports a broader Enterprise surface: model routing, multiple models, purpose-specific planning/coding/embedding models, agent graph editing, handoffs, per-agent memory/versioning/tests, Pydantic tool schemas, self-healing tool calls, Docker/Kubernetes sandboxes, RBAC, prompt-injection/PII scanning, signed audit logging, repo ingestion, AST parsing, diff generation, React Flow, Monaco, live task timelines and observability. These are treated as source-reported implementation evidence, not independently reproduced verification.

The AI App Builder corpus reinforces P100's private/local-first architecture. Its analysis of Lovable and related systems emphasizes Vite/React/TypeScript, Supabase, sandboxed execution, Git as source of truth, MCP, Build/Plan modes, semantic Skills/RAG, RBAC/SSO and self-managed deployment.

P100 therefore treats GGUF Agent Studio and OmniStack AI as implementation witnesses and architectural inputs, not separate project identities.

## Iteration 27 — Lovable / local-first builder / cloud execution convergence

The new Lovable corpus adds a stronger architecture for controlled AI software generation: constrained project DNA, semantic context selection, Git-backed persistence, local-first/BYOK operation, sandboxed builds, structured testing and agent-swarm decomposition. It also reinforces the security boundary around generated database policies: LLM-generated RLS/configuration is proposal data and must be validated by authoritative policy tests before promotion.

P100 adopts these patterns as first-class product capabilities:
- **Project DNA:** explicit stack/version/schema constraints available to every agent;
- **semantic context broker:** retrieve only the code/state needed for the current task;
- **Git authority:** generated changes become candidate diffs until reviewed/tested;
- **local-first mode:** allow local model/runtime execution where privacy or cost requires it;
- **sandbox federation:** isolate generated builds and external tool execution;
- **persistent project memory:** retain architecture rules, failures and accepted decisions;
- **swarm scheduler:** delegate specialized work while keeping one capability/policy boundary;
- **security-policy verifier:** generated RLS, IAM and access rules require independent checks.

The supplied Lovable report compares Lovable, Bolt.diy, Libra AI, Dyad and Open-Laudable and documents materially different runtime/storage models. Those comparisons are architectural evidence, not proof that one platform is universally superior. fileciteturn883file3L653-L706 fileciteturn883file3L708-L850

## Iteration 27 — Locally Uncensored / Flow orchestration boundary

The LU/Flow material strengthens P100's multimodal production adapter: natural-language intent can be compiled into a structured master object containing metadata, entity registry and scene/timeline specifications; MCP can expose bounded media operations; and asynchronous orchestration can connect local planning with remote rendering. fileciteturn877file13L571-L598

P100 explicitly rejects credential extraction, hidden-endpoint abuse, CAPTCHA-token harvesting and authentication bypass as product capabilities. Only authorized APIs, explicit credentials, scoped tools, provenance and postcondition verification can enter the execution fabric. The source's descriptions of hidden endpoints and browser-token extraction remain threat-model material, not implementation requirements.

## Iteration 36 — MCP selective-context, sandbox and desktop-control convergence

The supplied MCP corpus adds a concrete context-efficiency layer to P100. Sequential Thinking is treated as a decomposition/reflection capability; Memory MCP as persistent graph memory; documentation servers such as Context7 as current grounding; GitHub MCP as repository control-plane access; Filesystem MCP as bounded file capability; and Ripgrep MCP as a high-selectivity retrieval accelerator. P100 should therefore route context through a **semantic context broker** instead of loading large repository regions indiscriminately. fileciteturn16file0L34-L64 fileciteturn16file0L67-L79 fileciteturn16file0L95-L116

The execution fabric is extended with an explicit capability taxonomy:

```text
READ CONTEXT
  ↓
PLAN / DECOMPOSE
  ↓
AUTHORIZE CAPABILITY
  ↓
SANDBOX / MCP / BROWSER / DESKTOP
  ↓
AUTHORITATIVE READBACK
  ↓
POSTCONDITION VERIFY
  ↓
ARTIFACT LEDGER
```

E2B-style sandboxes, Docker, WebAssembly/WASI and Electron/Tauri/CDP/IPC are treated as bounded execution substrates. The fact that an MCP server can expose a capability does not grant authorization to use it, and emitting a GUI/browser event is not sufficient evidence that the requested application state changed. fileciteturn19file2L80-L106 fileciteturn19file8L313-L328

### New P100 requirements

- selective context retrieval with provenance/freshness metadata;
- explicit tool side-effect declarations;
- per-capability authorization and quota;
- sandbox identity and lifecycle tracking;
- browser/desktop action readback;
- MCP tool-result classification as observation rather than authority;
- regression tests for confused-deputy and stale-context behavior;
- PL/EN UI localization remains a product invariant.

## Maturity
Architecture-ready product/integration concept. Source reports of installed extensions, successful builds, latency and RSI remain to be independently reproduced in the repository's verification environment.

## Derived from
P24, P27, P28, P37, P40, P54, P57, P59, P61, P65, P72, P80, P87, P90, P91, P97, P98, P114.

## Primary source witnesses
- `NeXus AI Code.md`
- `NeXus AI Code2.md`
- `NeXus AI Code3.md`
- `Architektura OMEGA-X Multimodalny Agent ASI oparty na języku Rust, Zenoh i DGM-H.md`
- `Architektura i Rozwój Systemu Nexus AI Code.md`
- `God Level.md`
- `Aplikacja GGUF .pdf`
- `Analiza Repozytoriów i Projekt Aplikacji.PDF`
- `Architektura własnej platformy typu AI App Builder.PDF`
- `Kod Lovable.dev i Alternatyw.PDF`
- `Integracja API z Locally Uncensored.PDF`
- `Zaawansowane Serwery MCP dla Agentów AI.PDF`
- `Zaawansowani Agenci AI: Tworzenie Oprogramowania i Procesy Kreatywne.PDF`

---

## Knowledge evolution — batch 15 / 2026-10-08: OmniStack source comparison and verifiable design/build/test log

The 34-page `Analiza Repozytoriów i Projekt Aplikacji.PDF` reviews eight report-claimed app builders (Convex Chef, Wasp MAGE, Open Lovable, Open Design, CodinIT.dev, December, Dyad, Bolt.diy) and proposes an OmniStack AI workbench, enforcer architecture and TDD-first workflow. P100 is the actual product/workbench surface; P33 manages delivery governance; P115 orchestrates tools.

Add `BuilderAdapterEvidence` with upstream URL+revision+license, offline/local/cloud capability, storage/auth model, cost and dependency snapshot, untrusted prompt boundary, sandbox availability, verified build receipt and change risk. `DesignEnforcer` checks real component tokens and accessibility; `ShadowQA` runs **actual** tests with receipts, not a report's fabricated supervisor console. Enforce secret scanning, CSP, backend auth, typed APIs, no unapproved changes to default branch and reviewed publishing. Upstream status and supplied success logs remain unverified until checked from repository/runtime.
