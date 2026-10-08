# Project 99 — OmniGame Mobile Product & LiveOps Strategy Engine MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build a governed product-strategy and LiveOps engine for mobile games that converts a game concept into a testable value proposition, business model, competitive position, measurable product strategy, ethical monetization system and continuous experiment program.

## Why this is distinct
- **P86 — OmniGame Text-to-Android Factory** owns conversion from natural-language game idea/scenario into inspectable game specifications, engine projects and reproducible Android artifacts.
- **P97 — Project MIDAS Recursive Multimedia Production Factory** owns recursive multimedia content production, critique, scene planning, rendering and QA.
- **P34/P56/P66/P67** own broader venture commercialization, outcome delivery and business/strategy experimentation.
- **P99** owns the game-specific product-management layer between concept and scalable operation: player/value proposition, market positioning, Business Model Canvas, core-loop economics, LiveOps, KPI design, experimentation and controlled monetization evolution.

## Source-derived product baseline
The supplied `Szept Wyroczni / Whisper of Fate` blueprint combines a simple social-looter loop with voice interaction, AI companion behavior, RNG, social events and mobile IAP. Its central loop is voice ritual → AI/RNG verdict → voiced revelation → progression → PvP/return loop.

P99 treats those mechanics as candidate product hypotheses, not proven market truths.

## Architecture

`market/context → player segments → value proposition → Business Model Canvas → competitive position → product hypothesis → core-loop model → economy/monetization model → LiveOps calendar → experiment compiler → telemetry → analysis → decision → controlled rollout/rollback → strategy update`

## Core modules

### 1. Market & Segment Intelligence
- define player/customer segments;
- identify underserved needs and niches;
- maintain competitor/genre benchmark matrix;
- separate observed market evidence from strategic assumptions.

### 2. Value Proposition Designer
- map player jobs, pains, gains and desired experience;
- generate competing propositions;
- score differentiation, feasibility and evidence requirements;
- require a concrete hypothesis before implementation.

### 3. Business Model Canvas Compiler
Compile and version:
- customer segments;
- value propositions;
- channels;
- customer relationships;
- revenue streams;
- key resources;
- key activities;
- key partnerships;
- cost structure.

The source corpus explicitly frames the business model around value creation, delivery and capture and provides Business Model Canvas as a design/test structure.

### 4. Competitive Strategy Engine
- cost/differentiation/focus hypotheses;
- niche identification;
- resource/capability mapping;
- strategic position tracking;
- blue-ocean-style opportunity exploration;
- competitive-response scenarios.

### 5. Core Loop & Economy Model
Model:
- session loop;
- progression loop;
- social loop;
- event loop;
- resource sinks/sources;
- virtual economy balances;
- reward distributions;
- pricing and offer hypotheses.

Randomness must be transparent and independently auditable where it affects paid outcomes. The engine must not covertly manipulate individual users through emotional state inference.

### 6. LiveOps Compiler
Generate a versioned schedule for:
- events;
- content drops;
- challenges;
- social/co-op activities;
- experiments;
- economy changes;
- notification campaigns;
- rollback windows.

### 7. Experiment Engine
Every material product decision becomes a falsifiable experiment where feasible:

`hypothesis → baseline → treatment/control → metric → sample/window → result → confidence/uncertainty → decision → learning record`

Supported decisions:
- KILL;
- ITERATE;
- HOLD;
- SCALE;
- ROLLBACK.

This operationalizes the supplied Obłój principle of ambitious goals combined with cost discipline, experimentation, customer focus and learning from failure.

### 8. Monetization Governance
Supported candidate models:
- cosmetic IAP;
- convenience purchases;
- battle/event passes;
- subscriptions with stable, disclosed benefits;
- advertising/rewarded ads;
- bundles and time-limited offers.

Disallowed design requirements:
- conditioning basic social approval on payment;
- covert emotion-based price discrimination;
- deliberate near-miss manipulation targeted from inferred emotional state;
- hidden microphone monitoring;
- coercive obstruction of exit;
- deceptive scarcity or fabricated claims.

