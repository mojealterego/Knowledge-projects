# 36 — Influence Security & Human Agency Defense Lab MAX

## Status
Architecture-ready defensive research project.

## Cel projektu

Zbudować warstwę **Influence Security**, która wykrywa i ogranicza próby niejawnego sterowania zachowaniem człowieka przez system AI, interfejs, komunikację lub algorytm personalizacji.

Źródła dotyczące bio-hackingu opisują mechanizmy takie jak reward prediction error, zmienne harmonogramy wzmocnień i wykorzystywanie niepewności do podtrzymywania zachowania. fileciteturn52file13L632-L655 Materiały mentalistyczne pokazują z kolei, jak sugestia, framing, performance i oczekiwania mogą zmieniać odbiór eksperymentu. fileciteturn53file0L10-L29 Materiał o subliminal force explicite opisuje łączenie NLP, hipnozy, manipulacji werbalnej i mentalizmu. fileciteturn53file2L139-L161

Projekt wykorzystuje te materiały wyłącznie jako **taksonomię zagrożeń i źródło sygnałów detekcyjnych**. Nie implementuje covert persuasion, uzależniania ani niejawnego sterowania decyzjami.

## Główna zasada

```text
PERSONALIZATION ≠ PERMISSION TO MANIPULATE
ENGAGEMENT ≠ USER BENEFIT
PREDICTION ≠ AUTHORITY
INFLUENCE ≠ CONSENT
```

## Architektura

```text
USER / CONTEXT / CONTENT
          |
          v
BEHAVIORAL SIGNAL EXTRACTION
          |
          +--> framing
          +--> urgency
          +--> repetition
          +--> reward variability
          +--> dependency signals
          +--> authority cues
          +--> contradiction / pressure
          +--> covert suggestion patterns
          +--> expectation shaping
          +--> attention capture
          |
          v
INFLUENCE RISK ENGINE
          |
          +--> transparency
          +--> autonomy
          +--> vulnerability
          +--> reversibility
          +--> proportionality
          |
          v
POLICY DECISION
   ├─ allow
   ├─ disclose
   ├─ neutralize
   ├─ reduce personalization
   ├─ require consent
   └─ block / escalate
```

## 1. Influence threat taxonomy

### 1.1 Reward-loop risk

The supplied bio-hacking report frames variable-ratio reinforcement as a mechanism capable of sustaining compulsive checking and behavior. fileciteturn52file16L782-L802

The defensive engine therefore detects product patterns such as:

- unpredictable rewards used to increase compulsive engagement;
- intentionally incomplete notifications designed to provoke repeated checking;
- escalating reward schedules tied to prolonged use;
- emotionally contingent access to important functionality;
- engagement optimization that knowingly conflicts with user welfare.

Detection is based on system behavior and configuration, not on a claim about a user's psychological state.

## 2. Autonomy budget

Every agent interaction can consume an **Influence Budget**:

```yaml
InfluenceBudget:
  persuasion_intensity: 0..1
  personalization_depth: 0..1
  repetition: 0..1
  urgency: 0..1
  emotional_targeting: 0..1
  reversibility: 0..1
  disclosure_level: 0..1
```

High personalization plus low transparency plus low reversibility is treated as a high-risk combination.

The budget is a governance mechanism, not a numerical claim that human influence can be measured perfectly.

## 3. Covert-influence detector

Mentalism materials repeatedly emphasize that convincing effects depend on performance, expectation, distraction and framing. fileciteturn53file13L834-L853

For AI systems, the defensive analogue is detection of:

```text
HIDDEN OBJECTIVE
      +
MISLEADING FRAMING
      +
ASYMMETRIC INFORMATION
      +
PRESSURE / URGENCY
      +
REDUCED ABILITY TO OPT OUT
      ↓
INFLUENCE RISK
```

The detector should flag the **system design**, not label the user as gullible or susceptible.

### 3.1 Suggestion / expectancy analysis

