# 01 — APEIRON BLACK SYSTEM — MAX

## Status
**Engineering Specification Baseline**

## Scope
Stateful symbolic card interaction engine. The project evolves the existing visual card concept into a deterministic, testable application with a strict separation between artifact data, ritual/session state, interpretation, persistence and presentation.

## Engineering objective
Provide a production-ready domain boundary in which:

- card data is immutable and versioned;
- session state is explicit and cannot enter impossible states;
- randomness is reproducible in test mode;
- AI interpretation is advisory and has no direct authority over domain state;
- persistence does not equate to presentation-layer clearing;
- every consequential state change produces an auditable event;
- accessibility and reduced-motion behavior are first-class requirements.

## Domain model

```ts
type CardId = string;
type SessionId = string;
type DrawId = string;

type ApeironCard = {
  id: CardId;
  arcana: 'major' | 'void' | 'glitch' | 'memory' | 'system';
  archetype: string;
  title: string;
  descriptions: {
    dormant: string;
    active: string;
    uv: string;
  };
  symbolism: string[];
  tags: string[];
  riskLevel: 'low' | 'medium' | 'high';
  icon: string;
};

type SessionState =
  | 'IDLE'
  | 'DRAWING'
  | 'REVEALED'
  | 'INSPECTING'
  | 'UV_REVEAL'
  | 'ARCHIVED';
```

### State-transition contract

```text
IDLE --start--> DRAWING
DRAWING --draw_success--> REVEALED
REVEALED --inspect--> INSPECTING
INSPECTING --uv_reveal--> UV_REVEAL
UV_REVEAL --finish--> REVEALED
REVEALED --archive--> ARCHIVED
ARCHIVED --new_session--> IDLE
```

Invalid transitions must return typed domain errors and leave authoritative state unchanged.

## Application architecture

```text
Presentation
  ├─ CardRenderer
  ├─ SessionControls
  ├─ HistoryView
  └─ AccessibilityAdapter
          │
Application
  ├─ SessionService
  ├─ DrawService
  ├─ InterpretationService
  └─ ArchiveService
          │
Domain
  ├─ CardCatalog
  ├─ SessionAggregate
  ├─ DrawPolicy
  └─ DomainEvents
          │
Infrastructure
  ├─ CardRepository
  ├─ SessionStore
  ├─ EventStore
  └─ TelemetrySink
```

Dependencies point inward. The domain must not depend on React, browser storage, network clients or an LLM SDK.

## Deterministic randomness

Production random draws may use a secure browser entropy source when the product requires unpredictable selection. Tests use a seeded PRNG and record the seed in the test fixture. The shuffle implementation must satisfy permutation and cardinality invariants.

Required invariants:

- no duplicated card IDs in a draw unless explicitly allowed by deck policy;
- every returned card exists in the catalog version used by the session;
- draw size is within configured bounds;
- clearing the visual table never mutates archived draw history.

## Canonical event model

```ts
type ApeironEvent =
  | { type: 'SESSION_STARTED'; sessionId: SessionId; at: string }
  | { type: 'DRAW_REQUESTED'; drawId: DrawId; count: number; at: string }
  | { type: 'DRAW_COMPLETED'; drawId: DrawId; cards: CardId[]; at: string }
  | { type: 'CARD_INSPECTED'; cardId: CardId; at: string }
  | { type: 'UV_ENABLED'; cardId: CardId; at: string }
  | { type: 'SESSION_ARCHIVED'; sessionId: SessionId; at: string }
  | { type: 'PRESENTATION_CLEARED'; sessionId: SessionId; at: string };
```

The terminal/diagnostic UI is a projection of these events, not their authoritative source.

## Persistence contract

Separate stores are required for:

1. **Catalog** — immutable/versioned card definitions.
2. **Session state** — current resumable aggregate.
3. **History** — archived immutable draw/session records.
4. **Telemetry** — operational events and performance metrics.

A UI reset may only emit `PRESENTATION_CLEARED`; it must not delete history unless an explicit archive-deletion command exists and is separately authorized.

## Interpretation boundary

Deterministic interpretation may operate on catalog metadata and draw structure. An optional LLM adapter receives a read-only projection:

```json
{
  "sessionId": "...",
  "cards": ["..."],
  "drawContext": "...",
  "catalogVersion": "..."
}
```

Output must validate against a versioned schema. AI output is advisory only. It cannot create authoritative entities, change card state, delete history, grant capabilities or write memory outside an explicit application command.

## API / capability surface

### Read-only
- `apeiron.list_cards`
- `apeiron.get_card`
- `apeiron.get_session`
- `apeiron.get_history`

