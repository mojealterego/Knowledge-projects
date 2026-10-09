# Project 97 — Project MIDAS Recursive Multimedia Production Factory MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Create a production-grade, feedback-driven AI content factory in which research, scripting, critique, editing, audiovisual planning, asset retrieval, rendering and post-publication learning form a controlled recursive loop rather than a one-pass automation.

## Distinct boundary
P56 owns broader AI commercialization/outcome delivery. P97 owns the specialized multimedia production pipeline and its recursive quality gates.

## Architecture

`brief/source → Architect → draft → Critic → revision → Editor → structured production plan → visual/audio asset retrieval → scene assembly → render → QA → publish → outcome telemetry → feedback into next production cycle`

## Core modules

1. **Architect Agent** — content structure, audience and narrative plan.
2. **Critic Agent** — adversarial review for factual, structural and quality defects.
3. **Editor Agent** — revision against critic findings.
4. **Scene JSON Compiler** — typed scenes, timing, narration, SFX cues and asset requirements.
5. **Asset Resolver** — authorized stock-media and generated-asset retrieval.
6. **Audio/SFX Layer** — explicit sound-cue planning and asset mapping.
7. **Renderer** — Creatomate or equivalent deterministic render backend.
8. **Aggregator** — joins rendered scenes into a final deliverable.
9. **Artifact Generator** — optional companion PDFs and other reusable outputs.
10. **QA / Evidence Gate** — validates schema, missing assets, timing, provenance and publication readiness.

## Recursive quality loop

The defining pattern is not `topic → video`; it is:

`plan → generate → critique → revise → render → inspect → measure → learn`.

No irreversible publication should occur solely because a model produced syntactically valid JSON.

## Integration
- P24: structured prompt/compiler patterns.
- P27: compound reasoning and disagreement-driven escalation.
- P56: commercialization and outcome measurement.
- P61/P80: agent/runtime orchestration.
- P72: runtime assurance and postcondition verification.
- P90: SOP compilation.
- P91: persona/style controls, where appropriate.

## Safety / quality boundaries
- Style references must not be used to falsely attribute generated work to living creators.
- Source claims require provenance and verification.
- The system must not fabricate testimonials, news or financial claims.
- Publication remains policy- and authorization-gated.
- Source-derived performance claims are hypotheses until measured.

## Verification
- schema/property-based tests for scene JSON;
- asset URL and media-format validation;
- deterministic render smoke tests;
- audiovisual timing checks;
- critic-vs-editor regression tests;
- factual provenance sampling;
- end-to-end Make.com scenario replay;
- rollback and partial-render recovery tests;
- outcome A/B testing against a documented baseline.

## Iteration 27 — Local orchestrator + cloud-render adapter

The supplied Locally Uncensored/Google Flow material strengthens P97 with a formal hybrid-render architecture. The durable pattern is:

`NATURAL-LANGUAGE BRIEF → MASTER STRUCTURED OBJECT → ENTITY REGISTRY → SCENE/TIMELINE MATRIX → AUTHORIZED MEDIA API → RENDER → EXTEND/UPSCALE → QA`

The source describes entity-aware scene generation, structured cinematic parameters and asynchronous media operations. It also identifies image prototyping, video generation, clip extension and final upscaling as separable pipeline stages. fileciteturn877file1L47-L80

P97 adopts:
- entity registry and stable asset references;
- typed scene/timeline representation;
- start-frame and continuity metadata;
- asynchronous render jobs;
- explicit render status/readback;
- local-first orchestration with optional cloud rendering;
- cost/latency-aware render routing;
- provenance for every generated media artifact.

The source additionally discusses reverse-engineering hidden service endpoints and extracting browser/session authorization material. Those mechanisms are explicitly outside P97. The production adapter uses only documented/authorized access paths and treats credentials as secrets outside generated scene data.

## Maturity
Architecture-ready specialization; production-scale economics and content-performance claims are NOT_VERIFIED.

## Derived from
P24, P27, P56, P61, P72, P80, P90, P91, P100.

## Source witnesses
- `PROJECT MIDAS.pdf` — 92-page automation blueprint.
- `Integracja API z Locally Uncensored.PDF`

## Batch 24 — 2026-10-09 PDF source evolution

18-page Locally Uncensored / Google Flow PDF describes local Tauri/Rust orchestration, ComfyUI/Wan, structured Master Prompt/ShotPlan and asynchronous cloud rendering. Adopt only documented authorized providers and typed async job status, source media rights, cost cap, continuity checks and render receipts. The document also discusses use of hidden browser-session interfaces: exclude that access pattern, do not extract session material or call undocumented Flow endpoints. Official Gemini API Veo 3.1 documentation provides a separate supported programmable path. No Google Flow account or API accessed, no media generated in this batch.
