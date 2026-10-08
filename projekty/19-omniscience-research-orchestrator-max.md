# 19 — OmniSynthesis Research Orchestrator MAX

## Status
Research → architecture-ready prototype.

## Objective
Create a rigorous virtual research institute that converts a difficult question into a structured evidence graph, parallel domain analyses, adversarial critique, explicit hypothesis space, falsification tests and a provenance-linked synthesis.

## Source-derived design
The supplied Omni-science protocol proposes deep domain context, targeted scientific databases and a multi-expert panel whose members challenge one another before synthesis. fileciteturn219file4L7-L27 fileciteturn219file4L28-L45

The project formalizes those ideas into a durable research runtime rather than a single prompt.

## Pipeline

```text
QUESTION
  ↓
SCOPE + SUCCESS CRITERIA
  ↓
SOURCE PLAN
  ↓
PARALLEL EXPERT TRACKS
  ├─ domain specialist A
  ├─ domain specialist B
  ├─ systems specialist
  └─ skeptic / falsifier
  ↓
EVIDENCE NORMALIZATION
  ↓
HYPOTHESIS GRAPH
  ↓
ADVERSARIAL CROSS-EXAMINATION
  ↓
PREDICTION + FALSIFICATION TESTS
  ↓
SYNTHESIS
  ↓
CONFIDENCE + PROVENANCE
```

## Expert contracts

Every specialist must return:

- claims;
- supporting evidence;
- assumptions;
- uncertainty;
- contradictions with other evidence;
- alternative explanations;
- proposed tests.

The skeptic has an explicit obligation to search for disconfirming evidence.

## Evidence model

```text
EvidenceRecord
├─ source_id
├─ source_type
├─ publication_date
├─ authority
├─ excerpt/reference
├─ claim_ids[]
├─ confidence
└─ retrieval_metadata
```

No claim enters the final synthesis without either a source reference or an explicit `hypothesis` label.

## Research memory

Persist:

- research_run_id;
- query decomposition;
- selected sources;
- extracted evidence;
- rejected hypotheses;
- unresolved questions;
- experiment results;
- final synthesis;
- provenance graph.

This makes research resumable and prevents losing intermediate discoveries when an agent run is interrupted.

## Retrieval strategy

The source explicitly illustrates domain-specific sources such as arXiv/CERN for physics, PubMed/PDB/AlphaFold for biology, and archival/paleoclimate data for history. fileciteturn219file4L12-L27

Implement a source registry where each research domain declares preferred primary sources, APIs, metadata fields and evidence-quality rules.

## Cognitive architecture

Use structured expert roles rather than uncontrolled role-play:

`analyze → critique → reconcile → test → synthesize`

The multi-persona panel is a mechanism for generating diverse hypotheses, not proof. Agreement between agents is never treated as evidence.

## Falsification engine

For each major hypothesis generate:

1. strongest supporting observation;
2. strongest contradiction;
3. testable prediction;
4. cheapest discriminating experiment;
5. conditions under which the hypothesis should be rejected.

## Output contract

```json
{
  "question": "...",
  "answer": "...",
  "claims": [],
  "evidence": [],
  "hypotheses": [],
  "contradictions": [],
  "tests": [],
  "uncertainty": [],
  "provenance": []
}
```

## Safety and epistemic controls

Research agents must distinguish:

- observed fact;
- source assertion;
- model inference;
- speculation;
- unresolved uncertainty.

For medical, legal and safety-sensitive research, the system must increase verification requirements and avoid presenting speculative synthesis as professional advice.

## Evaluation

Measure:

- citation correctness;
- evidence completeness;
- contradiction detection;
- hypothesis diversity;
- falsification quality;
- synthesis faithfulness;
- duplicate/redundant reasoning;
- time-to-answer;
- cost per research run;
- successful recovery after interruption.

## Definition of done

A run is accepted only when an independent reviewer can reconstruct how each material claim arose from the evidence and can identify what remains uncertain.

---

## 2026-10-08 — batch 12: research fan-out and historical evidence stress test

Sources: `Tworzenie zaawansowanego agenta badawczego.pdf` (13 pages), `Upadek Epoki Brązu- Klimat i Cywilizacje.pdf` (9 pages).

### New `DeepResearchRun` contract
```yaml
DeepResearchRun:
  question_id: null
  scope_and_deadline: null
  provider_capability_snapshot: null
  parallel_query_budget: null
  source_registry_refs: []
  freshness_policy: null
  citation_span_coverage: null
  contradiction_graph: []
  unsupported_claims: []
  independent_review_status: pending
```
A fast summarization model, model-generated consensus and claimed benchmark scores **cannot certify source quality**. Provider names (Gemini, OpenAI, DeepSeek), prices, availability and SWE-bench/GAIA claims in the 2025–26 source must be rechecked before performance comparisons.

### Historical verification fixture
The archaeological report combines Voynich parchment radiocarbon dating, material analysis, Late Bronze Age palaeoclimate, archaeoseismology, Ugarit texts, migration and trade disruption.
- `DateClaim` must distinguish measured **parchment material age** from the **date of text composition**.
- `ChronologyGraph` expresses calibrated measurement intervals, region, dating uncertainty and source evidence, rather than a single "Year of Collapse."
- `CausalHypothesis`: drought, seismic events, migration, warfare, maritime disruption, state fragility and trade feedback loops are candidates, **not proven simultaneous causes** from correlation alone.
- `CounterevidenceProtocol` requires dissenting chronologies, regional heterogeneity, confounds and explicit "insufficient evidence" output.

**Acceptance:** every archaeological/market claim traceable to provenance and measurement type; a cited source cannot be fabricated or treated as definitive because repeated by several models. No independent archaeology verification performed here.
