# Project 87 — Nexus Hybrid Programming Language & Compiler MAX

## Status
PROPOSED → RESEARCH ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Research and prototype the proposed **Nexus** programming language as a unified systems/application/scripting language, together with a compiler and interoperability toolchain based on LLVM/MLIR.

The source specification synthesizes paradigms from C/C++, C#, JavaScript/TypeScript, Lua, Python, GDScript, Java, Kotlin and Swift. It proposes a single environment spanning low-level systems, games, backends and dynamic scripting.

## Compiler architecture

```text
NEXUS SOURCE
    ↓
LEXER / INDENTATION PARSER
    ↓
AST
    ↓
TYPE + OWNERSHIP ANALYSIS
    ↓
EFFECT / CONCURRENCY CHECKS
    ↓
NEXUS IR
    ↓
LLVM / MLIR LOWERING
 ┌──────┼────────┐
NATIVE   WASM     DYNAMIC
  ↓       ↓         ↓
C/C++ ABI JS/DOM   VM/GC
```

## Memory model

The proposal combines:
- affine ownership and borrowing as the default deterministic domain;
- ORC-style owned reference counting for shared objects;
- isolated tracing-GC regions for dynamic workloads;
- compile-time lifetime analysis;
- explicit dynamic/GC-managed boundaries.

The design is a research hypothesis. Performance and safety claims require compiler prototypes and benchmarks.

## Type system

Core proposals:
- reified generics;
- sound gradual typing;
- explicit nullable types;
- `let` immutable bindings and `var` mutable bindings;
- exhaustive pattern matching;
- compile-time checking at static/dynamic boundaries.

## Concurrency

The language proposes:
- structured concurrency;
- M:N coroutine scheduling;
- async/await with explicit scopes;
- actor-style mutable-state isolation;
- integration with single-threaded event loops for JS/WASM targets.

## Error model

Normal recoverable failures use `Result<Value, Error>` and explicit propagation. Panics are reserved for unrecoverable conditions. The compiler must make error-handling obligations statically visible.

## Interoperability

### C/C++
LLVM/MLIR-based ABI integration is intended to reuse existing native libraries.

### Python/Lua
Dynamic-language support is a proposed embedded bytecode/interpreter domain rather than a claim of drop-in compatibility.

### Web
The proposed web backend emits WebAssembly for compute-heavy code and JavaScript for DOM interaction.

## Toolchain

```text
nexus fmt
nexus check
nexus test
nexus build
nexus run
nexus doc
nexus bindgen
nexus migrate
```

Every command should expose machine-readable diagnostics for integration with agentic development systems.

## Verification strategy

The project must establish evidence for each major claim:

| Claim | Required evidence |
|---|---|
| Memory safety | compiler test suite + adversarial lifetime corpus |
| Sound gradual typing | formal model + boundary tests |
| Structured concurrency | cancellation/leak tests |
| Performance | benchmark vs representative C++/Rust/Python workloads |
| ABI compatibility | compiled interoperability tests |
| WASM correctness | differential/reference tests |
| Compiler reliability | fuzzing + regression corpus |
| Toolchain reproducibility | pinned compiler/dependency/build manifest |

## AI-assisted compiler

AI may propose code, optimization or migration transformations, but the compiler remains the authority. Generated transformations must pass parsing, typing, ownership, semantic, test and equivalence gates.

```text
AI PROPOSAL
 ↓
DIFF
 ↓
STATIC CHECKS
 ↓
TEST
 ↓
BENCHMARK
 ↓
OPTIONAL FORMAL EVIDENCE
 ↓
HUMAN / POLICY PROMOTION
```

## Portfolio integration

- Project 24 — Agentic Prompt Compiler & DSL
- Project 28 — Verified Code Generation
- Project 61 — OmniCore Convergence Fabric
- Project 65 — Autonomous Systems / Agentic OS Factory
- Project 68 — Constitutional Runtime Fabric
- Project 80 — OmniCore Self-Evolving Computing Substrate

Project 87 owns the language/compiler lineage. Project 61 owns system convergence; Project 80 owns governed self-evolution. They must not be collapsed into one identity.

## Definition of Done

1. Formal grammar exists.
2. Parser and AST prototype compile representative examples.
3. Type/null/ownership checker rejects defined invalid programs.
4. At least one MLIR/native backend executes a representative subset.
5. WASM backend passes differential tests for its supported subset.
6. C ABI interoperability is demonstrated with test libraries.
7. Structured concurrency semantics have executable tests.
8. Compiler diagnostics are machine-readable.
9. Fuzzing and regression suites are reproducible.
10. No claim of replacing existing languages is made before comparative evidence exists.

---

## 2026-10-08 — batch 12: MLIR/Omnis and CIRA legacy-verification contract

Source: `Unifikacja Języków i Kodu Legacy.pdf` (7 pages); earlier knowledge: `docs/knowledge-base/omnis-hybrid-language-and-cira.md`, P23 and P80.

### Proposed technical increments
- Define an MLIR multi-dialect prototype with explicit high-level scripting operations, typed `Affine` loop transforms and separately controlled hardware-lowering passes. **No claim of lossless universal compilation** is accepted without semantic proof.
- `MemoryBridgeIR`: compile-time ownership/region annotations, reference lifetimes and dynamic-object boundary; prohibit assuming Rust-like safety if a GC-managed object crosses an unchecked FFI.
- `CiraMigrationProof`: `source_sha`, source AST, generated IR, target diff, test artifacts, static analyzer findings, symbolic/path-condition obligations, ABI compatibility and rollback.
- `DifferentialExecution`: compare legacy vs rewritten output on stable fixtures, fuzz targets, concurrency scenarios and error paths; use compiler-level verification where actually feasible.
- `TargetCapabilityRegistry`: CPU baseline first, GPU/FPGA backends only after target toolchains, ABI ownership, accuracy and maintenance feasibility are proven.

**Definition of Done:** a minimal executable grammar/AST/IR subset, a lowering example and differential tests. Source code does not exist merely because the PDF names Omnis/Nexus; the 2026-10-08 batch does not build a compiler.
