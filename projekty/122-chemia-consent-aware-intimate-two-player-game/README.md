# P122 — CHEMIA
## Consent-Aware Intimate Two-Player Game

**Status:** Architecture / MVP specification  
**Platform:** Android  
**Target:** 2 players, offline-first, one phone initially; two-phone mode later  
**Stack:** Kotlin + Jetpack Compose + Room

---

## 1. Product thesis

CHEMIA is an intimate two-player game built as a persistent adaptive game system, not a static Truth-or-Dare generator.

Core loop:

`LOSuj → WYKONAJ → WYBIERZ REAKCJĘ → ZWIĘKSZ HEAT → ODBLOKUJ WARSTWĘ → LOSUJ PONOWNIE`

There is no automatic end state caused by reaching a goal. Players explicitly end the session.

---

## 2. Eight-category deck

| Code | Category | Target |
|---|---|---:|
| CONNECTION | closeness, questions, memories | 60 |
| TEASE | flirt, tension, choices | 70 |
| HEAT | bold shared challenges | 100 |
| ROLEPLAY | role-based scenarios | 70 |
| DESIRE | preferences, secrets, fantasies | 60 |
| CONTROL | partner choice / safe variants | 50 |
| CHAOS | rules changing future rounds | 50 |
| AFTERGLOW | calmer continuation | 60 |

**Total:** 520 target cards.

---

## 3. Intensity and Heat

Intensity:

`SOFT → SPICY → HOT → EXTREME`

Heat:

`0–24 Warm-up → 25–49 Spicy → 50–74 Hot → 75–99 Extreme → 100 Afterglow / Endless`

Heat is a game-state variable, not consent. Reaching a higher Heat value never authorizes an activity that a player has not accepted.

---

## 4. Consent architecture

Per category, each player independently chooses:

`TAK / MOŻE / NIE`

Global intensity can be changed at any time.

`POMIŃ`:

- has no score/Heat penalty;
- requires no explanation;
- immediately removes the card from the current decision path.

Production app should enforce an explicit adult/18+ boundary because the product is designed around intimate content.

---

## 5. Engine architecture

```text
GameState
   ↓
ConsentFilter
   ↓
CardSelector
   ↓
HeatEngine
   ↓
ChainEngine
   ↓
EventEngine
   ↓
NextCard
```

The card/content layer is independent from mechanics so new decks and packs can be introduced without rewriting the game engine.

---

## 6. Card data contract

```text
id
category
intensity
type
text
duration
heat
chain
requires_both
requires_choice
afterglow
tags[]
min_players
max_players
```

The contract should remain extensible for deck versioning, localization, content provenance and future modifiers.

---

## 7. Adaptive weighted selector

The selector must not be `random()` alone.

Candidate score is conceptually:

`category_preference × intensity_match × heat_multiplier × chain_bonus × novelty_bonus × recent_card_penalty × skipped_tag_penalty`

Inputs include:

- consent state
- intensity
- previous cards
- skipped cards/tags
- favorite categories
- Heat
- Chain level
- session duration
- Afterglow state
- player balance

**Critical boundary:** selection score is not permission. Consent filtering is authoritative for card eligibility.

### Repetition controls

- used card → block for the latest 20-card window;
- skipped card → temporary tag suppression;
- dominant category → progressively favor adjacent eligible categories;
- selector decisions should support seeded deterministic replay for testing/debugging.

---

## 8. Chain Engine

```text
CHAIN ×1
   ↓
CHAIN ×2
   ↓
CHAIN ×3
   ↓
CHAIN ×4
   ↓
BONUS
```

A chain can:

- change category;
- raise Heat;
- lower repeat probability;
- transfer choice to the other player;
- continue the round without returning to the normal loop.

After the chain ends, normal selection resumes.

---

## 9. Partner Choice

Cards may expose:

`CHOICE_A / CHOICE_B / CHOICE_C`

The designated partner selects one eligible variant. The system records the game-state consequence, not a hidden individual preference disclosure.

---

## 10. Secret Mission Engine

At session start:

`PLAYER A → PRIVATE MISSION`  
`PLAYER B → PRIVATE MISSION`

Example missions:

- cause three category changes;
- make partner choose `JESZCZE JEDNA` twice;
- collect three sincere answers;
- use all three choice opportunities;
- reach a specified Heat value.

Mission completion:

`SECRET COMPLETE → BONUS CARD → +15 HEAT`

Missions must remain gameplay/communication goals and must not depend on forcing a particular sexual activity.

---

## 11. “Jeszcze jedna”

After each card:

`🔥 JESZCZE JEDNA | 🎲 LOSUJ | ❤️ ZMIEŃ KLIMAT`

If both choose `JESZCZE JEDNA`:

`Heat += 10`

The selector then increases the probability of stronger **eligible** categories. The increase never bypasses consent or intensity constraints.

---

## 12. Afterglow and Endless

At Heat 100:

`AFTERGLOW UNLOCKED`

Three paths:

- `SLOW`
- `CLOSE`
- `RANDOM`

After 10 Afterglow cards:

`TAK / LOSOWAĆ / ZAKOŃCZ`

Endless mode keeps the session alive without automatic GAME OVER.

---

## 13. Adaptive private state

Local state includes:

- category frequency;
- skipped cards;
- Heat history;
- favorite task types;
- session duration;
- Chain frequency;
- per-player preferences.

