# Project 114 — OmniCognitive Engineering Memory & Verification Substrate MAX

## Status
PROPOSED → ARCHITECTURE BASELINE

## Mission
Build a model-agnostic cognitive runtime substrate that gives agents durable, provenance-aware memory; multi-path inference; reflective verification; neuro-symbolic/structured validation; reversible self-improvement; and auditable state transitions.

## Problem
Prompt-level scaffolding and isolated context windows do not constitute durable cognition. The supplied corpus combines stateful memory, inference-time scaling, Generator/Verifier/Reviser loops, expert/contrarian evaluation, structured schemas, MCP/tool use, zero-trust execution and versioned rollback into a coherent runtime requirement.

## Architecture
```text
TASK / STATE
  ↓
MEMORY RETRIEVAL + PROVENANCE
  ↓
PROBLEM DECOMPOSITION
  ↓
MULTI-PATH SEARCH
  ├─ GENERATOR
  ├─ ALTERNATIVE HYPOTHESES
  └─ SPECIALIST / CONTRARIAN
  ↓
VERIFIER
  ↓
REFLECTION / REWARD
  ├─ ACCEPT
  ├─ BACKTRACK
  └─ REQUEST MORE EVIDENCE
  ↓
SYMBOLIC / EXECUTION VALIDATION
  ↓
POSTCONDITION
  ↓
VERSIONED MEMORY UPDATE
  ↓
PROMOTION / ROLLBACK
```

## Core modules
- **Memory Fabric:** episodic, semantic, procedural and working memory.
- **Provenance Graph:** source, timestamp, confidence, contradiction and supersession edges.
- **Conflict Resolver:** source hierarchy + external verification + temporal precedence.
- **Reasoning Scheduler:** consequence/uncertainty-aware inference-time compute allocation.
- **Cognitive Panel:** generator, verifier, reviser, specialist and contrarian roles.
- **Contract Engine:** typed XML/JSON/XSD/JSON Schema compatible intermediate representations.
- **Verification Fabric:** tests, formal checks where applicable, executable validation and postconditions.
- **Evolution Controller:** candidate generation, evaluation, promotion, rollback and regression tracking.
- **Durable State Machine:** resumable tasks, checkpoints and event history.
- **Tool Boundary:** MCP/tool calls treated as untrusted capability surfaces with least privilege and sandboxing.

## Iteration 36 — MCP selective-context and agentic workflow convergence

The supplied MCP corpus strengthens P114's context and tool boundary. It identifies Sequential Thinking as a decomposition/reflection surface, Memory MCP as persistent graph memory, documentation-oriented retrieval such as Context7 as grounding, GitHub MCP as repository control-plane integration, Filesystem MCP as bounded storage access and Ripgrep MCP as high-selectivity code retrieval. The engineering implication is that P114 should prefer **selective retrieval** over indiscriminate context loading. fileciteturn16file0L34-L64 fileciteturn16file0L67-L79 fileciteturn16file0L95-L116

New runtime contract:

```text
TASK
 ↓
CONTEXT BROKER
 ├─ MEMORY
 ├─ CURRENT DOCUMENTATION
 ├─ REPOSITORY SEARCH / RIPGREP
 └─ AUTHORITATIVE STATE
 ↓
DECOMPOSE / PLAN
 ↓
CAPABILITY BROKER
 ↓
SANDBOXED TOOL EXECUTION
 ↓
AUTHORITATIVE READBACK
 ↓
POSTCONDITION VERIFICATION
 ↓
MEMORY / EVIDENCE UPDATE
```

The agent-engineering corpus reinforces reflection, tool use, planning/decomposition and multi-agent collaboration as recurring workflow primitives. P114 therefore models them as explicit state transitions and contracts rather than prompt-only behavior. fileciteturn19file5L194-L225

## Iteration 36 — sandbox and desktop execution boundary