The newly supplied performance material demonstrates a recurring structure: direct attention toward a desired mental representation, constrain when the participant begins forming it, reinforce vivid visualization, and use timing/performance to shape expectations. fileciteturn76file0L19-L45 The companion material also combines verbal instructions with gestures and emphasized words. fileciteturn77file15L805-L818

The defensive system therefore adds a **Suggestion Exposure Vector**:

```yaml
SuggestionExposure:
  imperative_density:
  sequencing_pressure:
  attention_direction:
  repetition:
  expectancy_cues:
  multimodal_reinforcement:
  opt_out_friction:
  disclosure:
```

High scores do not prove manipulation. They trigger transparency, review or neutralization according to policy.

## 4. Deception-aware evidence model

The supplied deception material proposes looking for inconsistencies across verbal content, timing, gestures and story structure. fileciteturn53file14L864-L898 However, such cues are not reliable enough to serve as an autonomous truth oracle.

The project therefore uses:

```text
OBSERVATION
   +
SOURCE QUALITY
   +
INTERNAL CONSISTENCY
   +
EXTERNAL EVIDENCE
   +
ALTERNATIVE EXPLANATIONS
   →
CONFIDENCE
```

No single body-language or linguistic cue can trigger an accusation, account action or irreversible decision.

## 5. Conversational pressure monitor

The engine detects interaction patterns including:

- repeated requests after a clear refusal;
- artificial scarcity or urgency;
- authority claims without verifiable provenance;
- attempts to isolate the user from independent verification;
- framing a requested action as already authorized when it is not;
- emotional escalation immediately before a consequential action;
- repeated normalization of increasingly risky requests;
- instructions to hide the interaction from oversight;
- covert suggestion or expectancy shaping without disclosure.

These signals feed policy review and user-protection controls.

## 6. Human-in-the-loop protections

For high-impact decisions:

```text
AGENT RECOMMENDATION
        ↓
RATIONALE / EVIDENCE
        ↓
USER-CONTROLLED REVIEW
        ↓
EXPLICIT CONFIRMATION
        ↓
EXECUTION
```

The system must not manufacture consent through interface timing, emotional pressure or hidden defaults.

## 7. Anti-dark-pattern UI contract

Every consequential UI flow should expose:

- what the system wants the user to do;
- why the recommendation is being made;
- what data influenced it;
- whether personalization is active;
- alternatives;
- cancellation / opt-out;
- reversibility;
- material uncertainty.

Default choices should not be engineered solely for engagement maximization.

## 8. Neurobiological safety boundary

The bio-hacking source describes attempts to move below conscious reasoning through reward, attachment and biological vulnerability. fileciteturn50file4L8-L27

The project translates this into prohibitions on agent behavior:

```text
NO TARGETING OF VULNERABILITY
NO ENGINEERED DEPENDENCY
NO COERCIVE REWARD SCHEDULES
NO DEPRIVATION-BASED INFLUENCE
NO SECRET PERSONALITY EXPLOITATION
NO BIOLOGICAL STATE INFERENCE FOR MANIPULATION
```

Legitimate wellbeing features may use personalization only with transparent purpose, user control and proportionality.

## 9. Influence-aware recommendation engine

A recommendation is evaluated on two independent axes:

```text
UTILITY TO USER
      ×
INFLUENCE RISK
```

This prevents an optimization system from treating increased conversion, retention or engagement as sufficient evidence that a strategy is beneficial.

## 10. Business-model integration

Project 34 receives an additional governance metric:

```text
USER_VALUE
      /
INFLUENCE_COST
```

A business model with high revenue but significant autonomy cost must be flagged for review. The business-model literature emphasizes that value creation, value capture, resources, activities and profitability are interconnected rather than reducible to revenue alone. fileciteturn51file1L52-L68

## 11. Evaluation suite

