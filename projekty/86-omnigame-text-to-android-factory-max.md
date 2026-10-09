# Project 86 — OmniGame Text-to-Android Factory MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build a governed production factory that converts a natural-language game idea or complete scenario into an inspectable game specification, engine project, reproducible Android build and signed APK/AAB.

The source architecture separates the Android control client, orchestration brain, procedural engine generators and CI/CD build farm. It uses **Universal Game Description Language (UGDL)** as the intermediate representation and targets Godot 4 and Unity 6 first.

## Product boundary
This is not a generic chatbot and not a prompt wrapper. It is a game compilation and delivery system:

```text
GAME IDEA / SCENARIO
        ↓
INTENT + REQUIREMENTS
        ↓
UGDL GAME MANIFEST
        ↓
GAME DESIGN VALIDATION
        ↓
ENGINE ADAPTER
   ┌────┴─────┐
 GODOT      UNITY
   │           │
   ↓           ↓
PROJECT ARTIFACTS
        ↓
ASSET / CODE VALIDATION
        ↓
HEADLESS BUILD FARM
        ↓
APK / AAB
        ↓
SIGN + VERIFY
        ↓
ARTIFACT DELIVERY
```

## UGDL

```yaml
UGDL:
  schema_version:
  game_id:
  genre:
  target_platforms: [android]
  scenes: []
  actors: []
  mechanics: []
  rules: []
  assets: []
  input_model:
  camera_model:
  audio:
  progression:
  accessibility:
  monetization:
  privacy:
  build_profile:
  source_refs: []
```

UGDL is the authoritative design representation. Engine files are compiled artifacts and must not silently redefine the game specification.

## Scenario compiler

Long-form scenarios are processed as:

```text
DOCUMENT
 ↓
SCENE SEGMENTATION
 ↓
CHARACTER / OBJECT EXTRACTION
 ↓
MECHANIC EXTRACTION
 ↓
WORLD / STATE EXTRACTION
 ↓
CONSTRAINT CHECK
 ↓
UGDL SYNTHESIS
```

RAG may provide source context, but retrieved text never becomes an executable instruction without schema validation.

## Engine adapters

### Godot
Generate and validate:
- `project.godot`;
- `.tscn` scenes;
- `.gd` scripts;
- resource references;
- Android export configuration.

### Unity
Generate and validate:
- `.unity` scene YAML;
- `.cs` scripts;
- `.meta` files and GUID relationships;
- package/configuration state;
- Android build settings.

### Future engines
Unreal support is an adapter research track because the source identifies `.uasset` and editor-dependent workflows as substantially different from text-native Godot/Unity project formats.

## Build Farm

```text
UGDL + ADAPTER OUTPUT
        ↓
ISOLATED WORKER
        ↓
ENGINE HEADLESS/BATCH MODE
        ↓
COMPILE
        ↓
UNIT / STATIC / ASSET CHECKS
        ↓
PACKAGE
        ↓
SIGN
        ↓
INSTALL / SMOKE TEST
        ↓
SCREENSHOT / LOG / PERFORMANCE EVIDENCE
```

Every build receives a reproducible manifest containing source revision, UGDL hash, adapter version, engine version, dependency lock and build environment.

## Android control client

The mobile application is a control plane, not the build environment. It supports:
- project creation;
- scenario upload;
- engine selection;
- build status;
- logs;
- artifact history;
- verification results;
- download/install handoff where platform policy permits.

## Agent topology

```text
MANAGER
 ├── Game Architect
 ├── Narrative Compiler
 ├── Mechanics Specialist
 ├── Asset Planner
 ├── Engine Adapter Agent
 ├── Build Engineer
 ├── Test Agent
 └── Security / Policy Agent
```

Agents produce typed artifacts. The manager does not promote unverified generated code.

## Verification

Required gates:

- UGDL schema validity;
- requirement coverage;
- source/provenance preservation;
- generated-code static checks;
- engine project integrity;
- build reproducibility;
- runtime smoke test;
- input/control validation;
- crash/log inspection;
- accessibility checks;
- security/supply-chain checks;
- APK/AAB signature verification.

## Safety and governance

Generated content, code and assets are untrusted until verified. Skills, MCP tools, model output and external dependencies are capability-scoped. No tool may expand project permissions through generated instructions.

The factory does not optimize compulsive engagement, covert persuasion or deceptive monetization. Game-economy and personalization mechanisms must remain explicit and auditable.

## Portfolio integration

- Project 07 — Agentic Creative Studio
- Project 24 — Agentic Prompt Compiler & DSL
- Project 28 — Verified Code Generation
- Project 33 — Agentic App Builder & Delivery Control Plane
- Project 52 — Adaptive Game Designer & Procedural Playtest Foundry
- Project 61 — OmniCore Convergence Fabric
- Project 65 — Autonomous Systems / Agentic OS Factory
- Project 69 — Agentic Development Runtime
- Project 72 — Cross-Framework Agent Assurance
- Project 80 — OmniCore Self-Evolving Computing Substrate

## Definition of Done

1. Natural-language scenarios compile into valid UGDL.
2. UGDL compiles into at least one reproducible Godot Android project.
3. Unity Android adapter produces a reproducible project/build for supported templates.
4. Generated projects pass static and runtime smoke checks.
5. Build artifacts have provenance and cryptographic identity.
6. Failed builds return structured diagnostics and can be resumed.
7. Human approval is required before external release.
8. Adapter/version mismatches are detected rather than silently repaired.

---

## Knowledge evolution — batch 16 / 2026-10-08

DGM architecture proposes 33 niche +33 developer apps and GameBuilder SDK. Loops emitting NicheApp_0..32 or scaffolding strings are **not** 66 built applications. Require per-project user problem, spec, source, licensing, build/test, signed binary, install, accessibility, error handling and independent review. New P126 owns IBM server-side game state and AI NPC backend reference; P86 retains game/app production. No game build completed in this ingest.

---

## 2026-10-09 — batch 23: seven-page GitHub MCP Registry discovery

The GitHub MCP Registry pages 1–7 display **Unity MCP** and **gamedev.pl** as game-development tools. Treat these as **candidate toolchains** only: review source/license, SDK/Unity version, game IP rights, explicit write scope and sandbox, deterministic source build and Android/desktop performance baseline. A catalog card does not supply a running Unity editor, account, editor bridge, generated game or signed APK. Existing P86 retains game creation ownership; no P127 created, external provider activated or user media transferred.
