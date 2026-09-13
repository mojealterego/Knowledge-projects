# Project 115 — OmniPrivacy Identity Relay & Disposable Communications Fabric MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → COMMUNICATION-IDENTITY EXTENSION

## Mission
Build a provider-neutral privacy and identity-separation fabric for legitimate use of alternate phone numbers, VoIP identities, email masks, temporary mailboxes, eSIM-backed connectivity and disposable communication endpoints while preserving authoritative ownership, consent, lifecycle control, provenance and abuse-resistant policy boundaries.

## New knowledge absorbed
The current corpus expands P115 from a generic alias layer into a **multi-channel communication identity fabric**. The important distinction is not "temporary vs permanent" but the combination of **purpose, lifetime, forwarding model, verification suitability, revocation semantics and authoritative ownership**.

### Email identity classes
- **Temporary mailbox** — short-lived inbox with bounded lifetime; appropriate only where mailbox loss is acceptable.
- **Forwarding alias/mask** — stable or semi-stable address that forwards to a protected primary mailbox and can be disabled independently.
- **Provider-managed private address** — platform-integrated alias generation with lifecycle controls.
- **Self-hosted relay** — user-controlled routing where infrastructure and retention policy are under the user's control.

### Phone identity classes
- Persistent secondary number.
- Temporary virtual number.
- SIP/VoIP identity.
- Cloud-PBX extension.
- eSIM-backed subscriber identity.
- Self-hosted/private telephony endpoint.

### Identity suitability matrix
Every identity must carry an explicit suitability state:
`GENERAL_CONTACT | LOW_TRUST_SIGNUP | ACCOUNT_RECOVERY_UNSUITABLE | FINANCIAL_UNSUITABLE | IDENTITY_VERIFICATION_UNSUITABLE | BUSINESS_CONTACT | EMERGENCY_CRITICAL`

The system must never infer that a disposable address or number is appropriate for a security-sensitive workflow merely because a provider accepts it.

## Distinct boundary
- **P95 — Omni-Entity Sovereign Network & Cyber-Resilience Fabric** owns resilient sovereign connectivity, identity/trust lifecycle and infrastructure continuity; P115 owns user-facing communication-identity abstraction.
- **P112 — OmniCompanion Sovereign Edge & Cognitive Relationship Platform** may consume P115 aliases for companion privacy, but P115 owns the communication identity lifecycle.
- **P111/P100** may orchestrate communication providers through authorized connectors; they do not own the identity-relay product boundary.
- **P108** owns adversarial validation of the boundary, not the relay service itself.
- **P126 — Communications Execution Fabric** may request authorized outbound communication execution; P115 owns the identity used by that communication and its lifecycle.

## Core capabilities
1. Alternate phone-number profiles.
2. Email alias/mask profiles.
3. Temporary mailbox profiles.
4. VoIP/SIP identity connectors.
5. eSIM/subscriber-profile abstraction where lawful provider APIs permit it.
6. Provider-neutral routing abstraction.
7. Lifecycle states: proposed → active → paused → expiring → revoked → destroyed.
8. User-controlled activation windows and retention.
9. Separation of personal, professional, project and public-facing identities.
10. Inbound message/call routing with authoritative delivery state.
11. Consent and authorization ledger.
12. Provenance for provider, number/address, acquisition time and lifecycle events.
13. Privacy-preserving local policy evaluation where feasible.
14. Export/deletion and revocation controls.
15. Defensive abuse-risk scoring and anomaly telemetry.
16. Verification-suitability policy.
17. Provider health and expiry monitoring.
18. Cross-channel identity graph without exposing unnecessary primary identifiers.

## Canonical architecture
```text
USER / POLICY OWNER
        ↓
IDENTITY INTENT
        ↓
IDENTITY PROFILE MANAGER
 ├── PHONE ALIAS
 ├── EMAIL MASK
 ├── TEMPORARY MAILBOX
 ├── VOIP / SIP IDENTITY
 ├── ESIM PROFILE
 └── PROJECT / BUSINESS IDENTITY
        ↓
PROVIDER ADAPTER LAYER
 ├── VIRTUAL NUMBER PROVIDERS
 ├── VOIP / CLOUD PBX
 ├── EMAIL RELAY
 ├── TEMP MAIL PROVIDERS
 ├── ESIM PROVIDERS
 └── LOCAL / SELF-HOSTED ENDPOINTS
        ↓
ROUTING + DELIVERY FABRIC
        ↓
POLICY / CONSENT / RISK / SUITABILITY GATE
        ↓
AUTHORITATIVE STATE
        ↓
AUDIT / EVIDENCE / USER CONTROL
```

