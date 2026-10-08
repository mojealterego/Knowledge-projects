# Project 89 — CCR: The Closed Lens Interactive Game MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build the interactive game implementation of the **CCR** universe as a third-person narrative adventure/puzzle in Unreal Engine 5.4+, where light, memory, photography and temporal structure form the core mechanics.

Project 82 owns the graphic-novel/transmedia universe. Project 89 owns the game runtime and interactive implementation. Project 73 owns the separate Człowiek Roku game product.

## Source-derived world model
The supplied CCR report defines light as an active carrier of memory and the Closed Lens as a non-linear temporal structure. Shadows are presented as autonomous manifestations of memory and time. These are fictional/world-building axioms for the game, not claims about real physics.

## Core loop

```text
EXPLORE
 ↓
OBSERVE LIGHT / SHADOW
 ↓
USE CCR CAMERA
 ↓
REVEAL HIDDEN STATE
 ↓
TUNE TIME AXIS
 ↓
SOLVE PUZZLE
 ↓
ALTER / RECONTEXTUALIZE WORLD
 ↓
PERSIST CONSEQUENCE
```

## Core systems

### CCR Camera
A camera-based interaction system reveals objects, states or traces invisible to ordinary perception. Captured frames become typed game evidence or puzzle state.

### Closed Lens / TimeRift
The player tunes to an authored time axis rather than simply walking through a linear timeline. Time transitions change world layers while preserving explicitly persistent state.

### MemoryParticle
Light/memory particles act as interactive world objects. They may encode spatial clues, event fragments or puzzle parameters.

### Shadow Self
A shadow can be an autonomous temporal echo. Its behavior is driven by authoritative world state and puzzle rules rather than arbitrary model improvisation.

## Unreal architecture

```text
ACCR_GameMode
ACCR_Character
UCCR_CameraComponent
UCCR_EvidenceSubsystem
UTimeRiftSubsystem
AMemoryParticle
AShadowEcho
UCCR_PuzzleSubsystem
UCCR_WorldState
UCCR_SaveVersionManager
UCCR_VerificationSubsystem
```

## Temporal data model

```yaml
TimeAxis:
  id:
  label:
  valid_interval:
  world_layer:
  visual_profile:
  audio_profile:
  persistent_state_refs: []
```

## Light-memory model

```yaml
LightMemory:
  id:
  emitter:
  event_ref:
  capture_conditions: []
  reveal_state:
  temporal_axis:
  puzzle_payload:
  provenance:
```

The model is a game abstraction. It must never be presented as experimentally established physics.

## Technical implementation

The source proposes Lumen, Nanite, Niagara, World Partition and C++/Blueprints. The production implementation must benchmark these systems on target hardware and can replace individual technologies when constraints require it.

The camera system uses on-demand capture rather than unnecessary continuous rendering. Temporal state should be authoritative and presentation effects derived from it.

## Narrative and puzzle QA

- every puzzle has explicit prerequisites and valid solution states;
- time-axis transitions cannot corrupt persistent state;
- camera reveals are reproducible;
- shadow behavior remains within authored state bounds;
- no solution depends on hidden developer intent;
- save/reload reproduces the same puzzle state;
- spoilers are separated from player-facing marketing assets.

## Portfolio integration

- Project 19 — research/orchestration
- Project 50 — game/phygital foundry
- Project 52 — procedural design and playtest
- Project 57 — multimodal/camera interaction
- Project 61 — OmniCore/PUI/runtime convergence
- Project 72 — agent/runtime assurance
- Project 82 — CCR graphic-novel/transmedia canon
- Project 86 — future text-to-game delivery adapter

## Definition of Done

1. UE5 vertical slice is playable.
2. CCR Camera, TimeRift and MemoryParticle are functional.
3. At least one complete puzzle uses the Closed Lens mechanic.
4. Shadow behavior is deterministic from authoritative state.
5. Save/reload preserves temporal and puzzle state.
6. Visual/audio presentation is separated from authoritative game state.
7. Target-hardware performance is measured.
8. Narrative canon references are lineage-bound to Project 82.

## Iteration 25 evolution
CCR TOM III and TOM IV are now explicit canon witnesses for P89's authored timeline system. Add continuity anchors for 1679, 1983, 2016, 2025, 2035, 2045, 2135, 2345, 2378, 2387, 2777 and infinity. The 33-year cycle, 16.5-year breath, CCR-Rebuild, Memory Archive and Devil Mind are narrative state concepts. Their light/time/consciousness mechanisms remain fictional game abstractions and must not be presented as real physics.

---

## Integracja wiedzy — batch 11 (2026-10-08): CCR UE5 implementability gates

Źródło: `Unreal Engine 5- Tworzenie Gry Krok po Kroku.pdf`. **Status: SPECIFIED / NOT COMPILED IN UE5.**

1. `UCCR_CameraComponent`: zdjęcia tworzą wersjonowane zdarzenia pamięci/puzzli, widoczne tylko w autorskich warunkach.
2. `UTimeRiftSubsystem`: PRELOAD → CONFIRM DATA LAYER READY → TRANSITION → SWITCH AXIS → CHECK SAVE/PUZZLE STATE → RESUME / ROLLBACK. Migające przejście „błysk” musi mieć wariant bezmigotaniowy i ustawienie reduced motion.
3. `UNiagaraMemoryEmitter`: grafika cząsteczek nie stanowi źródła autorytatywnego stanu gry; profil CPU/GPU i limit cząstek.
4. `CCRVisualProfile`: skalowalne Lumen/Nanite/PostProcess, porównywalny benchmark i fallback.
5. `SaveCompatibility`: serializacja wątków fabularnych i osi czasu, ze sprawdzaniem wersji podczas migracji silnika.

**Akceptacja:** kompilacja C++ i testy Blueprint na faktycznie przypiętej wersji Unreal; zapisy i wznowienia z częściowo streamowanymi Data Layers, pomiar FPS i pamięci, kontrola deprecjacji API. 33-letnie pętle światła/czasu z PDF to fabuła, nie rzeczywista fizyka.
