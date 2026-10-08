# 31 — OmniCore Sovereign Agent Operating Fabric MAX

## Status
Architecture-ready systems project.

## Objective
Unify the newest research set into a single control-plane architecture for sovereign, adaptive and auditable agent execution. The project combines agentic market patterns, autonomous software engineering, OmniCore hyper-adaptation, multimodal security evaluation, Skills, tool search, remote MCP, file search, code execution, OSINT/evidence workflows and explicit defenses against manipulation-oriented systems.

The source materials consistently converge on one observation: production-grade agents are not just models. They are stateful systems built from planning, tool use, reflection, specialized roles, memory/context management and verification. The market report describes the transition from chatbot to autonomous multi-step systems, while the engineering report emphasizes reflection, tool use, planning and multi-agent collaboration as reusable patterns. fileciteturn23file0L8-L29 fileciteturn23file3L29-L49

## 1. Core architecture

```text
                         USER / OPERATOR
                                │
                         INTENT / GOAL
                                │
                    ┌───────────▼───────────┐
                    │ SOVEREIGN CONTROL     │
                    │ PLANE                 │
                    │                       │
                    │ Policy                │
                    │ Identity / AuthZ      │
                    │ Provenance            │
                    │ Reasoning Budget      │
                    │ Data / Tool Budget    │
                    │ Safety / HITL         │
                    │ Evaluation            │
                    └───────────┬───────────┘
                                │
              ┌─────────────────┼──────────────────┐
              │                 │                  │
        Agent Runtime       Knowledge         Capability
              │             / Evidence          Catalog
              │                 │                  │
              └─────────────────┼──────────────────┘
                                │
                     PLANNER / ROUTER
                                │
                  TOOL SEARCH / DISCOVERY
                                │
                    ┌───────────▼───────────┐
                    │ AUTHORIZED CAPABILITY  │
                    │ BROKER                 │
                    └───────────┬───────────┘
                                │
       ┌────────────────────────┼────────────────────────┐
       │                        │                        │
     MCP                     Skills                  Built-ins
       │                        │                        │
       │                  Shell / Code              Web/File
       │                  Execution                 Search
       └────────────────────────┼────────────────────────┘
                                │
                         VERIFIED PLAN
                                │
                       SANDBOX / ISOLATION
                                │
                        EXECUTION / ARTIFACT
                                │
                    OBSERVABILITY / EVALUATION
                                │
                         VERSIONED RESULT
```

## 2. Agent operating-system layer

The supplied AIOS research identifies a useful abstraction layer between ordinary operating systems and LLM agents: an LLM System Call Interface, Agent Scheduler, Context Manager, Memory/Storage Manager and Tool Manager. fileciteturn15file2L21-L33 fileciteturn15file2L35-L68

Project 31 turns this into an explicit **Agent Operating Fabric**:

```text
agent_syscall(intent)
agent_alloc(reasoning_budget)
agent_mount(skill)
agent_search_tool(namespace)
agent_call(capability)
agent_checkpoint(state)
agent_verify(result)
agent_commit(artifact)
```

The OS analogy is used only as an architectural model. It does not grant the model privileged authority over the host operating system.

## 3. Dynamic capability discovery

Large tool surfaces are no longer loaded indiscriminately.

The current OpenAI tool architecture supports tool search and deferred loading, allowing namespaces and functions to be loaded at runtime. The documentation explicitly identifies tool search as a way to reduce token cost and latency when servers expose many functions. fileciteturn24file1L1294-L1348

Project 31 standardizes:

```text
TASK
 ↓
CAPABILITY SEARCH
 ↓
SELECT MINIMAL TOOL SET
 ↓
LOAD DEFINITIONS
 ↓
SCHEMA / POLICY CHECK
 ↓
EXECUTE
```

This extends the Project 29/30 Sugra doctrine to every large MCP or function namespace.

## 4. Skills as versioned executable knowledge

The Skills system provides versioned bundles containing `SKILL.md` instructions and optional scripts/assets. Skills can be mounted in hosted or local shell environments, and explicit versions can be referenced. fileciteturn28file0L14-L24 fileciteturn28file0L69-L95

Project 31 defines a **Skill Registry Contract**:

```json
{
  "skill_id": "...",
  "version": 3,
  "purpose": "...",
  "required_capabilities": [],
  "network_domains": [],
  "risk_class": "low|medium|high",
  "provenance": {},
  "evaluation_status": "pass|conditional|blocked"
}
```

A Skill is therefore treated like executable configuration plus instructions, not harmless prompt text. The supplied documentation explicitly warns that Skills can influence planning, tool usage and command execution, and recommends developer-level integration, bounded workflows and approval for sensitive actions. fileciteturn28file0L540-L568

## 5. Separation of reasoning, data and execution

The system distinguishes four artifacts:

1. **Observation** — externally retrieved fact or tool result.
2. **Inference** — model-derived interpretation.
3. **Plan** — structured intended action graph.
4. **Effect** — actual externally observable side effect.

This preserves the trust doctrine developed in Projects 26–30:

```text
OBSERVATION ≠ INFERENCE
INFERENCE ≠ AUTHORITY
PLAN ≠ EXECUTION
EXECUTION ≠ SUCCESS
```

## 6. Compound reasoning and model elevation

Project 27 already treats reasoning as a bounded resource. The new materials reinforce that system performance depends on orchestration rather than only on model scale. One report explicitly frames optimized performance around inference-time compute, caching, RAG and agentic workflows. fileciteturn16file4L185-L196

Project 31 adds provider-neutral routing:

```text
COMPLEXITY
CONSEQUENCE
UNCERTAINTY
LATENCY BUDGET
COST BUDGET
PRIVACY
MODEL HISTORY
      ↓
MODEL / STRATEGY ROUTER
      ├─ direct
      ├─ tool-assisted
      ├─ compound reasoning
      ├─ retrieval-grounded
      ├─ sandboxed code execution
      └─ human escalation
```

Claims about one smaller model actually outperforming a larger model remain empirical hypotheses and must be evaluated on frozen suites rather than assumed.

## 7. Software factory integration

The software-engineering research supports a staged agent factory in which coding agents plan, modify, run tests, inspect failures and iterate. The market report describes the autonomous software engineer as operating in precisely this loop. fileciteturn23file0L35-L48

Project 31 standardizes role separation:

```text
PRODUCT / INTENT
      ↓
ARCHITECT
      ↓
IMPLEMENTER
      ↓
TESTER
      ↓
SECURITY REVIEWER
      ↓
VERIFIER
      ↓
REPAIR AGENT
      ↓
RELEASE GATE
```

No single model instance receives authority to generate and unilaterally promote its own privileged artifact.

## 8. OmniCore adaptive hardware doctrine

The newer OmniCore research rejects forced pixel parity between devices and introduces Adaptive Fidelity: behavioral identity remains stable while rendering quality diverges according to hardware capability. fileciteturn23file2L21-L35 fileciteturn23file2L39-L46

Project 31 generalizes this into **Adaptive Capability Fidelity**:

```text
SAME INTENT
   ↓
HARDWARE / NETWORK / PRIVACY PROFILE
   ↓
ADAPTIVE EXECUTION PLAN
   ├─ low-power mobile
   ├─ workstation GPU
   ├─ edge NPU
   ├─ local CPU
   └─ remote accelerator
```

Only the implementation profile changes; policy, provenance and safety invariants remain stable.

## 9. Remote MCP trust boundary

The OpenAI documentation makes clear that remote MCP servers are external services that can read, send and receive data, and that developers must carefully inspect what data is shared. fileciteturn25file0L210-L213

Project 31 therefore mandates:

```text
REMOTE MCP
   ↓
SERVER IDENTITY
   ↓
TOOL INVENTORY
   ↓
ALLOWED_TOOLS FILTER
   ↓
APPROVAL POLICY
   ↓
DATA EGRESS CHECK
   ↓
CALL
```

The platform supports `allowed_tools`, which should be used to constrain large or sensitive servers to the minimum required function surface. fileciteturn25file0L491-L514

Approval is policy-driven. The system retains the ability to require explicit approval for tool calls and to record approval state before execution. fileciteturn25file0L691-L705

## 10. Multimodal security model

The supplied Gemini security research separates **jailbreaking** from **prompt injection** and correctly locates application compromise in the interaction between untrusted content and tool privileges. fileciteturn23file5L39-L56

Project 31 adds a unified adversarial matrix:

