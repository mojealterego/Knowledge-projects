# 23 — CIRA Graph Migration Engine MAX

## Status
Architecture-ready project.

## Objective
Build a graph-native legacy modernization system in which source code is transformed from flat text into a queryable Code Property Graph, analyzed by CIRA agents, and migrated incrementally with impact analysis, tests, RBAC and rollback.

## Source-derived architecture
The UGR/CIRA material treats code as a multidimensional graph combining control-flow, data-flow and dependency relationships rather than only AST structure. CIRA investigates technical debt and risky constructions and generates remediation proposals. fileciteturn219file7L39-L68

The IDE integration is explicitly separated into a lightweight client and a server that performs heavy graph analysis. LSP is used to decouple editor integration from analysis. Local graph caching and asynchronous deep queries reduce latency, while mTLS/OAuth and RBAC protect access to sensitive code. fileciteturn217file4L173-L200

## Architecture

```text
SOURCE REPOSITORIES
       ↓
PARSERS / AST / SEMANTIC INDEX
       ↓
UNIVERSAL CODE GRAPH
  ├─ AST
  ├─ CFG
  ├─ DFG
  ├─ PDG
  ├─ dependency graph
  └─ runtime observations
       ↓
CIRA INVESTIGATION AGENTS
  ├─ debt detector
  ├─ architecture analyst
  ├─ security analyst
  ├─ migration planner
  └─ test-impact analyst
       ↓
CHANGE PLAN
       ↓
SANDBOXED TRANSFORMATION
       ↓
BUILD + TEST + STATIC ANALYSIS
       ↓
IMPACT VERIFICATION
       ↓
HUMAN REVIEW / APPROVAL
       ↓
MIGRATION COMMIT
```

## Migration strategy

Avoid Big Bang Rewrite. Use:

`inventory → graph → bounded slice → characterization tests → transform → differential verification → deploy slice → expand`

Every migration unit has a rollback point and an explicit dependency/impact report.

## Graph contract

```text
Node: file | symbol | type | endpoint | database object | config | test
Edge: calls | imports | reads | writes | inherits | depends_on | configures | covers
```

## Agent policy

CIRA may propose changes autonomously, but modifications to protected modules require approval. The agent cannot broaden its repository scope through its own tool calls.

## IDE integration

Use an LSP-compatible client so the same CIRA service can support VS Code, JetBrains IDEs and future clients. Keep the UI thin; business logic belongs to the analysis service.

## Evaluation

- semantic preservation;
- test pass rate;
- graph completeness;
- false-positive rate;
- migration defect rate;
- dependency-impact precision;
- developer acceptance rate;
- rollback success;
- analysis latency.

## Definition of done

A migration is complete only when the new implementation preserves declared business behavior, passes the verification suite and has a machine-readable provenance chain from original code to final change.

---

## 2026-10-08 — batch 12: CIRA staged migration and equivalence dossier

Source: `Unifikacja Języków i Kodu Legacy.pdf` (7 pages). P87 owns the hybrid language/compiler, P23 owns legacy migration evidence.

```text
LEGACY BUILD + TEST BASELINE
 → DEPENDENCY / FFI / ABI GRAPH
 → NONDESTRUCTIVE REFACTOR PROPOSAL
 → PATCH + SOURCE / IR DIFF
 → STATIC ANALYSIS + FUZZ / DIFFERENTIAL TEST
 → POLICY / SECURITY / PERF GATES
 → HUMAN REVIEW + ROLLBACK
 → STAGED READBACK
```
New `EquivalenceDossier` records original and transformed hashes, interface behavior matrix, test coverage, unsupported semantics, exception/latency differences, version locks, provenance, counterexamples and rollback. MLIR IR lowering is not universally semantics preserving; its adequacy is a testable property of a specific domain subset.

Acceptance includes representative legacy integrations, stable ABI, exception and concurrency behavior, reversible deployment and detection of falsely asserted equivalence. No legacy repository was migrated as part of this batch.