The MCP material describes E2B-style sandboxes, Docker, WebAssembly/WASI and Electron/Tauri/CDP/IPC execution surfaces. P114 treats these as **capability surfaces**, not authority sources. A sandbox completion, emitted GUI event or browser action is only an observation until authoritative readback confirms the requested postcondition. fileciteturn19file2L80-L106 fileciteturn19file8L313-L328

## Source-derived design evidence
The Enterprise blueprint specifies PostgreSQL/pgvector, Redis/BullMQ, Temporal durable workflows, Human-in-the-Loop signals, Firecracker/gVisor isolation, MCP routing, ephemeral agent identity and an Agent-Native IDE. fileciteturn1039file0L117-L226

The Gemini corpus describes inference-time multi-path search, Generator/Verifier/Reviser roles, explicit failure recognition, expert/contrarian evaluation and structured XML/XSD contracts. fileciteturn1041file0L232-L284

It also describes stateful memory with source hierarchy, external consensus and temporal conflict resolution, plus MCP zero-trust controls. fileciteturn1041file0L445-L530

The persona corpus reinforces instruction hierarchy, structured contracts, prompt-injection/prompt-leakage defenses and zero-trust tool execution. fileciteturn1040file1L571-L608 fileciteturn1040file1L1041-L1059

## Distinct boundary
- **P80** owns the broader self-evolving computing-substrate lineage.
- **P94** owns adaptive problem solving and strategic discovery.
- **P100** owns the NeXus AI Code developer environment.
- **P111** owns the enterprise multi-tenant agent platform/control plane.
- **P72** owns cross-framework assurance/runtime verification.
- **P114** owns the reusable cognitive substrate: stateful memory + reflective reasoning + verification + reversible evolution.

## Security invariants
- model capability ≠ authorization;
- memory ≠ authorization;
- approval ≠ execution;
- tool output is untrusted data;
- stale state cannot overwrite newer authoritative state;
- self-modification requires evaluation and rollback;
- no secret extraction or prompt-leakage capability;
- no undocumented endpoint abuse;
- no absolute-security claim without empirical evidence;
- selective retrieval must preserve provenance and freshness metadata;
- sandbox success does not constitute real-world execution evidence;
- GUI/browser event emission does not constitute successful state transition evidence.

## Verification plan
- memory consistency/provenance tests;
- conflict-resolution determinism;
- retrieval relevance and stale-memory suppression;
- selective-context vs full-context ablation tests;
- multi-path reasoning ablations;
- verifier/reviser error-detection benchmarks;
- structured-schema conformance;
- sandbox escape and confused-deputy tests;
- GUI/browser authoritative-readback tests;
- crash recovery and rollback;
- self-improvement holdout/regression suite;
- tool-injection/confused-deputy tests through P108;
- independent replication of performance claims.

## Dependencies
P54, P61, P72, P77, P80, P94, P100, P108, P111.

## Maturity
ARCHITECTURE_BASELINE

No production-readiness or unrestricted autonomous self-modification claim is implied.

---

## Knowledge evolution — batch 15 / 2026-10-08: feasibility-series bitemporal/semantic-caching evidence

Five ODYN/Nexus feasibility reports describe `bitemporal_store.py`, CoALA/GoT and (later) Semantic Clustering, Memory of Failures and Semantic Caching. Their "implemented" claims were **not independently inspected in ODYN-AI source**, and two versions (2)/(3) have text-identical content despite distinct byte hashes.

`SemanticCacheEntry` must include tenant_id, knowledge_source_sha, model_revision, prompt_revision, authorization context, retrieval as-of/effective time, retention limits, privacy class and validation/expiry state. No cross-tenant cache sharing, stale claims masquerading as fresh or reuse of prior tool rights. Distinguish `observed` versus `inferred` versus `verified` confidence on every memory edge; future dynamic code execution cannot inherit memory search authority. Cache correctness requires invalidation, negative tests and independent source readback.