### 9. KPI & Performance Layer
Track product and business indicators such as:
- acquisition;
- activation;
- D1/D7/D30 retention;
- session frequency/duration;
- progression completion;
- payer conversion;
- ARPDAU/ARPPU where applicable;
- churn;
- event participation;
- crash/error rate;
- content-production cost;
- contribution margin.

Metrics are observations. They do not automatically establish causality.

### 10. Strategy Memory & Learning Ledger
Persist:
- assumptions;
- experiments;
- results;
- failed approaches;
- successful variants;
- market changes;
- strategy versions;
- business-model versions;
- rationale for decisions.

A failed experiment is retained as learning rather than silently erased.

## Iteration 24 — Business-Model & Strategy Extensions

The new corpus strengthens P99 with a more explicit distinction and coupling between **business model, strategy, resources/competences, operations and measurable outcomes**.

### A. Business model as an integrated system
The corpus describes business models through interconnected components covering value offered, target market, product scope, pricing, revenue sources, required activities, organizational capabilities and durability of competitive advantage. The model is therefore represented as a dynamic system rather than a static canvas.

P99 consequence: every Business Model Canvas version must have an explicit dependency graph linking its elements and a change history showing how market or capability changes affect the model.

### B. Strategy vs business model
The corpus contains several scholarly interpretations, but consistently treats the two concepts as closely related rather than identical. Drzewiecki emphasizes different levels of generality, element sets and intended use; both can support strategic planning and both can be iterative. The implementation model should therefore preserve two separate versioned objects with typed links rather than collapsing them into one record.

### C. Resource and competence layer
Key organizational competences are treated as strategic assets that can be rare, difficult to substitute and difficult to imitate. P99 therefore adds a capability map between product hypotheses and execution: each proposed game feature or LiveOps capability should identify required capabilities, bottlenecks, dependencies and whether the capability is actually evidenced.

### D. Adaptive strategy modes
The corpus includes classical, adaptive, shaping, visionary and survival approaches depending on environmental predictability, organizational influence and resource constraints. P99 may use these as strategy-mode hypotheses for LiveOps and product planning, but must not infer that a mode is correct without evidence about the operating environment.

### E. Real + virtual + hybrid market strategy
The marketing corpus extends the Ansoff matrix to virtual products and virtual markets and emphasizes interaction between traditional, digital and social channels. P99 therefore supports real, digital and hybrid channel/product configurations and tracks channel-specific evidence rather than treating online activity as a separate universe.

### F. Performance measurement discipline
The corpus warns against overreliance on short-term sales and financial results while neglecting customer satisfaction, loyalty, operational service quality and customer/brand/project value. P99 consequently separates financial KPIs from customer, product, operational and capability indicators and requires multi-horizon evaluation.

### G. Strategy-to-experiment loop
Drzewiecki explicitly describes strategy and business-model construction as iterative processes involving identification/creation, description, implementation, verification and improvement. P99 now treats this as a first-class lifecycle rather than an optional workflow:

`CREATE → DESCRIBE → IMPLEMENT → VERIFY → IMPROVE → VERSION`

### H. New governance invariant
`business-model coherence != business-model validity`

A logically coherent model can still fail in the market. Coherence is tested internally; validity requires external or measured evidence.

## Integration with portfolio
- **P24** — structured intent/prompt compilation.
- **P27** — compound reasoning and disagreement-driven escalation.
- **P34/P56** — commercialization and outcome delivery.
- **P61/P80** — runtime/orchestration and self-evolving substrate patterns.
- **P66/P67** — business-model and strategy experimentation.
- **P72** — assurance, postcondition verification and policy gates.
- **P86** — downstream game build/delivery factory.
- **P90** — SOP/process compilation.
- **P91** — persona/style controls where useful and authorized.
- **P97** — multimedia production for trailers, assets and campaign content.

