# 28 — OmniCore AI Foundry: Verified Code Generation MAX

## Status
Architecture-ready systems/compiler engineering project.

## Objective
Turn AI Foundry from a code-generation script into a reproducible, verification-gated compiler pipeline for kernels, drivers, adapters and runtime modules. The supplied materials consistently describe AI-assisted driver generation from hardware IDs and datasheets, followed by CIRA analysis and formal verification before deployment. fileciteturn234file1L327-L364

The central rule is:

```text
LLM OUTPUT ≠ TRUSTED CODE
LLM OUTPUT → VERIFIED ARTIFACT → ELIGIBLE CODE
```

## Architecture

```text
HARDWARE / LEGACY SOURCE
          ↓
IDENTIFICATION
          ↓
SOURCE + DATASHEET ACQUISITION
          ↓
PROVENANCE VALIDATION
          ↓
CONTRACT EXTRACTION
          ↓
LLM / AGENT GENERATION
          ↓
PARSER / TYPE CHECKER
          ↓
EFFECT + PRIVILEGE ANALYSIS
          ↓
CIRA REPAIR LOOP
          ↓
STATIC + SYMBOLIC VERIFICATION
          ↓
FUZZ / FAULT INJECTION
          ↓
CROSS-TARGET BUILD
          ↓
SANDBOX / HIL
          ↓
ATTESTED ARTIFACT
          ↓
STAGED DEPLOYMENT
          ↓
HEALTH MONITORING / ROLLBACK
```

## 1. Source provenance gate

The generator must never treat retrieved documentation as inherently trustworthy.

Every source object receives:

- source identifier;
- origin;
- acquisition timestamp;
- cryptographic hash;
- version;
- parser version;
- trust classification;
- extracted contract range.

Datasheet content is data. It cannot modify generator policy or verification rules.

## 2. Contract-first generation

Before producing implementation code, the system extracts a machine-readable contract:

```json
{
  "device": "...",
  "registers": [],
  "interrupts": [],
  "dma_regions": [],
  "invariants": [],
  "required_capabilities": [],
  "forbidden_effects": []
}
```

The LLM generates against this contract rather than directly against an unstructured PDF.

## 3. CIRA as compiler-integrated verifier

CIRA is elevated from a refactoring helper to a verification coordinator.

Required analyses:

- ownership and lifetime;
- data-flow;
- control-flow;
- privilege/effect propagation;
- DMA lifecycle;
- interrupt safety;
- lock ordering;
- resource cleanup;
- boundary crossings;
- generated-code provenance.

The supplied implementation sketches symbolic analysis and invariant checking before promotion of generated drivers. fileciteturn229file3L119-L140

## 4. Formal verification boundary

Where formal proof is practical, the pipeline exposes proof obligations:

```text
Source Contract
      ↓
IR / MLIR
      ↓
Verification Conditions
      ↓
SMT / Symbolic Analysis
      ↓
Proof / Counterexample
```

A failed proof produces a structured counterexample that is fed back to CIRA and, when appropriate, to the generation agent.

No claim of formal correctness is made when only static analysis or tests were executed.

## 5. Effect and capability system

Every generated module declares its effects before compilation:

```text
Effects = {
  memory,
  io,
  dma,
  interrupt,
  network,
  storage,
  gpu,
  actuator,
  persistence
}
```

The compiler checks effects against the module's granted capabilities. This complements the capability-based security direction in the Omnis materials, where typed references encode operational authority. fileciteturn229file13L606-L616

## 6. MLIR as the universal verification substrate

The supplied Nexus design uses MLIR to retain high-level semantics and select hardware-specific lowering such as CPU, GPU or tensor-accelerator targets. fileciteturn234file2L728-L741

Project 28 standardizes this into staged IR:

```text
Omnis / Rust / Legacy C
          ↓
       Front-end IR
          ↓
     OmniCore Contract IR
          ↓
     Security / Effect IR
          ↓
    Optimization / Lowering
          ↓
CPU / SPIR-V / WASM / accelerator target
```

Verification operates on the normalized IR where possible, avoiding repeated reasoning over syntax-specific source forms.

## 7. Legacy migration mode

