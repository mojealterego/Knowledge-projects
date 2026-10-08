# Project 50 — OmniCore Physical Game & Phygital Experience Foundry MAX

## Thesis
Turn the supplied Apeiron/AURA/game-design corpus into a safe, measurable, engine-neutral system for creating, simulating, manufacturing and digitally extending physical games and interactive artifacts.

The project treats the physical component as a first-class interface while keeping all persuasive, gambling and behavioral-control mechanisms behind an explicit cognitive-safety boundary.

## Core pipeline

```text
GAME / ARTIFACT INTENT
        ↓
ONTOLOGY + RULESET SPEC
        ↓
CARD / COMPONENT GENERATION
        ↓
PHYSICAL INTERACTION MODEL
        ↓
DIGITAL COMPANION MODEL
        ↓
SIMULATION + PROBABILITY ANALYSIS
        ↓
SAFETY / ACCESSIBILITY / AGENCY GATES
        ↓
PROTOTYPE
        ↓
PLAYER TESTING
        ↓
MANUFACTURING SPEC
        ↓
RELEASE + TELEMETRY
        ↺
ITERATE
```

## 1. Card Artifact Compiler

A single typed artifact definition drives physical print, game logic and digital companion layers.

```yaml
CardSpec:
  card_id:
  deck_id:
  archetype:
  suit:
  value:
  rules_effect:
  trigger:
  visual_layers: []
  physical_layers: []
  reveal_mode:
  digital_binding:
  accessibility:
  provenance:
  print_profile:
  safety_profile:
```

## 2. Layered Physical Interface

Apeiron's overlay model is generalized into:

```text
CONTEXT
 + ACTOR / ARCHETYPE
 + MODIFIER / CONSTRAINT
 + OUTCOME
```

Transparent or semi-transparent substrates, white-underprint levels and tactile/optical finishes become explicit rendering parameters rather than hidden implementation assumptions. The supplied Apeiron design specifically proposes transparent layers and variable opacity as core mechanics. fileciteturn64file5L398-L422

## 3. Safe Sensory Interaction Layer

The system may use:
- thermochromic reveal;
- selective UV;
- optical interference;
- tactile textures;
- conductive interaction;
- phosphorescent or fluorescent visual accents.

However:

`SENSORY EFFECT ≠ BEHAVIORAL CONTROL`

Effects must have a stated player-facing purpose and pass physical-safety and accessibility review.

The supplied AURA corpus describes thermochromic, scented, conductive and glow effects. fileciteturn68file0L10-L48

## 4. Rules Engine

Rules are machine-readable and independent from presentation.

```yaml
Rule:
  id:
  preconditions: []
  action:
  targets: []
  randomness:
  reward:
  penalty:
  resolution_order:
  visibility:
  reversibility:
```

This prevents the physical print layer from becoming the hidden source of gameplay truth.

## 5. Probability Simulator

The lottery corpus provides a reusable framework for outcome-space analysis: pool size, draw size, player selection, prize classes and multi-pool products. fileciteturn69file5L31-L47

The simulator reports:

```text
STATE SPACE
→ OUTCOME DISTRIBUTION
→ VARIANCE
→ EXPECTED VALUE
→ PAYOUT DISTRIBUTION
→ EDGE / FAIRNESS
```

For random mechanics, simulations must expose assumptions and seeds so results are reproducible.

## 6. Competitive Integrity Layer

Every stochastic mechanic receives:

- deterministic test seeds;
- statistical distribution tests;
- boundary-case tests;
- replay verification;
- fairness metrics;
- exploit-surface analysis.

Critical randomness is isolated from presentation logic, following the architectural separation visible in the supplied casino-system model. fileciteturn68file1L73-L138

## 7. Push-Your-Luck Without Compulsion Engineering

The supplied AURA rules show reveal, declaration, challenge and push-your-luck resolution. fileciteturn68file3L200-L221

Project 50 retains the game mechanic but explicitly removes:

```text
NO LOSS-CHASING OPTIMIZATION
NO VARIABLE-REWARD DEPENDENCY TARGETING
NO VULNERABILITY TARGETING
NO HIDDEN PSYCHOLOGICAL COMMANDS
```

The design objective is strategic tension, not engineered compulsion.

## 8. Phygital Bridge

```text
PHYSICAL CARD
    ↓
IDENTITY / SIGNAL
    ↓
MOBILE / WEB COMPANION
    ↓
DIGITAL STATE
    ↓
OPTIONAL ASSISTANCE / LORE / SCORING
    ↓
RETURN TO PHYSICAL GAME
```