Privacy rule:

`PLAYER_PRIVATE_STATE ≠ SHARED_GAME_STATE`

The other player must not receive the individual preference profile of their partner.

---

## 14. Two-phone architecture

Future model:

`PHONE A ↔ PHONE B`

Private:

- secret cards;
- secret missions;
- own answers;
- individual preferences.

Shared:

- Heat;
- board/session state;
- current card;
- result;
- Chain progress.

Bluetooth / Wi-Fi Direct is a later transport layer; the shared-state protocol must preserve private/public field separation and conflict-safe updates.

---

## 15. Modes

| Mode | Duration | Mechanics |
|---|---:|---|
| QUICK | 15 min | fast session |
| SPICY | 30 min | full Heat |
| HOT | 60 min | Chain + Secret Mission |
| EXTREME | 90 min | all categories/mechanics |
| ENDLESS | unlimited | no automatic time limit |

Time is a session target, not a forced termination mechanism.

---

## 16. MVP / v1.0

- 8 categories;
- 4 intensity levels;
- 520 target base cards;
- 100+ Chain cards;
- 50 Secret Missions;
- Heat 0–100;
- Afterglow;
- Endless Mode;
- two-player offline one-phone mode;
- private cards/missions;
- no-penalty skip;
- session history.

Base content can be expanded combinatorially with:

`BASE CARD + ROLE + INTENSITY + TIME + CHOICE + CHAIN`

rather than manufacturing hundreds of near-duplicates.

---

## 17. Android implementation boundary

Initial stack:

- Kotlin;
- Jetpack Compose;
- Room;
- encrypted local preferences;
- no mandatory account.

Later:

- Bluetooth;
- Wi-Fi Direct;
- Internet session mode.

Suggested module boundaries:

```text
:app
:game-core
:game-content
:game-consent
:game-selection
:game-heat
:game-chain
:game-missions
:game-sync
:persistence
```

The core engine should be platform-independent enough to permit deterministic simulation tests without UI dependencies.

---

## 18. Verification contract

Minimum invariants:

- `NIE` category cards are never selected;
- active intensity constraints are never exceeded;
- Skip has no penalty;
- private missions/cards do not enter shared payloads;
- Heat is clamped to 0–100;
- Chain cannot advance beyond its configured limit;
- Afterglow does not reset the session;
- the latest 20-card repetition window is respected;
- skipped-tag suppression follows explicit expiry policy;
- seeded replay reproduces selector decisions;
- persisted sessions restore without private-state leakage.

Property-based testing is recommended for the selector and state machine because many failures will occur in combinations of Heat, consent, Chain, skips and Afterglow transitions.

---

## 19. Security / privacy boundary

No mandatory cloud account is required for MVP.

Local persistence should separate:

`GAME STATE / CONTENT / PLAYER A PRIVATE STATE / PLAYER B PRIVATE STATE / SESSION HISTORY`

Any future synchronization layer must treat transport as untrusted and validate state transitions before applying them.

---

## 20. Product differentiation

CHEMIA differs from a simple card generator through the combination of:

1. consent-aware eligibility;
2. adaptive weighted selection;
3. persistent Heat progression;
4. Chain state machine;
5. private Secret Missions;
6. partner-controlled choices;
7. Afterglow continuation;
8. Endless Mode;
9. private preference adaptation;
10. one-phone and future two-phone privacy-aware play.

---

## 21. Portfolio lineage

**New project P122.** No existing CHEMIA-specific project was found in the repository search.

The project may consume shared portfolio primitives from existing systems, but its game-domain state machine, content system, consent model and two-player privacy boundary remain project-specific.

Core loop:

`SESSION → CONSENT → SELECT → PLAY → HEAT → CHAIN/CHOICE/MISSION → AFTERGLOW → CONTINUE OR PLAYER-INITIATED END`

Core invariants:

`HEAT ≠ CONSENT`  
`RANDOM ≠ PERMISSION`  
`SKIP ≠ FAILURE`  
`PRIVATE ≠ SHARED`  
`INTENSITY ≠ GRAPHIC CONTENT`  
`GAME STATE ≠ REAL-WORLD AUTHORITY`

---

## 2026-10-09 — batch 19: agent marketplace, MCP and consent-based games

**Competitive sources:** Privé (two separate responses, only mutually positive topics exposed), LovePlay (browser games and intensity/skip controls; conflicting one-device vs two-device descriptions), Modern Love (commercial editorial, not independent clinical data). Existing CHEMIA already owns this product. Privacy-first match phase is an incremental mechanism, not P127.

**Actual code:** `consent_intersection.py` calculates only the intersection of two ephemeral immutable topic sets and subtracts either player's blocked topics; denies mismatched sessions, inactive/revoked partner or missing adult self-attestation. `tools/test_chemia_consent_intersection.py` contains 12 pure-Python tests. **Not implemented:** an age verification service, storage, end-to-end encryption, online pairing, Android UI or real player accounts. `adult_self_attested` is only a declared boolean — no assurance of actual age. Do not infer real-world consent from matching activity; **MATCH ≠ CONSENT, HEAT ≠ CONSENT, SKIP ≠ FAILURE**. No individual answers should be logged, sent to analytics, exposed to partners or inserted in GitHub.