```text
TEXT
IMAGE / OCR
AUDIO / TRANSCRIPT
VIDEO
WEB CONTENT
LOGS / HISTORY
MEMORY
SKILLS
MCP METADATA
      ↓
UNTRUSTED INPUT NORMALIZATION
      ↓
PROVENANCE + CLASSIFICATION
      ↓
CONTEXT ISOLATION
      ↓
POLICY / CAPABILITY CHECK
      ↓
REASONING
```

No modality receives privileged authority merely because it appears in context.

## 11. Influence-security layer

Several supplied reports focus on visual persuasion, behavioral conditioning, coercive influence, sect dynamics, propaganda and narrative engineering. The material describes mechanisms intended to increase compliance or dependency. fileciteturn15file0L15-L33 fileciteturn23file6L16-L27 fileciteturn23file7L14-L28

Project 31 converts these materials into a **defensive Influence Security Module** rather than an influence-generation engine.

```text
CONTENT / INTERACTION
      ↓
INFLUENCE SIGNAL ANALYSIS
      ├─ coercion
      ├─ dependency loops
      ├─ social isolation cues
      ├─ deceptive framing
      ├─ urgency manipulation
      ├─ authority mimicry
      └─ reward/punishment cycling
      ↓
RISK SCORE + EXPLANATION
      ↓
USER PROTECTION / ESCALATION
```

This preserves research value while explicitly excluding covert manipulation as a system objective.

## 12. Behavioral analysis: uncertainty-aware deception detection

The supplied behavioral report states an essential methodological constraint: no single gesture, microexpression or movement is a definitive proof of lying. Instead, deception analysis should examine cognitive load and inconsistencies across communication channels. fileciteturn23file9L9-L23

Project 31 therefore prohibits binary “truth detector” outputs.

Required output:

```json
{
  "observations": [],
  "inconsistencies": [],
  "alternative_explanations": [],
  "confidence": 0.0,
  "not_a_determination": true
}
```

The agent reports evidence and competing hypotheses rather than asserting guilt or deception as fact.

## 13. OSINT / evidence fabric

Deep OSINT materials describe an iterative cycle in which every discovered data point can become a new pivot, while scope, legality and provenance must remain explicit. fileciteturn250file2L38-L55

Project 31 integrates this with Projects 19, 29 and 30:

```text
RESEARCH QUESTION
      ↓
TARGET / SCOPE
      ↓
SOURCE PLAN
      ↓
PASSIVE / PUBLIC DATA
      ↓
ENTITY RESOLUTION
      ↓
PIVOT GENERATION
      ↓
EVIDENCE GRAPH
      ↓
COUNTER-CHECK
      ↓
REPORT
```

Active interaction with external targets remains policy-gated and authorization-sensitive; the portfolio does not equate technical capability with permission.

## 14. File, web and code execution fabric

The current tool stack supports file inputs, web search, file search, code execution and remote MCP. fileciteturn24file0L399-L401 fileciteturn27file0L115-L117 fileciteturn27file0L384-L405

Project 31 creates a common execution interface:

```text
CapabilityInput
├─ text
├─ image
├─ file
├─ web
├─ structured data
└─ external tool

        ↓
NORMALIZER
        ↓
SOURCE / PROVENANCE
        ↓
ROUTER
        ↓
MODEL / SEARCH / CODE / MCP
        ↓
VERIFIER
```

This prevents each product from inventing a separate security model for every modality.

## 15. MCP Apps and decoupled UI

The latest UI guidance recommends MCP Apps as the standards-first foundation, with `window.openai` used only as an additive compatibility layer. It also recommends decoupling data tools from render tools and keeping authoritative state on the server. fileciteturn25file1L1028-L1054 fileciteturn25file1L1160-L1193

Project 31 therefore uses:

```text
DATA TOOL
  ↓
structuredContent
  ↓
MODEL CHECK / FILTER
  ↓
RENDER TOOL
  ↓
MCP APP
```

UI state remains presentation state; authoritative business state remains on the server. fileciteturn25file1L1375-L1415

For remote resources, CSP is declared narrowly using `connectDomains`, `resourceDomains` and only necessary `frameDomains`. fileciteturn25file1L1684-L1695

## 16. Submission and release engineering

The current plugin submission workflow requires accurate tool annotations, public MCP endpoints, domain verification when applicable, reviewer-ready test cases and a verified publisher identity. fileciteturn28file2L824-L838

