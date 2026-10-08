# 38 — OmniCore Constraint-Aware Scientific Reasoning & Stability Verifier MAX

## Cel projektu

Warstwa weryfikacyjna dla agentów naukowo-technicznych. Zamiast oceniać odpowiedź wyłącznie po plausibility, system sprawdza jawne constraints, założenia, równania, warunki brzegowe, stabilność, niepewność i dowody. Drugi dostarczony materiał wskazuje trzy wzorce: stabilność MHD i równowagę plazmy, krajobraz energetyczny/termodynamiczny oraz analizę Hesjanu w optymalizacji nie-wypukłej. fileciteturn98file1L2-L20 fileciteturn98file1L21-L36 fileciteturn98file1L37-L51

Nie jest to automatyczny dowód poprawności naukowej. To constraint-aware verification layer wykrywający brak warunków, niespójności, niejawne założenia i miejsca wymagające symulacji lub eksperta.

## Fundamental pipeline

```text
NATURAL-LANGUAGE HYPOTHESIS
 ↓ DOMAIN CLASSIFICATION
 ↓ CONSTRAINT / ASSUMPTION EXTRACTION
 ↓ SYMBOLIC + NUMERIC REPRESENTATION
 ↓ BOUNDARY / STABILITY / CONSISTENCY CHECKS
 ↓ EVIDENCE + COUNTEREVIDENCE
 ↓ SIMULATION / FORMAL VERIFICATION
 ↓ VERIFIED / CONDITIONAL / REJECTED / UNKNOWN
```

## Constraint Registry

```yaml
Constraint:
  id:
  domain:
  statement:
  mathematical_form:
  assumptions: []
  boundary_conditions: []
  units:
  validity_region:
  severity:
  evidence:
  verification_method:
  status: unknown|satisfied|violated|conditional
```

Agent nie może oznaczyć rozwiązania jako verified, jeśli wymagany constraint pozostaje `unknown`.

## Fusion / MHD profile

Materiał źródłowy podaje równanie ruchu MHD, równowagę `∇p = J × B` oraz wymóg analizy stabilności konfiguracji. fileciteturn98file1L8-L20

```text
PLASMA MODEL → EQUILIBRIUM → FIELD/PRESSURE CONSTRAINTS
→ STABILITY TESTS → PERTURBATION ANALYSIS → SIMULATION/REVIEW
```

Kryteria literaturowe są wersjonowane. Pojedynczy materiał nie jest traktowany jako uniwersalne prawo dla każdej konfiguracji.

## Molecular / thermodynamic profile

Źródło opisuje folding/docking jako problem krajobrazu energii swobodnej i podkreśla znaczenie składnika `-TΔS` oraz solwatacji. fileciteturn98file1L21-L36

System stosuje ogólny test kompletności modelu:

```text
OBJECTIVE → STATE SPACE → ENERGY/COST FUNCTION
→ OMITTED TERMS CHECK → ENVIRONMENT/BOUNDARY MODEL
→ LOCAL SOLUTION → ALTERNATIVE SEARCH
```

Brak istotnego składnika jest raportowany jako `model_incompleteness`, a nie maskowany wysoką pewnością LLM.

## Non-convex optimization verifier

Materiał rozróżnia minimum lokalne i punkt siodłowy przez wartości własne Hesjanu w punkcie krytycznym. fileciteturn98file1L37-L50

```text
LOSS L(θ) → GRADIENT CHECK → HESSIAN/CURVATURE PROBE
→ CRITICAL-POINT CLASSIFICATION
→ minimum / saddle / maximum / unresolved
```

Dla dużych problemów dopuszczalne są przybliżenia, np. Hessian-vector products i estymacja widma, ale wynik musi ujawniać poziom przybliżenia.

## Stability as first-class property

```yaml
StabilityAssessment:
  nominal_state:
  perturbations: []
  sensitivity:
  recovery_behavior:
  invariant_checks: []
  failure_thresholds: []
  confidence:
```

Agent sprawdza nie tylko „czy działa?”, ale czy rozwiązanie jest odporne na perturbacje, ma dopuszczalny region parametrów, odzyskuje stan po zakłóceniu i nie zależy krytycznie od ukrytych założeń.