Digital state changes require visible transitions and authoritative validation. QR, NFC and conductive-ink mechanisms are treated as optional transport technologies rather than hidden authorization channels.

## 9. Deck Compiler and Variant System

The tarot taxonomy demonstrates that card systems can legitimately vary in card count, hierarchy and nomenclature. Minchiate expanded to 97 cards, while Tarocchino reduced the structure to 62 cards. fileciteturn66file4L179-L203

Project 50 therefore supports:

```yaml
DeckVariant:
  base_system:
  card_count:
  major_structure:
  minor_structure:
  suits: []
  court_model:
  correspondence_model:
  game_mode:
  interpretation_mode:
```

## 10. Cyberpunk / Apeiron Visual Grammar

The supplied cyberpunk specification demonstrates a 22-card System Core plus 38-card Subroutines with four domain-specific suits. fileciteturn66file0L17-L30

The compiler can generate domain-specific ontologies while retaining stable machine-readable semantics.

Visual effects such as datamoshing, pixel sorting and RGB split become semantic renderer primitives. fileciteturn63file4L186-L199

## 11. Print & Manufacturing QA

The physical build pipeline validates:

```text
CARD DIMENSIONS
→ SUBSTRATE
→ TRANSPARENCY / HAZE
→ INK / UNDERPRINT
→ REGISTRATION
→ SURFACE FINISH
→ STACK BEHAVIOR
→ DURABILITY
→ ACCESSIBILITY
→ FINAL PLAYER TEST
```

The supplied production material recommends optical-grade PVC around 0.30 mm and stresses transmission/haze constraints for layered stacks. These values remain production hypotheses until measured against the selected supplier/material batch. fileciteturn69file8L32-L60

## 12. Artifact Identity & Provenance

Every manufactured or generated card receives:

```yaml
ArtifactIdentity:
  artifact_id:
  design_version:
  asset_hash:
  ruleset_hash:
  print_batch:
  material_batch:
  manufacturing_date:
  generator_version:
  test_results: []
```

This links the physical object to its digital definition and evaluation record.

## 13. Accessibility Gate

Accessibility is part of the game-system contract:

- color-independent cues;
- text alternatives for visual-only state;
- readable typography;
- sufficient contrast;
- tactile cues that do not compromise game information;
- alternative digital reveal modes;
- clear rules representation.

A visual effect cannot carry essential game semantics if a significant class of players cannot perceive it.

## 14. Human-Agency / Influence Gate

The project's existing Influence Security architecture becomes mandatory for games with AI-driven personalization, dynamic recommendations or social mechanics.

```text
GAME FEATURE
 ↓
INFLUENCE SURFACE ANALYSIS
 ↓
AUTONOMY / TRANSPARENCY / REVERSIBILITY
 ↓
ALLOW / DISCLOSE / REDESIGN / BLOCK
```

The supplied persuasion corpus contains hidden-command, urgency, scarcity and expectation-shaping concepts. These are treated only as threat patterns for defensive review. fileciteturn100file4L5-L25

## 15. Gambling-Safety Gate

The gambling corpus becomes a test harness for probability and integrity, not a template for behavioral exploitation.

Forbidden optimization targets:

`GGR maximization through hidden manipulation`
`compulsive session extension`
`loss-chasing`
`biometric targeting`
`deceptive personalization`

Allowed optimization targets:

`fairness`
`predictability of rules`
`mathematical integrity`
`accessibility`
`strategic depth`
`replayability`
`player agency`

## 16. AI-assisted Playtesting

A multi-agent test team evaluates candidate rulesets:

```text
DESIGN AGENT
     ↓
SIMULATION AGENT
     ↓
BALANCE AGENT
     ↓
ACCESSIBILITY AGENT
     ↓
SAFETY / AGENCY AGENT
     ↓
ADVERSARIAL PLAYER AGENT
     ↓
HUMAN PLAYTEST
```

The system compares disagreement clusters instead of averaging every evaluator into a single opaque score.

## 17. Novelty / Diversity Search

The supplied agent-development material identifies MAP-Elites/Pyribs as suitable for discovering diverse solutions and rare edge cases. fileciteturn62file8L315-L352

Project 50 uses this to search the design space across:

`complexity × interaction density × strategic depth × novelty × accessibility`

while keeping safety constraints hard.

## 18. AI-generated Card Production

The production loop becomes:

```text
PROMPT / DESIGN INTENT
        ↓
CARD SCHEMA
        ↓
IMAGE / VECTOR ASSET GENERATION
        ↓
LAYOUT COMPILER
        ↓
PRINT PREPRESS
        ↓
RULESET LINKING
        ↓
AUTOMATED QA
        ↓
HUMAN ART DIRECTOR REVIEW
```