## Identity model
Each communication identity is a versioned object containing:
- stable internal identity ID;
- external provider identifier where applicable;
- channel type;
- owner/tenant;
- purpose;
- consent scope;
- activation window;
- retention policy;
- provider metadata;
- routing rules;
- lifecycle state;
- verification-suitability state;
- provenance;
- audit history;
- revocation state.

The external phone number or email address is not the authoritative identity. The authoritative identity is the policy-controlled internal object and its provenance-bearing state.

## Privacy principles
- Minimize disclosure of the primary personal address/number.
- Prefer aliases over duplication of personal identity records.
- Keep sensitive routing metadata local where technically feasible.
- Make retention and deletion explicit.
- Do not infer anonymity from use of an alias.
- Do not treat disposable channels as suitable for critical identity verification unless the provider and policy explicitly support that use.
- Do not persist provider credentials, SIM secrets or routing secrets in source code or knowledge artifacts.

## Provider classes
### Persistent secondary number
Suitable for long-lived separation of personal/business/project communications.

### Temporary number
Suitable for bounded interactions such as classifieds or short-lived public contact where the user can revoke the identity afterwards.

### Cloud PBX / business VoIP
Suitable for teams, IVR, routing, call recording and analytics subject to applicable consent and legal requirements.

### Email relay/mask
Suitable for reducing exposure of a primary mailbox and controlling forwarding/spam/tracker exposure.

### Disposable mailbox
Suitable only for low-trust, low-value correspondence where loss of the mailbox is acceptable.

### eSIM-backed identity
Provides subscriber-level connectivity abstraction. P115 should treat eSIM provisioning as a provider-controlled resource, not as an identity-proof mechanism.

## Security and abuse boundary
P115 must not implement:
- caller-ID spoofing;
- fraud/scam calling;
- bulk account creation or activation;
- CAPTCHA or platform anti-abuse bypass;
- credential/session-token extraction;
- evasion of identity verification;
- covert surveillance.

These behaviors may appear as adversarial test cases in P108 to validate detection and policy enforcement.

## Verification plan
1. Provider adapter contract conformance.
2. Identity lifecycle state-machine tests.
3. Revocation propagation tests.
4. Routing correctness and stale-state prevention.
5. Consent boundary tests.
6. Data minimization and deletion tests.
7. Provider failure and number-expiry simulation.
8. Duplicate identity collision tests.
9. Audit/provenance completeness.
10. Privacy leakage tests.
11. Caller-ID and verification-abuse detection regression.
12. Multi-tenant isolation.
13. Recovery and migration tests.
14. Offline/local-policy degradation tests where supported.
15. Verification-suitability regression tests.
16. Secret-material scanning before knowledge/repository ingestion.
17. Provider credential rotation and revocation tests.
18. Cross-channel correlation minimization tests.

## New source-derived invariant set
`ALIAS ≠ ANONYMITY`
`TEMPORARY ≠ TRUSTED`
`FORWARDING ≠ OWNERSHIP`
`PROVIDER_ACCEPTANCE ≠ VERIFICATION_SUITABILITY`
`ESIM ≠ IDENTITY_PROOF`
`DELIVERY_SUCCESS ≠ HUMAN_RESPONSE`
`ROUTING_STATE ≠ AUTHORIZATION`
`CREDENTIAL MATERIAL ≠ KNOWLEDGE`

## Lineage
P37, P54, P61, P72, P95, P100, P105, P108, P111, P112, P126.

## Primary source cluster
10 Minute Mail; Firefox Relay; Apple Hide My Email; DuckDuckGo Email Protection; YOPmail; temporary-number ecosystem; eSIM/virtual-number repositories; Issabel/Asterisk/VoIP ecosystem; P126 CALL-E execution corpus; P115 prior architecture baseline.
