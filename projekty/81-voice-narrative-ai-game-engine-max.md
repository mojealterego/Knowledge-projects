# Project 81 — Voice-Narrative AI Game Engine MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-09

## Mission

Build a mobile/edge game runtime in which a player interacts primarily through voice with an AI character, while the authoritative game rules, rewards, economy and progression remain deterministic, inspectable and independently verifiable.

The project derives from the supplied `Szept Wyroczni / Whisper of Fate / Project Midas` corpus. The source's distinctive technical idea is retained — voice-first interaction with an AI Oracle — while its proposed covert manipulation, emotional monetization and compulsion mechanics are explicitly rejected as production requirements.

## Why this is a new project

The portfolio already contains:

- Project 27 — compound reasoning and self-consistency;
- Project 50 — physical/phygital game foundry;
- Project 51 — stochastic/game integrity;
- Project 52 — adaptive game design and playtesting;
- Project 57 — multimodal/cognitive interface;
- Project 71 — physical artifact manufacturing;
- Projects 61/68/72 — OmniCore, constitutional runtime and assurance;
- Projects 73/76 — narrative and sensory game lineages.

None owns the complete **voice-native AI-character game runtime** in which natural-language interaction drives a branching narrative while an independent authoritative state machine governs gameplay and economy.

## Core architecture

```text
PLAYER VOICE
     ↓
LOCAL / MINIMIZED AUDIO PROCESSING
     ↓
SPEECH-TO-INTENT
     ↓
INTENT VALIDATION + CONSENT / PRIVACY POLICY
     ↓
NARRATIVE ORACLE
     ↓
STRUCTURED GAME COMMAND
     ↓
AUTHORITATIVE GAME STATE MACHINE
     ├── rules
     ├── RNG / seeds
     ├── progression
     ├── inventory
     └── economy
     ↓
POSTCONDITION / INTEGRITY CHECK
     ↓
VOICE + VISUAL RESPONSE
     ↓
PLAYER
```

## 1. Dual-plane state model

The central architectural invariant is:

```text
AI / NARRATIVE STATE
        ≠
AUTHORITATIVE GAME STATE
```

The AI may propose an action such as `ATTEMPT_DRAW`, `CHOOSE_PATH`, `ACCEPT_QUEST` or `START_RITUAL`. It cannot directly mutate coins, inventory, purchases, win probability or progression.

Example:

```yaml
NarrativeIntent:
  intent_id:
  transcript_ref:
  normalized_intent:
  confidence:
  character_context:
  consent_state:

GameCommand:
  command_id:
  type:
  parameters: {}
  state_version:
  authorization:
  ruleset_version:
```

The game engine validates the command against the current authoritative state before execution.

## 2. Voice interaction layer

Supported interaction modes may include:

- spoken wishes/questions;
- character dialogue;
- voice navigation;
- contextual commands;
- accessibility controls;
- optional wake-word or push-to-talk interaction.

The system must make recording and processing state visible. Continuous hidden microphone monitoring is not a project requirement.

## 3. Oracle character engine

The Oracle is a fictional AI character, not an authority over the player.

It maintains an explicit character state:

```yaml
OracleState:
  persona_version:
  relationship_state:
  narrative_memory_refs: []
  current_scene:
  emotional_expression:
  allowed_topics: []
  safety_constraints: []
```

Character state may change tone, dialogue, available story branches and cosmetic presentation. It must not covertly control purchases, basic access, dignity or material rewards.

## 4. Intent compiler

Voice input is compiled into a typed intermediate representation:

```text
AUDIO
 ↓
TRANSCRIPT / SEMANTIC INTENT
 ↓
ENTITY + ACTION EXTRACTION
 ↓
CONSTRAINT CHECK
 ↓
COMMAND IR
 ↓
RULE VALIDATION
 ↓
EXECUTION
```

Ambiguous voice commands fail safely or request clarification rather than guessing a consequential action.

## 5. Narrative graph

The story is represented as a versioned directed graph:

```yaml
StoryNode:
  id:
  prerequisites: []
  dialogue:
  choices: []
  world_state_requirements: []
  effects: []
  provenance:
  version:
```

AI generation can propose dialogue and candidate branches, but released narrative content passes content, safety and continuity validation.

## 6. Authoritative game-state engine

Gameplay uses deterministic rules:

```yaml
GameState:
  version:
  player_id:
  scene_id:
  resources: {}
  inventory: {}
  progression: {}
  rng_seed:
  economy_version:
  ruleset_version:
```

State transitions are append-only in the event log and checked for version monotonicity.

```text
STATE_t
  ↓ command
VALIDATION
  ↓
STATE_{t+1}
  ↓
POSTCONDITION
  ↓
EVENT LOG
```

## 7. Fair randomness

If the game uses random outcomes, the RNG subsystem is isolated from the language model.

```text
RULESET HASH + RNG VERSION + SEED
                 ↓
          DETERMINISTIC DRAW
                 ↓
           OUTCOME RECORD
                 ↓
            REPLAY TEST
```

The Oracle cannot alter odds because of tone, spending behavior, emotional state or inferred vulnerability.

## 8. AI companion without coercive monetization

The source proposes a subscription that changes the Oracle's emotional behavior and a reward system intended to exploit near-miss dynamics. Those mechanisms are excluded.

Allowed model:

```text
FREE / PAID CONTENT
       ↓
CLEAR VALUE PROPOSITION
       ↓
COSMETIC / NARRATIVE / OPTIONAL CONTENT
       ↓
NO HIDDEN ODDS MODIFICATION
       ↓
NO EMOTIONAL PUNISHMENT FOR NON-PAYMENT
```

Purchases are handled by the authoritative economy layer, independently from the language model.

## 9. Privacy / edge-first voice processing

The supplied safer version proposes local voice processing and sending only a minimized intent to the server. Project 81 adopts this as an architecture preference where technically feasible.

```text
MICROPHONE
   ↓
LOCAL ASR / INTENT EXTRACTION
   ↓
MINIMIZED INTENT
   ↓
SERVER / GAME BACKEND
```

Retention, deletion and consent policies are explicit. Raw audio is not required for ordinary gameplay telemetry.

## 10. Audio and multimodal presentation

The system can combine:

- voice synthesis;
- ambient audio;
- reactive UI;
- character animation;
- optional haptics;
- visual state changes.

Audio steganography from the source is treated as an optional puzzle/narrative mechanic only when disclosed as part of the game experience and when it does not become covert behavioral control.

## 11. Verification / assurance

Project 72 supplies the runtime assurance boundary.

Every consequential game transition should be traceable through:

```text
VOICE INPUT
 ↓
INTENT
 ↓
COMMAND
 ↓
AUTHORIZATION / RULE CHECK
 ↓
STATE TRANSITION
 ↓
POSTCONDITION
 ↓
PLAYER-FACING RESULT
```

The AI's self-report is never the authoritative proof that a reward, purchase or state change occurred.

## 12. Adversarial testing

Test suites include:

- ambiguous speech;
- prompt injection through player dialogue;
- malicious or adversarial game content;
- speech-to-command confusion;
- replay of stale commands;
- duplicate purchase events;
- economy desynchronization;
- model hallucinated game state;
- attempts to persuade the model to bypass rules;
- multilingual and code-switched inputs;
- privacy-boundary violations.

## 13. Accessibility

Voice is an additional interaction channel, not the only path.

Provide:

- text interaction;
- captions;
- visual command confirmation;
- remappable controls;
- adjustable speech rate;
- non-voice play mode;
- clear indication of when speech is being processed.

## 14. Evaluation framework

Compare at least:

| Dimension | Metric family |
|---|---|
| intent accuracy | command classification / correction rate |
| narrative quality | human + rubric evaluation |
| state integrity | invariant violations / replay failures |
| fairness | RNG distribution / economy audits |
| latency | p50 / p95 end-to-end voice response |
| privacy | raw-audio retention / egress surface |
| safety | adversarial scenario pass rate |
| player agency | coercion / dark-pattern review |
| retention | voluntary return, not compulsive-use optimization |

The project must not optimize for addiction or vulnerability exploitation.

## 15. Technical implementation target

A first production-oriented stack can use:

```text
MOBILE CLIENT
├── microphone / speech layer
├── local intent extraction where feasible
├── presentation / haptics
└── offline-safe UI

GAME RUNTIME
├── authoritative state machine
├── RNG / replay
├── narrative graph
├── economy
└── telemetry

AI LAYER
├── character model
├── dialogue generation
├── intent compiler
├── memory retrieval
└── safety / content filters

ASSURANCE
├── event normalization
├── policy checks
├── postconditions
├── replay
└── audit
```

