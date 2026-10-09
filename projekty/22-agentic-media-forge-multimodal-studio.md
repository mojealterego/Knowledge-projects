# 22 — Agentic Media Forge MAX

## Status
Architecture-ready project.

## Objective
Create a multimodal production system for image, video, audio and screen-analysis workflows in which AI proposes transformations, deterministic tools perform measurable operations, and every final artifact carries provenance and evaluation metadata.

## Source-derived capabilities
The supplied Gemini materials describe multimodal reasoning, programmatic visual analysis with Python/OpenCV/NumPy, chart extraction, geometry computation, and high-resolution video analysis. fileciteturn217file2L91-L116

## Pipeline

```text
INPUT MEDIA
   ↓
MEDIA PROFILER
   ↓
TASK DECOMPOSER
   ↓
MULTIMODAL AGENT
   ├─ visual analysis
   ├─ audio analysis
   ├─ text extraction
   └─ contextual reasoning
   ↓
DETERMINISTIC COMPUTE
   ├─ image measurement
   ├─ frame extraction
   ├─ signal transforms
   └─ metadata normalization
   ↓
EDITOR / GENERATOR TOOLS
   ↓
QUALITY GATE
   ↓
PROVENANCE + EXPORT
```

## Core principle

Use the model for interpretation and orchestration; use deterministic code for quantities, transformations and validations whenever possible.

## Artifact model

Each output stores:

- source asset identifiers;
- transformation graph;
- model/provider metadata;
- tool versions;
- prompt/policy version;
- deterministic measurements;
- reviewer decisions;
- export checksum.

## Human controls

Editing operations that materially alter identity, rights, publication state or external distribution require explicit user approval.

## Evaluation

Measure visual fidelity, semantic adherence, artifact integrity, metadata correctness, temporal consistency for video, audio synchronization, deterministic-measurement agreement and recovery after failed processing.

## Security

No covert modification of provenance, no hidden payloads, no unauthorized publication and no processing of sensitive material beyond declared scope.

## Definition of done

The system can take a raw multimodal asset, execute a reproducible transformation pipeline, verify measurable properties and export a provenance-linked final artifact.

## Batch 24 — 2026-10-09 PDF source evolution

Locally Uncensored/Google Flow integration draft (18 pages) motivates provider-agnostic ScenePlan JSON → local renderer (optional ComfyUI/Wan) or officially documented hosted model endpoint → async job receipt → provenance and QA. Do not equate a consumer Google Flow project URL with a published API or credentials. Require license verification, consent for personal images, model version, billable cost, reproducible output and source-owner approval. No hidden service endpoints, login sessions or live video renders were used.