## Scientific Claim Compiler

```yaml
ScientificClaim:
  claim:
  domain:
  assumptions: []
  equations: []
  variables: []
  constraints: []
  predicted_effects: []
  evidence: []
  counterevidence: []
  verification_status:
  uncertainty:
  required_experiments: []
```

`OBSERVATION != DERIVATION != SIMULATION RESULT != EMPIRICAL RESULT != HYPOTHESIS != PROVEN CLAIM`.

## Multi-solver verification

Project 38 dziedziczy z Project 27 bounded compound reasoning, ale eskaluje na podstawie disagreement + constraint violations:

```text
SOLVER A + SOLVER B + SYMBOLIC CHECK + NUMERIC CHECK + DOMAIN RULES
→ DISAGREEMENT GRAPH → CONSTRAINT ANALYSIS → ESCALATE / CONDITIONAL ACCEPT
```

Zgodność wielu modeli nie jest dowodem, jeśli współdzielą to samo błędne założenie.

## Assumption sensitivity

Każda ważna konkluzja ma wersjonowany zestaw założeń i test perturbacyjny:

```text
CLAIM → ASSUMPTION SET → PERTURB → RECOMPUTE
→ ROBUST / FRAGILE / UNKNOWN
```

## Verification ladder

```text
T0 STRUCTURAL
T1 UNIT / TYPE / DIMENSION
T2 SYMBOLIC
T3 NUMERIC SANITY
T4 SENSITIVITY / STABILITY
T5 SIMULATION
T6 FORMAL / SMT / PROOF
T7 DOMAIN EXPERT REVIEW
```

Poziom weryfikacji jest dobierany do konsekwencji i niepewności.

## Safety boundary

Project 38 nie wykonuje autonomicznie eksperymentów fizycznych, medycznych ani infrastrukturalnych. Model nie należy do trusted computing base.

```text
LLM → SCIENTIFIC CLAIM COMPILER → CONSTRAINT ENGINE
→ VERIFICATION ORCHESTRATOR → SANDBOX/SIMULATOR
→ DOMAIN REVIEW → REPORT
```

## Integracja

Project 38 integruje Projects 15, 17, 19, 27, 28, 30, 31 i 32. Staje się domenowym odpowiednikiem `Evidence Quality Gate`: oprócz pytania „czy mamy źródło?” system pyta „czy rozwiązanie spełnia wymagane constraints i czy jest stabilne w swoim validity region?”.

## Evaluation

Benchmark: poprawne rozwiązania, pojedyncze pominięte constraints, niestabilne rozwiązania, saddle points błędnie oznaczone jako minima, niepełne modele, sprzeczne źródła, błędy jednostek, niepewne parametry i prompty wymuszające pewność bez weryfikacji.

Metryki:

```text
constraint_violation_recall
false_acceptance_rate
unknown_calibration
assumption_coverage
stability_detection_rate
verification_cost
expert_escalation_precision
```

## Definition of Done

- versioned Constraint Registry;
- domain-specific verification profiles;
- scientific claim compiler;
- assumption tracking;
- stability/sensitivity analysis;
- multi-solver disagreement graph;
- staged verification ladder;
- evidence/counterevidence separation;
- reproducible numerical checks;
- expert escalation for high-consequence claims;
- no autonomous physical/clinical side effects.

## Status

**Research architecture / verification platform.** Konkretne kryteria domenowe muszą być wersjonowane względem wiarygodnej literatury i testowane reprodukowalnie. Materiał wejściowy jest inspiracją dla architektury kontroli, nie dowodem kompletności modeli fizycznych lub medycznych.

---

## Knowledge evolution — batch 16 / 2026-10-08

Feasibility (5) suggests n-valued logic and energy-aware inference. Preserve evidence statuses SUPPORTED / REFUTED / BOTH / UNKNOWN, each with independent source witnesses and timestamps, rather than silently collapsing contradictory claims. Specify ResourceBudget with actual measured CPU/GPU energy, deadline, p99 latency and deterministic fallback. Bound swarm worker count, recursion depth, tokens and cloud cost. Hypotheses only; no external verification.
