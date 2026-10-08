# Project 80 — OmniCore Self-Evolving Computing Substrate MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-09

## Thesis

Project 80 turns the OmniCore concept from an AI-native operating system into a **measurable, self-improving computing substrate**.

The key distinction from Projects 61, 68 and 79 is the closed loop between **scientific discovery and the system that performs the discovery**:

```text
OBSERVE OMNICORE
      ↓
DISCOVER BOTTLENECK / DEFECT / OPPORTUNITY
      ↓
FORM COMPETING ENGINEERING HYPOTHESES
      ↓
GENERATE CANDIDATE KERNEL / COMPILER / DRIVER / RUNTIME CHANGE
      ↓
BUILD ISOLATED VARIANTS
      ↓
BENCHMARK / FORMALLY CHECK / HIL TEST
      ↓
COMPARE AGAINST BASELINE
      ↓
INDEPENDENT VERIFICATION
      ↓
SIGNED STAGED RELEASE
      ↓
HEALTH OBSERVATION
      ↓
ROLLBACK OR PROMOTE
      ↺
```

This is **not** unrestricted self-modifying code. Evolution is an experimentally controlled, versioned and reversible process.

## Why this is a new project

Project 61 converges Omnis, the learned kernel and PUI. Project 68 defines the constitutional runtime. Project 72 provides assurance. Project 79 provides autonomous scientific discovery.

Project 80 creates the missing **self-evolution substrate**: a concrete domain in which Project 79 can operate on the architecture itself while Project 72 prevents the discovery loop from becoming an authority loop.

## Iteration 12 source reinforcement
The newly supplied OmniCore documents independently reinforce the existing Project 80 architecture with three important dimensions:

1. **Self-healing loop:** anomaly detection → isolation → restart → candidate repair → verification.
2. **AI Foundry generation loop:** hardware discovery → authoritative documentation → generated driver candidate → compilation/verification.
3. **Cross-platform substrate:** Omnis/MLIR, learned scheduling, SemanticFS, heterogeneous execution and PUI are treated as candidate evolutionary surfaces.

The source material remains implementation inspiration and architectural specification, not proof that these capabilities are production-ready. In particular, generated low-level code remains untrusted until compilation, testing and assurance.

## 1. System architecture

```text
                         OMNICORE
                            │
          ┌─────────────────┼──────────────────┐
          │                 │                  │
      OBSERVATION       KNOWLEDGE          AUTHORITY
          │                 │                  │
          └────────────┬────┴───────┬──────────┘
                       ↓             ↓
                 OMNIDISCOVERY    POLICY / 72
                    (79)             │
                       ↓             │
               ENGINEERING HYPOTHESES
                       ↓
                VARIANT COMPILER
                       ↓
        ┌──────────────┼──────────────┐
        ↓              ↓              ↓
     KERNEL         OMNIS/CIRA      DRIVER/HAL
     VARIANT          VARIANT         VARIANT
        └──────────────┼──────────────┘
                       ↓
              DIGITAL EXPERIMENTAL LAB
                 QEMU / SIM / HIL
                       ↓
              ASSURANCE + REPLICATION
                       ↓
             STAGED ARTIFACT REGISTRY
                       ↓
             CONTROLLED ACTIVATION
                       ↓
                HEALTH READBACK
```

## 2. Four evolutionary domains

### A. Kernel evolution

Candidate changes include scheduling policy, memory paths, IPC, interrupt handling and resource allocation.

The AI Supervisor remains a bounded optimizer. Deterministic fairness, watchdogs, resource ceilings, emergency fallback and rollback remain authoritative. The source prototype treats learned scheduling as a kernel component, but a prototype is not evidence of production viability.

### B. Compiler evolution

Omnis uses MLIR to represent hybrid memory semantics, including `LinearRef`, `GcRef` and explicit hybrid scopes. The source material describes lowering and CIRA as an intelligent compiler pass.

Project 80 lets the discovery loop test compiler transformations against reproducible workloads rather than assuming a transformation is beneficial.

### C. Hardware / driver evolution

AI Foundry can inspect hardware, retrieve authoritative documentation and propose Rust drivers. Project 80 adds mandatory quarantine, compilation, sandbox/QEMU/HIL testing, signing and staged activation.

### D. Distributed substrate evolution

MeshBus exposes a device-level actor model with location transparency and predicted resource-aware task migration. Project 80 evaluates such migration experimentally across energy, latency, reliability and consistency dimensions.

## 3. Engineering Discovery Object