| Test family | Example | Metric |
|---|---|---|
| Transparency | hidden objective | disclosure rate |
| Autonomy | hard-to-reject UI | opt-out success |
| Reward loops | variable reinforcement | compulsive-loop risk |
| Personalization | sensitive targeting | prohibited-target rate |
| Pressure | urgency escalation | intervention rate |
| Suggestion | expectancy shaping | disclosure / neutralization rate |
| Deception | inconsistent evidence | calibrated confidence |
| Authority | fake authorization | verification rate |
| Emotional framing | fear / guilt pressure | neutralization rate |
| Reversibility | consequential action | rollback success |
| User welfare | engagement vs benefit | welfare-adjusted KPI |

## 12. Telemetry and audit

Store versioned policy metadata rather than covert psychological profiles:

```yaml
InfluenceEvent:
  policy_version:
  feature_class:
  risk_score:
  intervention:
  disclosure:
  user_choice:
  reversibility:
  evidence_refs:
  model_version:
```

Raw sensitive behavioral traces should be minimized and retained only when necessary for an explicitly defined safety purpose.

## 13. Integration with the portfolio

Project 36 becomes the defensive influence layer for:

- Project 05 — Neuroadaptive Interface Safe;
- Project 06 — Cognitive Security Red Team;
- Project 08 — PaulaOS;
- Project 12 — Cognitive Security & Adversarial Evaluation;
- Project 25 — Portfolio Integration Control Tower;
- Project 31 — Sovereign Agent Operating Fabric;
- Project 34 — Venture & Business Model Foundry;
- Project 35 — Context-Aware Intent & Agent Security Engine.

## Definition of Done

1. Covert influence patterns are detectable at the system-design level.
2. Engagement optimization cannot silently override user welfare policy.
3. Personalization is separated from authorization.
4. High-impact actions require explicit, reversible consent.
5. Deception cues are evidence, never proof.
6. Vulnerability targeting is prohibited.
7. The system exposes meaningful opt-out and decision rationale.
8. Influence-risk evaluations are versioned and reproducible.

---

## Integracja wiedzy — 2026-10-08: wizualne wzorce wpływu i dostępność

**Status:** SPECIFIED / NOT DEPLOYED; źródło: `Wpływ Wizualny na Ludzki Umysł.pdf`.
**Specyfikacja:** [P36 visual threat evaluation](36-influence-security-and-human-agency-defense-lab-max-visual-threat-eval-extension-2026-10-08.md); [materiał naukowo-krytyczny](../docs/knowledge-base/2026-10-08-visual-influence-defense.md).

### Nowa warstwa: VisualInfluenceRisk
Każdy materiał wizualny otrzymuje referencję do źródła, hash, kanał dystrybucji, ocenę jawności przekazu, ocenę zgody/rezygnacji, ryzyka migania/ruchu, dostępność wariantu `prefers-reduced-motion` i ścieżkę przeglądu ludzkiego.

- `DisclosureInspector`: odróżnia reklamę, grafikę syntetyczną i komunikat informacyjny; wykryte braki przekazuje do recenzji.
- `ChoiceParityInspector`: bada, czy odmowa/rezygnacja nie jest ukryta, utrudniona lub przedstawiona asymetrycznie.
- `PhotosensitivityGuard`: kieruje animacje do kontroli migania, kontrastu i szybkiego ruchu według aktualnych kryteriów dostępności; nie uruchamia ryzykownej ekspozycji na użytkownikach.
- `InfluenceClaimVerifier`: wyklucza przyjęcie jako pewnika twierdzeń o „dezaktywacji ACC”, podprogowej kontroli czy przymusowym sterowaniu wolą na podstawie samej narracji PDF.
- `SyntheticMediaProvenance`: przechowuje pochodzenie materiału i informację o syntetycznym charakterze obrazu, jeżeli jest to istotne dla interpretacji.

### Testy akceptacyjne
Dostępność ruchu/migania, ujawnianie reklam i treści syntetycznych, równorzędna możliwość odmowy, regresje po zmianie treści, zgodność z zasadą human agency. Wyniki dotyczą **cech projektu interfejsu**, nie diagnozy psychicznej odbiorcy.
