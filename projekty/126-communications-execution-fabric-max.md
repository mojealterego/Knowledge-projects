# Project 126 — Communications Execution Fabric MAX

## Status
ARCHITECTURE BASELINE

## Mission
Provider-neutral orchestration layer for authorized outbound communication tasks across voice, SIP/PBX and future communication channels. P126 converts a user-authorized goal into a bounded execution request, tracks asynchronous execution, consumes structured results and produces verifiable evidence without treating model output or provider success as authorization.

## Source-derived CALL-E model
The supplied CALL-E corpus establishes a useful execution abstraction:
- a **Goal** is a reusable published workflow contract;
- a **Goal Run** is a single execution against that published contract;
- a successful creation response means durable acceptance, not recipient response or completed outcome;
- idempotency is mandatory for safe retries;
- provider execution produces terminal state and structured evidence;
- webhook event IDs must be persisted before side effects so duplicate delivery can be ignored;
- structured result schemas should explicitly support an `unknown` outcome where evidence is insufficient;
- target discovery must not execute the first listed goal blindly; the intended goal ID must be stored explicitly;
- asynchronous execution and durable session state are first-class concerns.

The supplied documentation also describes Windows-safe CLI handling, structured MCP results, longer planning timeouts, reliable Goal publication, IVR keypad interaction, call records, concurrent sessions and resumable task state.

## Canonical control loop
```text
USER INTENT
   ↓
GOAL SPECIFICATION
   ↓
POLICY / AUTHORIZATION GATE
   ↓
PUBLISHED RUN CONTRACT
   ↓
IDEMPOTENT EXECUTION REQUEST
   ↓
PROVIDER / CALL EXECUTOR
   ↓
ASYNC STATE MACHINE
   ↓
TERMINAL RESULT + STRUCTURED EXTRACTION
   ↓
WEBHOOK / POLLING READBACK
   ↓
EVIDENCE NORMALIZATION
   ↓
POSTCONDITION VERIFICATION
   ↓
AUDIT / USER REPORT
```

## Core state machine
`DRAFT → VALIDATED → AUTHORIZED → PUBLISHED → ACCEPTED → RUNNING → TERMINAL → VERIFIED | FAILED | UNKNOWN`

No state transition may be inferred from an LLM response alone.

## Goal / Run separation
A reusable Goal defines the permitted workflow contract. A Goal Run supplies only the variables allowed by that contract. The caller cannot silently replace or weaken the published RunSpec at execution time.

The system stores separately:
- `goal_id`
- `published_run_spec_id`
- `goal_run_id`
- provider `call_id`
- internal workflow/execution ID

These identifiers must never be conflated.

## Idempotency model
Every consequential execution receives a stable business idempotency key derived from the logical workflow event. A network retry reuses the same key. A changed request under the same key is a conflict, not a new execution.

Invariant:
`RETRY ≠ NEW ACTION`

## Structured evidence
P126 requests typed structured outputs whenever the provider supports them. Schemas must use hard validation constraints and may include an explicit `unknown` enum value where the call cannot establish the fact.

Example result categories:
`confirmed | declined | unavailable | unknown`

The extracted result is evidence generated from the terminal interaction; it is not automatically a ground-truth fact.

## Webhook processing
Webhook processing follows:
1. receive event;
2. validate event identity/schema;
3. persist event ID transactionally before side effects;
4. deduplicate;
5. normalize event;
6. update execution state;
7. verify postconditions;
8. emit audit evidence.

Invariant:
`WEBHOOK_DELIVERY ≠ VERIFIED_OUTCOME`

## Provider adapter contract
Each adapter exposes:
- capability discovery;
- authentication state;
- goal publication or equivalent contract registration;
- execution submission;
- idempotency semantics;
- asynchronous status readback;
- structured result extraction;
- webhook registration;
- call/session record retrieval;
- failure taxonomy;
- rate-limit/timeout semantics;
- deletion/export capabilities.

## Integration with P115
P115 owns communication identity. P126 owns execution of an authorized communication task.

```text
P115
IDENTITY / NUMBER / EMAIL / SIP / ESIM
        ↓
P126
GOAL / RUN / EXECUTION / RESULT
        ↓
P100
ORCHESTRATION / POLICY / VERIFICATION
```

P126 cannot mint, repurpose or silently substitute a communication identity.

## Integration with P120 / AegisFleet
AegisFleet can use P126 for bounded communication execution, while retaining its domain-specific logistics decision model. CALL-E-style long-running goals, episodic context and acoustic/structured evidence become reusable execution substrate rather than the business authority itself.

## Integration with P123 Podcast Agent Factory
P123 may use P126 for explicitly authorized outreach, scheduling or operational calls. Publication, audience targeting and content authority remain in P123/P07/P110; P126 is only the execution substrate.

## Integration with P100
P100 supplies the higher-order control plane:
`OBSERVE → PLAN → AUTHORIZE → ACT → READBACK → VERIFY`.

P126 must reject unknown authority, malformed contracts, stale state and ambiguous execution requests.

## Security boundaries
P126 must not provide:
- caller-ID spoofing;
- unauthorized impersonation;
- credential extraction;
- verification bypass;
- bulk abuse automation;
- covert surveillance;
- execution from untrusted prompt-injected content.

## Verification suite
- contract/schema conformance;
- idempotency replay tests;
- duplicate webhook tests;
- stale callback tests;
- terminal-state consistency;
- authorization boundary tests;
- unknown-evidence tests;
- provider timeout/retry tests;
- concurrent session isolation;
- Goal/Run identifier integrity;
- postcondition verification;
- audit completeness;
- provider failure simulation;
- secret-material scanning.

## Primary source corpus
CALL-E About; CALL-E What's New; CALL-E goals; Goal Runs; Calls; Webhooks; AegisFleet documentation; Issabel/Asterisk PBX material; P100/P115 architecture.