```yaml
EvolutionCase:
  id:
  baseline_artifact:
  target_layer:
  observed_problem:
  hypotheses: []
  candidate_variants: []
  workload_suite: []
  invariants: []
  predicted_effects: []
  measured_effects: []
  regressions: []
  security_findings: []
  formal_evidence: []
  replication_refs: []
  rollout_policy:
  rollback_policy:
  status:
```

## 4. Variant laboratory

No candidate replaces the baseline directly.

```text
BASELINE B
   │
   ├── V1
   ├── V2
   ├── V3
   └── Vn
        ↓
CONTROLLED BENCHMARK
        ↓
STATISTICAL COMPARISON
        ↓
SAFETY / CORRECTNESS GATES
        ↓
INDEPENDENT RE-RUN
        ↓
PROMOTION CANDIDATE
```

A candidate is rejected when it improves one metric while violating a hard invariant, even if its aggregate score is better.

## 5. Multi-objective optimization

The system must not optimize only for throughput.

A candidate vector is evaluated across:

- latency;
- throughput;
- energy;
- memory pressure;
- fairness;
- crash/recovery behavior;
- security surface;
- determinism;
- compilation cost;
- portability;
- observability;
- reproducibility.

The optimization result is therefore a Pareto frontier, not a single opaque score.

## 6. Constitutional evolution boundary

The system distinguishes four classes:

```text
MODEL PROPOSAL
ENGINEERING ARTIFACT
VERIFIED CANDIDATE
AUTHORITATIVE RELEASE
```

No transition is implicit.

```text
proposal → artifact: compiler/build gate
artifact → candidate: tests + invariants
candidate → release: independent assurance + approval policy
release → active: staged deployment + health verification
```

## 7. SemanticFS as evolutionary memory

SemanticFS stores searchable engineering evidence:

```text
artifact
benchmark
trace
failure
counterexample
hardware profile
compiler decision
release
rollback
```

Semantic similarity remains discovery only; access still requires provenance, capability and authorization.

## 8. AI Foundry + Omniscience + OmniDiscovery

The three loops are composed rather than duplicated:

```text
OMNISCIENCE
knowledge injection + interdisciplinary critique
                 ↓
OMNIDISCOVERY 79
unknown → hypotheses → discriminating experiment
                 ↓
OMNICORE 80
experiment → system variant → measured result
                 ↓
PROJECT 72
assurance → verification → release decision
                 ↓
KNOWLEDGE
                 ↺
```

## 9. Private experimental forge

The experimental Forge may use nested virtualization, GPU inference, local models, vector retrieval, QEMU/KVM and federated coding agents. Such infrastructure is a test substrate, not proof of portability to arbitrary environments.

## 10. PUI / Human Factor evolution

PUI is another measurable subsystem. Candidate changes are evaluated for rendering latency, interaction latency, multimodal synchronization, accessibility, adaptation accuracy, privacy and user-visible control of adaptation.

Affective or biometric observations cannot silently change capabilities or permissions.

## 11. Cross-platform invariance

The source proposes one codebase across x86_64 and AArch64, with WASM and binary translation for portability. Project 80 defines an invariance suite comparing semantic behavior, safety, resources and observable behavior across targets.

## 12. Scientific operating principle

```text
simulation ≠ hardware observation
benchmark ≠ universal law
model agreement ≠ evidence
performance gain ≠ correctness
single run ≠ replication
```

Every important improvement must have a baseline, workload, environment, measurement protocol, uncertainty characterization and replication path.

## 13. Hard invariants

1. The system may propose changes; it cannot self-authorize privileged changes.
2. Every release is versioned and rollback-capable.
3. Baselines remain immutable during an experiment.
4. Learned scheduling cannot violate deterministic safety constraints.
5. Generated drivers remain untrusted until independently verified.
6. Semantic retrieval never grants permission.
7. Presentation state never becomes authoritative system state.
8. A performance improvement cannot compensate for a safety regression.
9. One experiment cannot establish a universal engineering claim.
10. Independent replication is required for promotion of high-impact discoveries.
11. Failed variants become evidence rather than disappearing history.
12. Human approval, where required, is a resumable state transition.
13. Monitorability degradation increases verification requirements.
14. Supply-chain and documentation inputs remain untrusted until validated.
15. Unknown, hypothesis, observation, simulation and established result remain distinct epistemic types.

## 14. Evaluation gates

| Gate | Requirement |
|---|---|
| E0 | reproducible baseline |
| E1 | variant builds successfully |
| E2 | semantic/invariant checks pass |
| E3 | deterministic regression suite passes |
| E4 | isolated QEMU/sandbox test passes |
| E5 | performance/energy measurements recorded |
| E6 | security and supply-chain checks pass |
| E7 | independent rerun reproduces result |
| E8 | staged activation succeeds |
| E9 | authoritative health readback succeeds |
| E10 | rollback path demonstrated |