The image itself is never the sole source of rules truth.

## 19. Release Model

```text
CONCEPT
 ↓
ALPHA RULESET
 ↓
PRINT PROTOTYPE
 ↓
BLIND PLAYTEST
 ↓
BALANCE / SAFETY REVIEW
 ↓
BETA
 ↓
MANUFACTURING CANDIDATE
 ↓
SIGNED RELEASE
```

Every revision increments the ruleset and artifact schema version.

## 20. Portfolio integration

Project 50 extends:

- Project 01 — APEIRON Black System;
- Project 05 — APEIRON Neuroadaptive Interface Safe;
- Project 07 — Agentic Creative Studio;
- Project 22 — Agentic Media Forge;
- Project 26 — Trustworthy Kernel;
- Project 27 — Compound Reasoning;
- Project 28 — Verified Code Generation;
- Project 34 — Venture & Business Model Foundry;
- Project 36 / 43 — Influence Security & Human Agency;
- Project 42 — Open Creator Layer;
- Project 44 — AI Content Product Studio;
- Project 49 — Cognitive Sovereignty & Adaptive Intelligence.

## 21. New integrity integration

The latest stochastic-system corpus adds a dedicated integrity boundary:

```text
RNG VERSION + RULESET HASH + SEED
          ↓
      REPLAY ENGINE
          ↓
 EXPECTED / OBSERVED OUTCOME
          ↓
 DISTRIBUTION / ANOMALY TESTS
          ↓
     EVIDENCE PACKET
```

Presentation, personalization and reward UI must not alter the underlying stochastic state without an explicit ruleset version transition.

## 22. New physical artifact contract

Manufacturing becomes a first-class state machine:

`DESIGN → MATERIAL SAMPLE → PREPRESS → PROTOTYPE → QC → BLIND PLAYTEST → RELEASE BATCH`.

Supplier/material specifications are hypotheses until measurements on production-intent samples confirm thickness, transmission, haze, friction, durability and registration tolerances.

## Definition of Done

1. A typed deck/rules schema exists.
2. Physical and digital states remain synchronized through explicit contracts.
3. Random mechanics are statistically testable and reproducible.
4. Manufacturing constraints are versioned and empirically validated.
5. Accessibility is tested before release.
6. Influence surfaces are audited defensively.
7. Gambling-like mechanics are isolated from compulsion optimization.
8. Generated visual assets are linked to provenance and rules metadata.
9. AI playtesting includes independent adversarial evaluators.
10. Human review remains the final gate for consequential product decisions.
11. RNG/ruleset versions are replayable and independently auditable.
12. Stochastic anomalies are investigated with replicated evidence rather than inferred from a single sample.
13. Physical production claims are backed by batch measurements.

---

## Knowledge evolution — batch 14 / 2026-10-08: AURA completeness and agency

Source: five-page AURA The Collective design. Existing ownership P50/P71 and [source gap](../docs/knowledge-base/2026-10-08-aura-deck-spec-gap-and-safety.md).

**Hard project constraints:** 60 unique IDs; 4 named domains; anomaly quotas Mirror=4, Venom=4, Black Swan=2. Only two domains with values 1–10 plus 10 anomalies are described (30/60 instances). **Other 30 instances and two domain names are not supplied**, so generation of a full product manifest must FAIL rather than hallucinate. Require turn/win rules and replayable RNG before a production deck is declared.

**User agency:** source recommends hidden NLP, urgency and addiction/slot-like reinforcement. Do not implement those as covert coercion or gambling pressure. Only disclosed, voluntary visual/touch mechanics and reversible participation; NFC web/audio requires opt-in, accessibility option and a safe external link.

**Code:** `tools/aura_deck_gate.py` validates a complete manifest without inventing missing cards; 11 local tests on a clearly synthetic fixture, not a manufactured or complete source-derived deck.

---

## 2026-10-09 — batch 19: agent marketplace, MCP and consent-based games

Reference studies Privé, LovePlay and Modern Love demonstrate a market taxonomy: physical/online cards, one-phone turn-taking, claimed two-phone pairing, roleplay/interactive story and challenge-level selection. Treat promotional engagement/benefit/privacy claims as unverified. P50 may reuse neutral rule mechanics for game experiments, with skip/no coercion, safe randomization, no manipulative gamification and independent adults-only gating. Detailed mutual-selection code belongs to **P122 CHEMIA**, not a second adult-game project. No competitor media, game content or accounts were copied.