## Governance invariants
- strategy ≠ business model ≠ operations;
- product hypothesis ≠ verified market outcome;
- telemetry ≠ causal proof without an appropriate design;
- model capability ≠ authorization;
- personalization ≠ permission to manipulate;
- simulation ≠ real-world evidence;
- paid randomness requires auditable rules;
- stale telemetry cannot overwrite newer product state;
- consequential releases require authorization and postcondition verification;
- business-model coherence ≠ business-model validity;
- capability/resource assumptions must remain distinct from observed capability evidence.

## Verification program
1. **Market hypothesis tests** — validate segment and value-proposition assumptions.
2. **Core-loop tests** — session-level usability and engagement baselines.
3. **Economy simulations** — sink/source balance, progression and inflation tests.
4. **A/B experiments** — controlled product changes with pre-registered metrics.
5. **Monetization safety tests** — verify disclosed pricing, stable benefits and absence of coercive mechanics.
6. **LiveOps replay** — replay event schedules against historical telemetry.
7. **Rollback tests** — validate rapid reversal of harmful economy/content changes.
8. **Cost tests** — compare content, acquisition and infrastructure costs with contribution outcomes.
9. **Cross-device QA** — integrate with P86 build artifacts.
10. **Evidence ledger audit** — verify that strategic claims can be traced to source evidence or measured outcomes.
11. **Capability-fit tests** — verify that required organizational/technical competences exist or have an explicit acquisition plan.
12. **Multi-horizon KPI review** — compare short-term financial outcomes with retention, satisfaction, operational quality and strategic capability indicators.

## Maturity
Architecture-ready concept. No claim of global hit status, specific revenue, retention, addiction, conversion uplift or market dominance is verified.

## Derived from
P34, P56, P66, P67, P72, P86, P90, P97.

## Primary source witnesses
- `Projekt gry mobilnej hitem 2026.PDF`
- `Modele biznesowe.pdf`
- `StrategiaamodelbiznesuAdamWiniewskiMagdalenaRaczyska.pdf`
- `Strategia-sukcesu-firmy-wyklad.pdf`
- `publikacja_PODR_8.pdf`
- `Niewiadomski_Konkretyzacja_3_4_2021.pdf`
- `Kachniewska_Magdalena_Innowacyjne_modele_biznesu_jako_strategia_ przetrwania_niezaleznych_obiektów_hotelowych_w_warunkach_polaryzacji_rynku_turystycznego.pdf`
- `model-biznesowy-a-strategia-i-zarzadzanie-strategiczne-16stgybpjs.pdf`
- `Stawka-większa-niż-biznes-Peter-MAY-Adrianna-LEWANDOWSKA.pdf`
- `JanPolowczykSkutecznestrategiebiznesowe2008.pdf`
- `Drzewiecki_Model_Biznesu_a_Strategia_Organizacji_P_251102_065553.pdf`
- `BCPS_30987_-_Model-biznesowy---ws_0000.pdf`
- `art_34-39_Siwak.pdf`
- `20-Marcin-Kardas-Pojęcia-i-typy-modeli-biznesu-Klincewicz-Krzysztof-red-Zarzadzanie-organizacje-i-organizowanie.pdf`
- `09.pdf`
- `adminojs,+Administrator+czasopisma,+PEFIM_2015_n63_s31.pdf`
- `BAŚŃ O PORNLANDI.pdf`
- `CCR TOM I Jak przetrwać w dziwnym świecie po przeniesieniu się do alternatywnej osi czasu.pdf`
- `CCR TOM II Oś Czasu w Alternatywnych światach.pdf`

---

## Knowledge evolution — batch 16 / 2026-10-08

The 26-page IBM Cloud backend document proposes Code Engine/Cloudant/Watsonx and server-owned player state with _rev optimistic concurrency. P99 retains LiveOps/market pricing; P126 owns concrete IBM game-state API and NPC backend. Add TrustedRewardReceipt{player_id,event_id,authorized_intent,old_revision,new_revision,committed_result,source}. Client-side balances and model NPC answers cannot mint credits. Cloudant 409 conflict requires reload/re-evaluate rather than blind write. No live IBM integration verified.
