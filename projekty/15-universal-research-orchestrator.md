# 15 — Universal Research Orchestrator

## Objective
Transform the supplied Omniscience methodology into a reproducible research agent that can ingest domain sources, decompose a question, run parallel expert analyses, attack assumptions, synthesize hypotheses and produce an evidence-weighted conclusion.

## Core idea
The supplied protocol recommends deep context ingestion, domain-specific source selection, first-principles reasoning, multi-expert debate and explicit hypothesis generation. fileciteturn213file4L2-L31 fileciteturn213file4L33-L45

## Architecture

```text
Question
  ↓
Research Planner
  ├── domain decomposition
  ├── source plan
  └── evidence requirements
  ↓
Parallel Expert Agents
  ├── domain expert A
  ├── domain expert B
  ├── domain expert C
  └── adversarial critic
  ↓
Evidence Normalizer
  ↓
Hypothesis Engine
  ↓
Counterargument / Falsification Pass
  ↓
Synthesis Agent
  ↓
Evidence-Weighted Report
```

## Evidence model
Every claim receives:
- source reference;
- evidence type;
- confidence;
- uncertainty;
- competing explanation;
- verification status.

The system must explicitly distinguish observed fact, source assertion, model inference and speculative hypothesis.

## Research modes
`survey`, `deep-dive`, `comparative`, `forensic`, `hypothesis-generation`, `falsification`, `continuous-monitoring`.

## Tool plane
MCP-compatible connectors for web search, document stores, scientific databases, code execution and structured datasets. Discovery and authorization remain separate.

## Quality controls
Require source diversity, contradictory evidence search, citation coverage, arithmetic/code verification where relevant, and a final critic pass that attempts to falsify the synthesis.

## Output contract
```json
{
  "question": "string",
  "claims": [],
  "evidence": [],
  "hypotheses": [],
  "counterarguments": [],
  "uncertainties": [],
  "conclusion": "string",
  "confidence": 0.0,
  "next_experiments": []
}
```

## Acceptance criteria
The system must never present an unsupported hypothesis as an established fact, must retain provenance through multi-agent transformations, and must be able to resume a partially completed research run.


---

## 20.10.2026 — podagent EVIDENTIA RESEARCH (003)

Pełny Universal Research Orchestrator pozostaje specyfikacją, natomiast utworzono [OCAF-AGENT-003](../OCAF%20Agents%20MOJEALTEREGO/OCAF-AGENT-003/docs/README.md) pod nazwą **EVIDENTIA RESEARCH** (serial `AM-RES-EVI-2026-000003`) jako nowy Rust moduł lokalnej kontroli spójności przekazanych przesłanek. Nie wykonuje samodzielnego deep researchu, nie łączy się z serwerami OMAs/OMAt, nie uwierzytelnia źródeł ani nie używa OSTc. Projekt P15 nie został zastąpiony ani uznany za kompletny.