## 15. Roadmap

### Phase I — Experimental substrate
QEMU boot, immutable images, baseline benchmark suite, artifact manifests and deterministic fallback.

### Phase II — Learned scheduler laboratory
Compare deterministic baselines with bounded learned scheduling under identical workloads.

### Phase III — Omnis/CIRA laboratory
Implement a small MLIR dialect, controlled transformations and equivalence/regression tests.

### Phase IV — AI Foundry laboratory
Build documentation-to-driver pipeline with quarantine, compilation and HIL/QEMU verification.

### Phase V — Cross-device laboratory
Evaluate MeshBus-style actor migration across heterogeneous devices.

### Phase VI — Closed self-evolution loop
Connect Project 79 experiment selection to Project 80 variant generation, with Project 72 acting as the assurance boundary.

### Phase VII — External replication
Reproduce the strongest findings on an independent environment and hardware class.

## 16. Definition of Done

Project 80 is not complete when OmniCore boots.

It is complete when the system can demonstrate, reproducibly:

- generation of multiple competing system variants;
- automated benchmark execution;
- rejection of unsafe or regressive variants;
- independent reproduction of selected improvements;
- staged promotion and verified rollback;
- provenance-linked engineering memory;
- cross-platform behavior comparison;
- a complete `observe → hypothesize → build → test → verify → release → learn` loop.

## Evidence boundary

**SOURCE-DERIVED:** the supplied OmniCore documents describe the kernel, AI Supervisor, SemanticFS, Omnis/MLIR, CIRA, AI Foundry, MeshBus, PUI and self-healing concepts.

**INFERRED:** these components can form a self-evolution architecture only when connected through an experimental controller and strict authority boundaries.

**PROPOSED:** Project 80 is an engineering research program. It does not claim that an autonomous self-improving operating system has already been demonstrated.

---

## 2026-10-08 — batch 12: OmniCore MVP verification ladder

Source: `Tworzenie Systemu MVP Omnicore.pdf` (16 pages). The supplied `Cargo.toml`, Rust `no_std` entry/supervisor examples, Python AI Foundry and GDScript/Android launcher fragments are **proposed code in a report**, not compiled, linked or validated artifacts.

### MVP decomposition / privilege invariants
1. **Phase A — deterministic kernel stub:** build minimal `x86_64-unknown-none` or supported boot target, with explicit panic path, allocator/interrupt scope and reproducible QEMU boot smoke test.
2. **Phase B — external supervisor:** isolate any probabilistic AI scheduler in user space or a dedicated host process. No LLM-generated decisions or self-patching in Ring 0; kernel privilege boundary is not an inference target.
3. **Phase C — driver generator:** AI outputs an untrusted patch, reviewed via typed HAL contracts, static checks, host-side simulation, fuzzing and reversible promotion. Source rules such as "no unsafe outside HAL" do not prove memory safety.
4. **Phase D — Omnis/MLIR experiment:** toy grammar and dialect pass with differential tests; avoid conflating speculative Hilbert/TND terminology with physical or mathematical proof.
5. **Phase E — PUI adapter:** optional avatar/3DGS UI with bounded resource budget, graceful fallback and explicit consent; webcam emotion inference is out of scope by default.
6. **Phase F — signed release:** versioned images, SBOM, change log, checkpoint/rollback and independent postcondition readback.

### Acceptance
A real compiling kernel build and QEMU log precedes any `IMPLEMENTED` label; staged mutation/rollback tests precede `SELF_HEALING`. No hardware deploy, kernel build, physics verification or platform release occurred in this batch.

---

## Knowledge evolution — batch 14 / 2026-10-08: learnt computation hypothesis and capability testing

Sources: `Architektura AI Zastępująca Statyczny Kod (1).pdf`, `Architektura Systemu AI OmniCore Omega (1).pdf`. Both advocate NPS, AI Foundry driver synthesis and generative perception UI, but **provide no runnable proof** of general superiority to deterministic OS layers.

```yaml
LearnedSubsystemExperiment:
  component: scheduler|driver_foundry|PUI
  baseline_revision: null
  model_revision: null
  hardware_profile_verified: false
  sandbox_and_fallback: mandatory
  metrics: [p50_latency,p99_latency,correctness,fairness,power]
  policy_review: pending
  rollback_issued: false
  evidence_status: SOURCE_HYPOTHESIS
```

The trusted kernel always retains deterministic limits; learned outputs are proposals in user space with independent authorization. Target drivers require ABI, memory and safety verification. No kernel image, driver, AI supervisor, or 3D scene was built from this batch.
