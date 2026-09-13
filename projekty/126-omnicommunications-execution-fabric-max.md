# P126 — OmniCommunications Execution Fabric MAX

## Status
ARCHITECTURE BASELINE / NEW STANDALONE SUBSTRATE

## Mission
Build a provider-neutral communications execution substrate for agentic systems that unifies bounded phone, SIP/PBX, mobile/eSIM identity, voice-agent, and communication-event capabilities without conflating identity, capability, authorization, or business outcome.

P126 is infrastructure. It does not own the business domain of logistics, podcasting, companion products, or disposable identity; those remain P120, P123, P112 and P115 respectively.

## Why a new project is justified
P115 owns communication identity and alias lifecycle. P120 owns logistics crisis coordination. P123 owns podcast production. P100 owns the general agentic control plane. The new corpus reveals a distinct missing layer: a reusable **communications execution fabric** that can connect those products to real-world voice/telephony infrastructure while enforcing a single authorization and verification model.

## Canonical architecture

```text
                 USER / POLICY OWNER
                         ↓
                 COMMUNICATION INTENT
                         ↓
              P100 CAPABILITY BROKER
                         ↓
             COMMUNICATION ORCHESTRATOR
          ┌──────────────┼────────────────┐
          ↓              ↓                ↓
     IDENTITY PLANE   CHANNEL PLANE    AGENT PLANE
        P115        SIP/PBX/PSTN/eSIM  CALL-E/TTS/STT
          └──────────────┼────────────────┘
                         ↓
                 EXECUTION ADAPTERS
                         ↓
               EVENT / WEBHOOK BUS
                         ↓
             RESULT + EVIDENCE LAYER
                         ↓
             POLICY / CONFIDENCE GATE
                    ↙         ↘
             HUMAN REVIEW     AUTHORIZED ACT
                    ↘         ↙
                  READBACK → VERIFY
                         ↓
                 AUDIT / LINEAGE
```

## Channel domains

### 1. Agentic phone execution
CALL-E is treated as a goal-driven execution provider. Current official documentation exposes one-shot Calls plus reusable published Goals/Goal Runs, structured result extraction, event listing and terminal webhooks. The provider contract must be versioned in an adapter rather than embedded into business logic.

### 2. SIP / PBX
Issabel/Asterisk and PyPhone are retained as implementation witnesses for SIP, RTP, IVR, queues, CDR, GUI calling and self-hosted telephony. Their older network assumptions must not be promoted unchanged into production architecture.

Required P126 controls:
- TLS for SIP signalling where supported;
- SRTP for media where supported;
- SBC/firewall boundaries;
- explicit ingress/egress ACLs;
- rate limiting and fraud detection;
- tenant isolation;
- recording/transcription consent policy;
- immutable call/event correlation;
- no broad host networking as a default security posture.

### 3. Mobile / eSIM identity
The supplied 5G/6G material is treated as a defensive architecture source for subscriber identity lifecycle, IMSI/MSISDN mapping, profile provisioning, network policy and mobile-to-PBX routing.

P126 does **not** operationalize stealth/evasion techniques or propagate subscriber secrets. Any real Ki/OPc, database passwords, access tokens, or other credentials appearing in source material are classified as sensitive and must never enter the knowledge corpus, source control, prompts, logs or examples.

Canonical lifecycle:

`PROFILE_INTENT → TEST_IDENTITY → PROVISION → ACTIVATE → BIND → OBSERVE → ROTATE → SUSPEND → REVOKE → DESTROY`

Production identity secrets belong in an HSM/secure element or dedicated secret-management boundary, not application configuration.

### 4. Voice model plane
ElevenLabs MCP/SDK, ElevenAgents Swift SDK, OmniVoice, VoxCPM, TTS WebUI and related voice projects become provider/model adapters. Voice generation is not identity proof. Voice cloning requires explicit provenance/consent metadata and an auditable voice-profile lineage.

## Identity separation

P126 never treats a phone number, IMSI, email mask, SIP account, voiceprint or provider token as the authoritative user identity.

The authoritative object is:

`communication_identity_id + owner + purpose + consent + capability_scope + lifecycle + provenance`

P115 remains the owner of alias/mask/identity lifecycle. P126 consumes those identities through a typed interface.

## CALL-E integration contract

The current provider evidence establishes several important primitives:

- `POST /v1/calls` creates asynchronous call tasks;
- stable `Idempotency-Key` protects retries from intentional duplication;
- `GET /v1/calls/{call_id}` returns current call state and structured results;
- call events are separately retrievable;
- published Goals can be listed and executed through Goal Runs;
- Goal Run creation binds execution to the published RunSpec and rejects caller-side schema/contract substitution;
- terminal webhooks provide event identifiers and terminal outcome categories.

P126 adds a provider-neutral wrapper:

`CallIntent → ProviderPlan → AuthorizationEnvelope → ProviderExecution → ProviderEvent → ResultContract → EvidenceAssessment → BusinessDecision`

A successful provider acceptance is never interpreted as a completed business action.

## Event model

Every external communication event is normalized into:

```text
event_id
provider
provider_event_type
communication_id
identity_id
business_correlation_id
occurred_at
received_at
sequence_hint
payload_digest
validation_status
provenance
```

The event processor must support:
- durable deduplication;
- replay;
- out-of-order delivery;
- provider retries;
- schema evolution;
- correlation loss;
- terminal-state monotonicity;
- stale-event rejection.

## Result model

P126 standardizes `unknown` as a first-class result state.

Example:

```text
OUTCOME = yes | no | unknown
EVIDENCE = source-linked observation
CONFIDENCE = low | medium | high | unknown
AUTHORITY = not_granted | granted
BUSINESS_STATE = proposed | approved | applied | rejected | verified | escalated
```

`confidence` is not truth. `structured_result` is not truth. `evidence` is not authorization.

## Authorization envelope

Every consequential execution carries:

```text
principal
agent_id
agent_version
identity_id
channel
recipient_scope
purpose
policy_version
allowed_action
allowed_time_window
budget
approval_state
expires_at
correlation_id
```

The provider adapter receives only the minimum fields required to execute the already-authorized operation.

## Security invariants

- `MODEL OUTPUT ≠ AUTHORIZATION`
- `MCP ≠ AUTHORIZATION AUTHORITY`
- `PHONE NUMBER ≠ IDENTITY PROOF`
- `VOICE ≠ IDENTITY PROOF`
- `SIM/eSIM PROFILE ≠ TRUST ROOT`
- `PROVIDER TOKEN ≠ BUSINESS AUTHORITY`
- `CALL PLAN ≠ CALL APPROVAL`
- `201/ACCEPTED ≠ BUSINESS COMPLETION`
- `WEBHOOK ≠ TRUSTED INPUT`
- `STRUCTURED JSON ≠ FACT`
- `CONFIDENCE ≠ TRUTH`
- `CALL COMPLETION ≠ POSTCONDITION`
- `SYNTHETIC CALL ≠ REAL-WORLD EVIDENCE`
- `LOG SUCCESS ≠ STATE SUCCESS`

## Abuse-resistance boundary
P126 must not provide:
- caller-ID spoofing;
- fraud/scam automation;
- bulk account creation;
- CAPTCHA/anti-abuse bypass;
- identity-verification evasion;
- credential extraction;
- covert surveillance;
- stealth provisioning for evasion.

Those patterns are legitimate inputs for P108 adversarial testing and detection regression only.

## Provider adapter registry

Initial adapter classes:

- `call-e-goal-runs`
- `call-e-one-shot-calls`
- `asterisk-pjsip`
- `issabel-pbx`
- `generic-sip`
- `mobile-esim-test-core`
- `elevenlabs-voice`
- `elevenlabs-mcp`
- `elevenagents-swift`
- `local-tts`
- `local-stt`

Each adapter must declare capabilities, side effects, credential class, supported regions, lifecycle semantics, rate limits, failure modes, evidence quality, and verification hooks.

## Integration map

- **P100** — capability broker, authorization, sandboxing, provenance, verification, agent identity and durable workflows.
- **P115** — communication identity, masks, aliases, temporary/persistent endpoint lifecycle.
- **P120** — logistics voice operations and ERP-connected call workflows.
- **P123** — podcast voice production and acoustic/voice adapters.
- **P119** — local Android field endpoints and action/readback.
- **P32** — phone intelligence under privacy/authorization constraints.
- **P108** — adversarial telephony, identity and anti-abuse testing.
- **P113** — model capability routing where voice/video assets are generated.

## New reusable primitives

- `communication-intent`
- `communication-identity-ref`
- `authorization-envelope`
- `provider-plan`
- `provider-execution`
- `communication-event`
- `result-contract`
- `evidence-assessment`
- `business-postcondition`
- `provider-adapter-manifest`
- `communication-lineage`
- `credential-class`
- `voice-provenance-record`
- `subscriber-profile-lifecycle`

## Verification program

1. Provider contract conformance.
2. Idempotent retry tests.
3. Duplicate-event and replay tests.
4. Out-of-order event tests.
5. Stale-state rejection.
6. Authorization-envelope expiry tests.
7. Recipient/identity binding tests.
8. Voice-profile provenance tests.
9. SIP/TLS/SRTP configuration validation.
10. PBX exposure and ACL regression tests.
11. eSIM test-profile lifecycle tests without real production credentials.
12. Secret scanning and credential-leak prevention.
13. Recording/transcription consent tests.
14. Multi-tenant isolation.
15. Provider failure and failover tests.
16. Human escalation and resumable approval tests.
17. Authoritative readback and postcondition verification.
18. Red-team abuse regression through P108.

## Definition of Done

P126 is ready for downstream production use only when one complete provider path can demonstrate:

`INTENT → POLICY → AUTHORIZATION → EXECUTION → EVENT → RESULT → EVIDENCE → BUSINESS ACTION → READBACK → VERIFIED POSTCONDITION`

with durable lineage, bounded credentials, replay-safe events, explicit human approval where required, and no propagation of provider secrets into artifacts or logs.