## 16. Relation to Project 27

Project 27 can provide compound inference for difficult intent interpretation, character planning or content evaluation.

The boundary remains:

```text
COMPOUND AI REASONING
        ↓
PROPOSED COMMAND / CONTENT
        ↓
AUTHORITATIVE VALIDATOR
        ↓
GAME STATE
```

Consensus among multiple models does not authorize a game-state mutation.

## 17. Relation to Project 50 / 51 / 52 / 57 / 71

- Project 50 provides the physical/phygital game design and safety architecture.
- Project 51 provides stochastic integrity and replayable probability testing.
- Project 52 provides adaptive game design, simulation and playtesting.
- Project 57 provides multimodal/cognitive interface patterns.
- Project 71 provides physical artifact and symbolic-interface manufacturing where a future hybrid edition is desired.

## 18. MVP

The first credible MVP is deliberately small:

1. one AI Oracle character;
2. one narrative chapter;
3. speech-to-intent;
4. ten typed game commands;
5. authoritative state machine;
6. deterministic replayable RNG;
7. voice response;
8. explicit microphone status;
9. purchase sandbox with no model-controlled odds;
10. Project 72-style event/audit record;
11. adversarial command suite;
12. human playtest.

## 19. Definition of done

Project 81 reaches its first validated milestone when:

- voice commands compile into typed intents;
- ambiguous consequential commands fail safely;
- AI-generated content cannot directly mutate authoritative state;
- game rules and RNG are deterministic/replayable under fixed seeds;
- economy and purchase events are independently verified;
- privacy boundaries are measurable;
- narrative continuity survives session restoration;
- adversarial tests cover model/game-state confusion;
- accessibility supports non-voice play;
- no hidden emotional or financial manipulation is required for engagement;
- telemetry distinguishes AI proposals from authoritative outcomes.

## Evidence classification

**SOURCE_DERIVED:** The supplied Szept Wyroczni documents describe voice-first interaction, an AI Oracle, NLP, Unity mobile implementation, hidden commands, audio concepts and monetization structures.

**INFERRED:** The strongest reusable engineering opportunity is a voice-native AI-character game architecture with a strict separation between probabilistic language behavior and authoritative game state.

**PROPOSED:** Project 81 is a new portfolio project because this complete digital voice-narrative runtime is not owned by an existing project.

**NOT ESTABLISHED:** The source documents do not establish that covert NLP commands, emotional monetization or compulsive design reliably create long-term retention, nor that such mechanisms are legally or ethically acceptable.

---

## Integracja wiedzy — 2026-10-08: walidacja zasobów syntezy mowy

**Status:** SPECIFIED / NO AUDIO ASSET IMPORTED; źródło: `voice_preview_ian — polish narrator (warm_deep).mp3.html`.
**Specyfikacja:** [P81 TTS artifact gate](81-voice-narrative-ai-game-engine-max-2026-10-08-tts-artifact-gate.md); [ustalenie formatu pliku](../docs/knowledge-base/2026-10-08-voice-preview-html-validation.md).

### Rozbudowa warstwy voice response
1. `AudioTypeValidator` sprawdza sygnaturę danych i możliwość dekodowania, nie rozszerzenie. Plik kończący się `.mp3.html` pozostaje w klasie HTML; nie jest transkodowany w oparciu o nazwę.
2. `VoiceRightsRegistry` wersjonuje dostawcę, `voice_id`, model, licencję, zgodę oraz zakres dopuszczonego wykorzystania głosu. Nie upublicznia sekretów API.
3. `SpeechQualityEvaluator` definiuje zestaw polskich próbek, wymowę imion i liczb, pauzy, zrozumiałość, opóźnienie, głośność i jakość na różnych urządzeniach.
4. `AccessibilityFallback` udostępnia napisy, możliwość wyłączenia dźwięku i sterowanie tekstowe.
5. `NarrationGameBoundary`: wynik TTS/ASR ani dowolny tekst narratora nie mutuje samodzielnie autorytatywnego stanu gry, RNG ani ekonomii.

### Kryteria akceptacyjne
Negatywny test: przesłany HTML nie zostaje zaakceptowany jako audio. Pozytywny test wymaga prawdziwych zdekodowanych danych audio wraz z metadanymi i prawami użycia. Przed wdrożeniem konieczny jest pomiar jakości polskiego głosu na rzeczywistym nagraniu.