Project 31 turns this into a release gate:

```text
SOURCE
 ↓
STATIC REVIEW
 ↓
SCHEMA / ANNOTATION CHECK
 ↓
SECURITY SCAN
 ↓
POSITIVE TESTS ≥ 5
 ↓
NEGATIVE TESTS ≥ 3
 ↓
DOMAIN / AUTH CHECK
 ↓
PROVENANCE / PRIVACY CHECK
 ↓
REVIEW BUILD
 ↓
RELEASE
```

The submission documentation explicitly requires five positive and three negative test cases. fileciteturn28file2L1079-L1098

## 17. Sovereignty and hardware root of trust

The Zero-Trust hardware report recommends a hardware root of trust, secure boot, remote attestation and strict compartmentation, and contrasts ARM TrustZone with open RISC-V approaches. fileciteturn250file3L8-L24 fileciteturn250file3L25-L34

Project 31 extends the trust chain:

```text
Hardware Root of Trust
        ↓
Secure Boot
        ↓
Verified Kernel
        ↓
Verified Agent Runtime
        ↓
Verified Skill / Tool Bundle
        ↓
Verified Policy
        ↓
Audited Execution
```

AI is deliberately kept outside the immutable trust root. Model output can recommend state changes; deterministic infrastructure decides whether those changes are admissible.

## 18. Definition of done

Project 31 is complete when:

1. agent capabilities are discoverable without loading the full catalog;
2. Skills are versioned, provenance-aware and policy-scoped;
3. tools are filtered to minimum necessary capability;
4. remote MCP services are treated as independent trust domains;
5. reasoning budget is explicit and measurable;
6. external observations are separated from inference and execution;
7. OSINT evidence is traceable and scope-controlled;
8. multimodal inputs pass through one normalized security boundary;
9. influence-analysis features are defensive rather than manipulative;
10. agent-created software passes verification before promotion;
11. UI/data state is separated using MCP Apps patterns;
12. release readiness is demonstrated by reproducible tests and negative-case evaluation;
13. hardware and platform adaptation never weakens policy or provenance invariants.

## Position in the portfolio

Project 31 becomes the **sovereign agent operating fabric** above Projects 13, 15, 17, 19, 24, 25, 26, 27, 28, 29 and 30. It is the point at which the repository's separate doctrines—agent OS, compound reasoning, evidence fabric, Skills, MCP, secure execution, adaptive hardware and adversarial evaluation—become one coherent control-plane architecture.

---

## Integracja wiedzy — batch 11 (2026-10-08): governance chmurowych agentów

Źródło: `Uruchomienie Agentów AI w Chmurze.pdf`; szerszy kontekst: [corpus batch 11](../docs/knowledge-base/2026-10-08-corpus-cloud-ccr-vantage-ai-2026-batch11.md). **Status: SPECIFIED / CLOUD NOT DEPLOYED.**

**Nowe komponenty:** `CloudCapabilityProbe` (zgodność nested VM/GPU, model regionów, kontrola rzeczywistych kwot), `AgentBuildSandbox` (L1/L2 QEMU i niezależna izolacja kernel Rust no_std), `ComputeBudgetLease` (limit kosztów, storage, cleanup), `BuildEvidenceGate` (artefakt, log, test, readback), `MCPPolicyBroker` (agent tożsamość ≠ prawo do hosta), `PrivacyGate` (zakaz domyślnej biometrii/wnioskowania o emocjach). Rozdzielić modele lokalne, VPC, RAG i execution auth; nie eksponować API lokalnych modeli publicznie.

```text
SPEC + PROVIDER CAPABILITY PROBE → SCOPED BUDGET
 → EPHEMERAL ISOLATED VM → BUILD / TEST
 → SOURCE + BINARY HASH + LOGS → INDEPENDENT VERIFIER
 → POSTCONDITION CHECK → HUMAN PROMOTION OR TEARDOWN
```

**Testy akceptacyjne:** odmowa operacji bez uprawnienia, quota exhaustion, brak GPU/virtualization, timeout kernel panic bez zagrożenia hosta, brak kosztu po cleanup i wymuszony audyt. Przykładowe maszyny N1/T4 ze źródła wymagają bieżącego sprawdzenia w GCP.
