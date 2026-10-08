# Project 71 — OmniCore Physical Artifact Manufacturing & Symbolic Interface Factory MAX

## Mission
Create a production-grade compiler from symbolic/game semantics to reproducible physical artifacts and their digital companions, using the Apeiron 2.0 / Black Apeiron 2.2 material and game-production corpus.

## Architecture
```text
SEMANTIC / GAME STATE
        ↓
SYMBOLIC PROJECTION
        ↓
CARD / BOARD / TOKEN SPEC
        ↓
VISUAL + PHYSICAL PROFILE
        ↓
PREPRESS / MANUFACTURING PLAN
        ↓
PROTOTYPE
        ↓
PHYSICAL QC + PLAYTEST
        ↓
BATCH RELEASE
        ↓
DIGITAL COMPANION / REGISTRY
```

## Physical artifact schema
```yaml
ArtifactSpec:
  artifact_id:
  design_version:
  semantic_hash:
  ruleset_hash:
  substrate:
  thickness:
  optical_profile:
  ink_profile:
  tactile_profile:
  registration_tolerance:
  accessibility_profile:
  batch_id:
  qc_profile:
```

## Apeiron-derived manufacturing layer
The supplied Apeiron 2.0 report proposes transparent optical-grade PVC around 0.30 mm, high light transmission, controlled surface friction and multi-card overlay behavior. It also specifies hybrid opaque/translucent/transparent print layers, white-underprint levels, digital metallic effects, tactile finishing and a physical Light Matrix Board. These are treated as engineering hypotheses/specification targets until supplier-batch measurements establish actual transmission, haze, registration, friction, durability and overlay readability.

The supplied design defines a 78-card system whose meaning emerges through stacked layers rather than isolated cards. It also proposes a server-rack-style box, a light-matrix board and a System Kernel Manual. These become first-class ArtifactSpec components rather than informal decoration.

## Black Apeiron 2.2 / reveal-state layer
Black Apeiron 2.2 extends the same artifact lineage with dark/glitch aesthetics, thermochromic and UV reveal states, black-polymer presentation and a Codex-style manual. Project 71 formalizes these as rule-defined reveal states.

A hidden layer must never silently function as a behavioral-control channel. Any puzzle, UV, thermal, QR or overlay reveal is part of the declared game/artifact rules.

## Cyberpunk / Neon & Glitch design profile
The supplied 60-card Cyberpunk Tarot specification contributes a separate visual profile: System Core / Subroutines, Neon / Chrome / Data / Wires, glitch, datamoshing and pixel-sorting aesthetics. Project 71 treats this as a reusable visual manufacturing profile, not a new canonical project.

## Symbolic semantics
```text
SYMBOL → INTERPRETATION
OBSERVATION → EVIDENCE
RULE → GAME STATE
REVEAL → DECLARED INFORMATION STATE
```

These are different channels. A symbolic interpretation does not become factual state, diagnosis or guaranteed prediction.

## Manufacturing verification
```text
DIMENSION
→ THICKNESS
→ TRANSMISSION / HAZE
→ PRINT REGISTRATION
→ SURFACE FRICTION
→ REVEAL RELIABILITY
→ STACK ALIGNMENT
→ DURABILITY
→ ACCESSIBILITY
→ PLAYER TEST
→ BATCH REPLICATION
```

No supplier specification is promoted to verified physical fact without measurement. Single-prototype success does not establish batch reliability.

## Phygital continuity
Physical event detection must carry artifact identity, event sequence and version metadata. Out-of-order, duplicate or stale events are explicitly rejected or reconciled.

## Safety and cognitive-integrity boundary
Terms such as neuro-trap, subconscious commands, sensory addiction, hidden commands and compulsive collection are preserved as source-derived design history but are not production requirements.

Allowed design direction:

```text
OPTICAL ILLUSION → DISCLOSED EXPERIENCE
HIDDEN LAYER → RULE-DEFINED PUZZLE
TACTILE EFFECT → ACCESSIBILITY-TESTED INTERACTION
COLLECTION → OPTIONAL COMPLETION LOOP
QR / DIGITAL REVEAL → EXPLICIT USER ACTION
```

## Novel contribution
Project 71 is a manufacturing-aware artifact factory joining semantic identity, rules, physical process control, optical layering, reveal-state engineering, QC, accessibility and digital state without making the physical artifact a hidden control channel.

## Iteration 13 — Tarot taxonomy + Apeiron 78-card production schema

The new Tarot taxonomy is added as a manufacturing input ontology. The compiler now treats historical deck families and structural variants as explicit lineage metadata rather than assuming a universal 78-card standard.

```yaml
DeckManufacturingSpec:
  lineage_id:
  card_count:
  historical_family:
  major_arcana_profile:
  minor_arcana_profile:
  court_profile:
  numbering_profile:
  correspondence_profile:
  visual_profile:
  physical_profile:
  accessibility_profile:
  provenance:
```

The Apeiron 78-card source adds deterministic HEX/checksum overlays and Stack Trace-style layered protocols. Project 71 maps these to production-testable optical regions:

```yaml
OverlayRegion:
  card_id:
  layer_index:
  opacity:
  alignment_tolerance:
  hex_value:
  overflow_rule:
  reveal_condition:
  readability_test:
```

HEX values, overflow states and `FF`/critical states are treated as declared symbolic/game encodings. They are not promoted to cryptographic truth or empirical prediction.

### Added physical QA

- verify 78-card stack readability across representative combinations;
- measure registration drift across batches;
- test optical transmission/haze after printing;
- verify HEX/checksum visual legibility under intended lighting;
- test reveal-state reliability and reset behavior;
- blind-playtest interpretation consistency without teaching a false empirical claim;
- preserve lineage/version identity on every production batch.

The Tarot study also documents transparent, round and double-sided deck families. Project 71 therefore supports these as physical-profile variants rather than forcing all decks into rectangular opaque-card assumptions.

---

## Knowledge evolution — batch 14 / 2026-10-08: AURA manufacturing-qualification contract

Five-page AURA source specifies 330gsm Black Core, Waterless UV print, reversible thermochromic ink, tactile soft-touch + 3D varnish, cold foil mirrors, scent effects, magnetic book box and NFC.

**Blocking contradiction:** thermochrom activation stated as 29°C in architecture and 26–27°C in supplier section. No verified material tolerance, shelf/heat stability, repeated-cycle tests or batch supplier acceptance. `ThermochromQualification` records actual test range, measured reveal/revert time, temperature tolerance, contrast, chemical/allergen and skin-contact safety, ink adhesion and lot identifiers.

`ProductionGate`: match 60 inventory IDs, rights/print layers, physical QC, NFC consent/URL/audio opt-in, availability for unscented/accessibility alternatives, timestamped quote and tested supply chain. Supplier names in PDF are proposals, not a verified purchase/order. No prototypes, vendor quotes or tests have been conducted.
