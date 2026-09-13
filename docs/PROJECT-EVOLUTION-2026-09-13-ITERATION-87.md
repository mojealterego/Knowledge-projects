# Project Evolution — 2026-09-13 — Iteration 87

## Input

Large cross-project corpus covering eSIM/mobile networks, PBX/SIP/VoIP, CALL-E Developer API, AegisFleet, voice-agent providers, local TTS, phone intelligence, low-code agent builders, model routing, companion/mobile AI and the user's related repositories.

## Primary architectural decision

**NEW_PROJECT: P126 — OmniCommunications Execution Fabric MAX**

The corpus reveals a missing reusable substrate between P100's general control plane and vertical applications. P115 owns communication identity; P120 owns logistics; P123 owns podcast production. P126 owns the provider-neutral execution/event/evidence boundary for real-world communications.

## P126 core loop

`INTENT → POLICY → AUTHORIZATION → EXECUTION → EVENT → RESULT → EVIDENCE → BUSINESS ACTION → READBACK → VERIFY`

## Existing-project upgrades

### P100
Added communication capability manifests, authorization envelopes, provider adapters, event/evidence contracts and postcondition verification.

### P115
Expanded identity lifecycle to cover temporary/persistent aliases, SIP identities and test eSIM profiles. Added strict secret isolation and revocation requirements.

### P120
Updated AegisFleet for current CALL-E Calls, Goals and Goal Runs semantics; strengthened idempotency, opaque cursors, event deduplication, terminal-state handling and simulation boundaries.

### P123
Added provider-neutral voice federation and voice provenance/rights records. TTS benchmarks become reusable QA suites.

### P32
Added phone/communication evidence classification and uncertainty boundaries. Identifiers are not treated as identity proof.

### P108
Added telephony/eSIM/PBX adversarial validation and abuse-regression families.

### P119
Receives the P126 mobile endpoint/action/readback interface rather than provider-specific telephony authority.

## Source-derived lessons

- PyPhone is a useful historical VoIP GUI witness but explicitly describes itself as early/unstable; it is not a production security baseline.
- Issabel/Asterisk is a mature PBX reference, but broad network exposure and privileged networking must be hardened.
- ESIM2 is a legacy NLI benchmark/witness that can support semantic contradiction/entailment experiments, not a current telephony component.
- ToolJet/Appsmith/Langflow reinforce visual workflow composition but do not replace authorization control.
- OpenRouter/model-provider aggregation strengthens the need for capability/cost/privacy/latency-aware routing.
- ElevenLabs/OmniVoice/local TTS strengthen provider federation but require voice provenance.
- OpenConstructionERP provides an enterprise ERP integration witness for P120-style workflows.
- VoIP spoofing and phone-intelligence corpora are most valuable as defensive threat-model inputs.

## Sensitive-source handling

The supplied 5G/6G and stealth provisioning materials contain credential-like subscriber/database values. Those values are deliberately not copied into the repository. The resulting architecture keeps secrets outside source control and treats test subscriber profiles separately from production identities.

## New invariants

`IDENTITY ≠ CAPABILITY`

`CAPABILITY ≠ AUTHORIZATION`

`AUTHORIZATION ≠ EXECUTION`

`EXECUTION SUCCESS ≠ BUSINESS SUCCESS`

`VOICE ≠ IDENTITY PROOF`

`eSIM PROFILE ≠ TRUST ROOT`

`STRUCTURED RESULT ≠ TRUTH`

`WEBHOOK ≠ TRUSTED INPUT`

`SECRET IN SOURCE ≠ SECRET SAFE TO REUSE`

## Next maturity gate

Implement one complete P126 provider path in a sandbox/test environment with durable idempotency, replay-safe events, scoped credentials, structured evidence, human approval where required, and authoritative postcondition readback.