### Mutating
- `apeiron.start_session`
- `apeiron.draw_cards`
- `apeiron.inspect_card`
- `apeiron.reveal_uv`
- `apeiron.archive_session`
- `apeiron.clear_presentation`

Mutating operations require typed authorization at the application boundary.

## Error model

Errors are typed and observable:

```text
CatalogNotFound
InvalidTransition
InvalidDrawSize
DuplicateCard
SessionNotFound
PersistenceFailure
InterpretationValidationFailure
AuthorizationDenied
```

A failed command must be atomic from the user's perspective: no partial authoritative state transition is permitted.

## Accessibility

Required behavior:

- complete keyboard navigation;
- visible focus indicators;
- semantic buttons and headings;
- non-color-only state communication;
- text equivalents for card states and UV content;
- `aria-live` only for meaningful asynchronous status updates;
- reduced-motion mode that removes non-essential animation;
- sufficient contrast against the obsidian visual system;
- touch targets meeting the application's accessibility target size policy.

## Observability

Track at minimum:

- draw latency;
- command success/failure rate;
- invalid transition count;
- persistence failures;
- interpretation validation failures;
- session resume success rate;
- client error rate;
- performance metrics for card/UV transitions.

Every event should include correlation identifiers where relevant (`sessionId`, `drawId`, application version).

## Security

- no secrets in client bundles;
- no direct model-controlled privileged commands;
- validate all persisted and externally supplied identifiers;
- defend against DOM/XSS injection when rendering imported card content;
- treat AI/tool output as untrusted input;
- maintain an explicit separation between presentation state and archival data;
- rate-limit remote interpretation endpoints if introduced.

## Test strategy

### Unit
- state transition matrix;
- draw invariants;
- catalog schema validation;
- event serialization;
- interpretation schema validation.

### Property-based
- shuffle remains a permutation;
- repeated seeded runs produce identical results;
- invalid transitions never mutate state.

### Component
- dormant/active/UV rendering;
- keyboard flow;
- reduced-motion mode;
- error states.

### E2E
`start → draw → inspect → UV → archive → resume/read history → clear presentation`.

### Regression
Every previously discovered bug becomes a versioned regression fixture.

## Performance budget

Measure rather than assume. Define budgets for initial render, interaction latency, draw completion and card-list rendering. Large catalogs must not require unnecessary rerenders of unrelated UI.

## MCP integration rule
MCP adapters, if implemented, call the same application services as the UI. They must not bypass domain validation or persistence invariants.

## Definition of Done

APEIRON reaches **Implementation Ready** only when the domain contracts, state machine, persistence model, error model, security boundary, accessibility behavior, test matrix and observable event model are implemented or represented by executable tests. Production release additionally requires deployment verification and evidence from real test runs.

## Non-goals

This project does not define a general-purpose tarot system, an autonomous agent with unrestricted authority, or an LLM-controlled persistent memory layer.

## Architectural relationship

APEIRON may consume shared platform capabilities from the repository's agent/runtime/security projects, but remains independently understandable as a bounded application domain. Shared infrastructure must not erase the application's explicit domain invariants.

---

## 2026-10-08 — batch 12: Apeiron FOG feedback and symbolic-artifact loop

Source: `Tworzenie Nowego Systemu Dywinacji.pdf` (12 pages). It proposes `PrismLayers` transparent-card composition, `Metamodern Arcana`, operators/filters, a five-node Fractal Oracle Grid (FOG) and an iterative three-part decision gate. Existing P01 owns stateful symbolic card interactions; P53/P71 own physical/AR artifact production.

### `ApeironReflectionSession`
- `input_question`: explicitly framed as a decision/problem, **not a reliable supernatural forecast**;
- `fog_nodes`: INPUT, PROCESS, REINFORCING, BALANCING, OUTPUT;
- `layer_draw_events`: reproducible event IDs, physical/card layer identities, optional seeded symbolic RNG and provenance;
- `reframe_operations`: user-controlled movement of an output modifier back into the input domain;
- `triple_gate`: evidence/physical plausibility, participant values and practical resources;
- `reflection_result`: non-authoritative hypotheses, alternatives, next experiment, user choice and optional follow-up review.

### Safeguards & acceptance
Prohibit deterministic future-event certainty, coercive persuasion, mental health diagnosis, covert dependency loops and unverifiable "quantum entanglement" claims. Prototype should show reversible draws, accessibility equivalent to 3D/UV gimmicks, randomization replay, transparency of AI-generated interpretations, consent, data minimization and opt-out. No divination accuracy or therapeutic outcome is proven.