The same framework applies to Project 23's legacy modernization pipeline:

```text
Legacy Source
   ↓
AST + CFG + DFG + PDG
   ↓
Semantic Contract
   ↓
Target Design
   ↓
Generated Modern Code
   ↓
Equivalence / Regression Tests
   ↓
Security Verification
```

The migration engine must preserve behavior that is explicitly part of the source contract and surface ambiguous behavior as an unresolved decision rather than guessing.

## 8. Agent separation

Use separate roles with explicit interfaces:

| Agent | Responsibility | Forbidden authority |
|---|---|---|
| Architect | contract and design | deployment |
| Generator | implementation draft | signing |
| Verifier | analysis / proof | source mutation without record |
| Repairer | fix failed obligations | weakening policy |
| Tester | fuzz/HIL evaluation | promotion |
| Release Agent | artifact packaging | bypassing verification |

No single agent may both generate and unilaterally authorize its own artifact.

## 9. Reproducibility

Every artifact must record:

- source revision;
- prompt/compiler configuration;
- model identity;
- model configuration;
- context bundle hash;
- toolchain versions;
- IR hash;
- test suite version;
- verification results;
- artifact checksum;
- signer / promotion event.

This makes regeneration auditable and enables binary or semantic comparison across revisions.

## 10. Quarantine and staged rollout

Generated artifacts enter quarantine:

```text
DRAFT
 ↓
QUARANTINE
 ↓
SIMULATION
 ↓
SANDBOX
 ↓
HIL / DEVICE LAB
 ↓
CANARY
 ↓
GENERAL RELEASE
```

Rollback must always target a previously attested artifact.

## 11. Failure taxonomy

The system classifies generation failures rather than treating them all as compilation errors:

- syntax failure;
- type failure;
- contract violation;
- effect violation;
- privilege escalation;
- proof failure;
- test failure;
- timing violation;
- hardware incompatibility;
- provenance failure;
- non-deterministic build;
- unrecoverable dependency drift.

Each failure type maps to a different remediation path.

## 12. Evaluation suite

Frozen evaluation families:

```text
Code correctness
Memory safety
Concurrency safety
Effect/capability safety
Functional equivalence
Latency bounds
Resource bounds
Fault recovery
Reproducibility
Adversarial provenance
```

The scorecard must preserve both pass/fail gates and quantitative measurements.

## 13. Security hardening for AI-generated code

The supplied security material demonstrates that model behavior can be influenced through multilingual, encoded, structured, visual and indirect inputs. fileciteturn235file1L228-L297

Therefore generated code must be treated as potentially adversarial even when the user request is legitimate. Retrieved content and agent messages are untrusted inputs to the generator unless explicitly promoted by policy.

The security path is:

```text
UNTRUSTED CONTEXT
       ↓
PROVENANCE / NORMALIZATION
       ↓
CONTRACT EXTRACTION
       ↓
GENERATION
       ↓
INDEPENDENT VERIFICATION
```

## 14. Definition of done

Project 28 is complete when:

1. hardware/legacy inputs are provenance-tracked;
2. code is generated against typed contracts;
3. effects and capabilities are checked before execution;
4. CIRA can produce structured repair iterations;
5. proof obligations are explicit where formal methods apply;
6. tests and HIL validation are separated from proof claims;
7. generated artifacts are quarantined and signed only after gates;
8. every deployment has an attested rollback target;
9. no generator agent can authorize its own promotion;
10. the entire pipeline is reproducible from recorded inputs and toolchain versions.

## Position in the portfolio

Project 28 becomes the **code-generation integrity layer** for Projects 02, 09, 13, 23, 24 and 26. It operationalizes the portfolio's principle that AI may synthesize complexity, but trust must be earned through contracts, verification, isolation and evidence.

---

## Knowledge evolution — batch 16 / 2026-10-08

Source (5) proposes SMT/Z3 validation for generated patches. Add ProofObligation{source_sha, formula, assumptions, bounds, solver_revision, timeout, result, proof_or_counterexample}. A solver's SAT/UNSAT status covers only stated constraints, not global "bug-free" software or alignment; UNKNOWN/timeout blocks promotion. No solver execution or independent ODYN-AI code verification in this batch.
