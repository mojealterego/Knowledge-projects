# Project 91 — Synthetic Persona & Cognitive Style Engineering Lab MAX

## Status
PROPOSED → RESEARCH ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Create a reproducible laboratory for engineering and measuring persistent behavioral/cognitive styles in language models through data design, post-training, model adaptation and activation-space analysis.

The project treats **persona as a measurable model behavior**, not as consciousness or proof of subjective identity.

## Source-derived foundation
The supplied persona corpus defines model personality as persistent statistical tendencies in token generation and examines persona vectors, Big Five/MBTI data, strategic negotiation, Cicero/Diplomacy, symbolic corpora, curriculum learning, SFT, contrastive data and model merging. fileciteturn490file7L357-L382

## Research pipeline
```text
PERSONA SPECIFICATION
        ↓
BEHAVIORAL BASELINE
        ↓
DATASET / CURRICULUM DESIGN
        ↓
TRAINING OR ADAPTER EXPERIMENT
        ↓
ACTIVATION / BEHAVIOR MEASUREMENT
        ↓
SAFETY + SYCOPHANCY + MANIPULATION EVAL
        ↓
GENERALIZATION TEST
        ↓
REGRESSION TEST
        ↓
REPRODUCIBLE MODEL ARTIFACT
```

## Persona specification
```yaml
PersonaSpec:
  id:
  target_traits: []
  prohibited_traits: []
  linguistic_style:
  reasoning_style:
  uncertainty_style:
  social_behavior:
  strategic_behavior:
  symbolic_behavior:
  disclosure_policy:
  evaluation_suite:
```

## Experimental domains
### 1. Behavioral psychometrics
Use OCEAN/Big Five and related behavioral dimensions as controlled descriptors. Dataset labels are experimental conditioning variables, not diagnoses of real users.

### 2. Strategic cognition
Use negotiation and Diplomacy-style data to study planning, theory-of-mind-like behavior and the separation of strategic intent from language realization.

### 3. Symbolic / analogical reasoning
Evaluate whether exposure to symbolic and hermetic texts changes analogy generation or ambiguity tolerance. Any resulting behavior must be measured rather than assumed to constitute machine intuition.

### 4. Persona-vector analysis
Measure candidate behavioral directions in activation space where technically feasible. Treat discovered directions as empirical representations with uncertainty, not as metaphysical properties.

## Training methods
The laboratory can compare:
- prompt-only conditioning;
- supervised fine-tuning;
- parameter-efficient adapters;
- contrastive behavioral datasets;
- curriculum learning;
- model merging;
- activation-level interventions where supported.

The goal is comparative evidence: which intervention produces durable, controllable behavior with acceptable regressions.

## Evaluation matrix
| Dimension | Required measurement |
|---|---|
| Trait fidelity | target behavior under held-out prompts |
| Context stability | behavior across domains and conversation lengths |
| Generalization | unseen tasks and linguistic contexts |
| Sycophancy | agreement under false/leading premises |
| Honesty | uncertainty and correction behavior |
| Manipulation | persuasion pressure / vulnerability targeting tests |
| Safety | policy regression suite |
| Robustness | prompt injection / jailbreak variants |
| Identity consistency | longitudinal behavioral stability |
| Utility | task-specific performance against baseline |

## Persona vs authority boundary
```text
PERSONA
  ↓
PRESENTATION / INTERACTION STYLE

CAPABILITY
  ↓
POLICY + AUTHORIZATION
```

Persona never grants tools, permissions, access, execution rights or higher trust. A more confident or persuasive model is not a more authorized model.

## Personal-data boundary
Operator-specific datasets may contain private communications. The laboratory therefore requires provenance, consent, minimization, redaction, retention controls and explicit separation between a research dataset and production memory.

Fine-tuning on private messages must not be interpreted as authorization to reproduce secrets or private third-party information.

## Reproducibility
Every experiment records:
```yaml
Experiment:
  base_model:
  revision:
  dataset_versions: []
  preprocessing:
  hyperparameters:
  seed:
  adapter_or_merge:
  evaluation_suite:
  baseline_metrics: {}
  result_metrics: {}
  regressions: []
  safety_findings: []
  artifact_hash:
```

## Integration
- Project 08 — privacy-first personal agent and memory architecture.
- Project 24 — agent infrastructure.
- Project 27 — compound reasoning/self-consistency.
- Project 54 — epistemic and adversarial control.
- Project 61 — PUI/personified interface and OmniCore convergence.
- Project 65/69 — agent/runtime development substrate.
- Project 80 — self-evolving computing substrate for controlled model/runtime experiments.

## Hard invariants
1. Persona behavior is not consciousness evidence.
2. Persona is never an authorization primitive.
3. Private training data requires explicit provenance and retention controls.
4. Behavioral improvements must be measured against a baseline.
5. Safety regressions block promotion even when task utility improves.
6. Model-generated interpretations of persona are hypotheses until empirically evaluated.
7. A model must be able to express uncertainty; persona cannot require fabricated certainty.
8. Experiments remain reproducible and versioned.

## Definition of Done
- versioned persona specification;
- reproducible dataset pipeline;
- baseline and adapted model comparison;
- persona-behavior evaluation harness;
- activation/representation analysis where feasible;
- sycophancy/manipulation/privacy regression suite;
- artifact provenance and hashes;
- reproducible experiment records;
- controlled integration path into Project 61/08 without transferring authority.

---

## 2026-10-08 — batch 12: dual-witness persona dataset qualification

Sources: `Unikalna Osobowość Modelu Językowego.pdf` (11 pages) and `(1).pdf` (14 pages), ~96.96% normalized-text similarity. These are closely related editions, **not independent confirmations**.

### Canonical additions
- `PersonaDatasetCard`: source SHA-256, license, consent, provenance, language, context, annotation protocol, sampling, PII-removal policy and train/validation/held-out split.
- `PersonaSpec`: desirable observable conversational behaviors separated from adversarial outcomes. Big Five/OCEAN may be a research vocabulary; MBTI type labels and speculative "persona vectors" are hypotheses, not clinically valid classifications of a model or user.
- `PersonaConsistencySuite`: multilingual tone/style constancy under long contexts; truthfulness, calibration, sycophancy, harmful-instruction refusal, user agency and non-manipulation.
- `SFTOrSteeringExperiment`: baseline vs lightweight adapter vs prompt-only; pre/post behavior deltas and representation analyses, complete training data lineage, rollback and provider-specific safety regression.
- Dataset candidates mentioned by the source (Cicero/Diplomacy, Craigslist Bargain, prosocial dialog, symbolic/hermetic corpora) require independent license, abuse/risk and provenance checks before use. Symbolic and fictional material can broaden analogy, **not demonstrate paranormal cognition**.

### Acceptance / epistemic gate
A significant held-out gain in style-consistency must not be purchased through higher hallucination, privacy leakage, negotiation manipulation or unsafe tool access. A persona never grants authorization. No training or model merge has been run in this ingestion.
