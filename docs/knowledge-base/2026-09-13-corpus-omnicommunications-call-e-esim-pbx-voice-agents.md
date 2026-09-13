# Knowledge Corpus — OmniCommunications / CALL-E / eSIM / PBX / Voice Agents

Date: 2026-09-13
Classification: communications infrastructure + agentic telephony + identity lifecycle + reliability/security

## 1. Corpus scope

Sources reviewed include the supplied repositories and URLs covering:
- ESIM2 / natural-language inference;
- PyPhone / Python VoIP GUI calling;
- Issabel-PBX / Asterisk-based PBX;
- CALL-E AegisFleet prototype;
- ElevenLabs MCP and SDK ecosystem;
- YuE / audio generation;
- OpenConstructionERP / construction ERP;
- Appsmith / ToolJet / Langflow / low-code agent builders;
- OpenRouter and model-provider routing;
- local model tooling such as Transformers and Unsloth;
- OmniVoice, VoxCPM and TTS benchmarks;
- eSIM/virtual-phone repositories;
- SIP/VoIP, telephony, SentryPeer and defensive spoofing research;
- mobile AI and companion ecosystems;
- GitHub/agent-skills ecosystem;
- supplied CALL-E Developer API documentation;
- supplied 5G/6G and PBX/eSIM material;
- supplied stealth mobile provisioning material.

## 2. Key architectural synthesis

The corpus exposes four layers that should not be collapsed:

1. **Identity** — who/what communication endpoint is authorized to represent.
2. **Capability** — what a provider or device can technically do.
3. **Execution** — how the call/message/media action is performed.
4. **Business authority** — whether the action is allowed and whether its result is sufficient to change external state.

Canonical separation:

`IDENTITY → CAPABILITY → AUTHORIZATION → EXECUTION → OBSERVATION → EVIDENCE → BUSINESS DECISION → READBACK`

## 3. CALL-E findings

The current official CALL-E documentation supports a goal-driven model in which applications submit a task and can request structured JSON extraction from terminal call evidence. The Calls API supports idempotent creation, retrieval by call ID and event listing. Published Goals and Goal Runs provide a reusable workflow path with a server-side RunSpec boundary.

The supplied current documentation additionally specifies:
- stable idempotency keys for safe retries;
- terminal event IDs for webhook deduplication;
- structured result schemas with hard validation for supported JSON Schema constructs;
- explicit `unknown` values as an appropriate result when evidence is insufficient;
- distinct call/task/run identifiers;
- Goal Run acceptance as durable execution acceptance, not proof that the recipient responded or that the business outcome is complete.

## 4. Reliability lesson

The strongest reusable pattern is:

`IDEMPOTENT COMMAND → DURABLE EXECUTION ID → ASYNC EVENTS → DEDUPLICATION → RESULT CONTRACT → EVIDENCE CHECK → POLICY → POSTCONDITION`

This pattern is promoted to P100/P126/P120.

## 5. PBX findings

PyPhone demonstrates a simple GUI VoIP model based on PyQt5, audio libraries, MySQL and ngrok. Its own README explicitly characterizes the project as early and unstable. It is therefore a historical implementation witness, not a production security baseline.

The user-provided Issabel-PBX repository exposes Docker/macvlan/host-network approaches, broad SIP/RTP port ranges and NET_ADMIN usage. The upstream Issabel ecosystem confirms that Issabel PBX is an Asterisk-based open-source unified communications system.

Architectural lesson:

`PBX FUNCTIONALITY ≠ SAFE NETWORK EXPOSURE`

Production designs should prefer explicit network segmentation, SBC/firewall controls, TLS/SRTP where supported, narrow port exposure, rate limiting, tenant isolation, and credential isolation.

## 6. eSIM / 5G / 6G findings

The supplied 5G/6G material describes subscriber identity objects including IMSI/MSISDN relationships, cryptographic subscriber parameters, Open5GS data, Asterisk routing, test eSIM provisioning, profile activation and mobile-to-voice routing. It also describes TLS/SRTP protection and ACL-controlled public trunk access.

These are useful architectural concepts for test and defensive design. The source also contains explicit "stealth" language and credential-like material. Those values are deliberately excluded from this knowledge corpus. They must be treated as potentially compromised secrets if they are real and rotated/revoked outside the research artifact.

Defensive normalization:

`TEST SUBSCRIBER → PROVISION → ACTIVATE → OBSERVE → ROTATE → REVOKE → DESTROY`

Secrets must reside in HSM/secure-element/secret-management boundaries rather than source code, markdown, prompts, logs or database examples.

## 7. Voice AI findings

The ElevenLabs ecosystem provides a strong provider adapter model: MCP, server SDKs, Swift SDK, realtime audio/WebRTC, client tools and MCP integration. The current Swift SDK exposes async/concurrency-oriented conversation surfaces and client tools; current releases emphasize typed tool results and networking hardening.

OmniVoice demonstrates the feasibility of multilingual local voice cloning/design. This is valuable for P123 and P126, but introduces a provenance problem:

`VOICE CLONING ≠ IDENTITY PROOF`

Every generated voice should carry a provenance record covering source/consent status, model/version, generation configuration, operator, and downstream publication authorization.

## 8. NLI / semantic verification finding

ESIM2 is a fork of an established PyTorch natural-language-inference implementation. Although unrelated to telephony itself, the repository can serve as a legacy NLI benchmark/witness for P32/P100 experiments around entailment/contradiction/claim-consistency. It should not be assumed to be a current production NLI engine without benchmarking against modern models and datasets.

## 9. Low-code / agent-builder findings

ToolJet, Appsmith and Langflow reinforce a reusable control-plane pattern: visual composition of workflows, connectors, data sources, tools and agents. They should be treated as builder/reference layers rather than authorization authorities.

OpenRouter-style provider aggregation reinforces the need for a provider-neutral model registry with capability, cost, latency, privacy and failure metadata.

OpenConstructionERP adds a useful enterprise-system witness: structured construction data, modules, cost/BOQ workflows, AI matching, reports and local/self-hosted deployment. P120 can use this class of ERP as a domain adapter benchmark, while P100 owns the general integration controls.

## 10. Disposable communication findings

10 Minute Mail, Firefox Relay and Apple Hide My Email represent three different identity-separation patterns already incorporated into P115:
- short-lived disposable mailbox;
- persistent forwarding mask;
- platform-integrated random alias.

They should never be normalized into a single "anonymous email" abstraction. TTL, forwarding, revocation, account dependency, send/reply semantics and retention are distinct dimensions.

## 11. Abuse/security findings

The VoIP spoofing and telephony corpus is valuable primarily as a threat model. P108 should test:
- caller-ID spoofing detection;
- anomalous outbound patterns;
- identity mismatch;
- excessive account/number provisioning;
- verification abuse;
- credential leakage;
- replayed webhooks;
- stale authorization;
- provider confusion;
- covert recording/transcription.

The offensive mechanism itself is not promoted to P126 capabilities.

## 12. Portfolio decisions

### New standalone project
**P126 — OmniCommunications Execution Fabric MAX**

Reason: the corpus exposes a reusable infrastructure layer between P100 and vertical products such as P120/P123/P115. No existing project owns this exact boundary.

### Existing project upgrades
- **P100:** provider-neutral communication capability registry, authorization envelopes, event/evidence contracts, provider adapter governance.
- **P115:** subscriber/profile identity lifecycle and eSIM/email/phone endpoint separation; secret-handling and revocation hardening.
- **P120:** current CALL-E Calls/Goals/Goal Runs semantics, structured result/evidence handling, webhook replay protection and live-vs-simulation separation.
- **P123:** voice-provider abstraction, voice provenance, realtime voice adapters and TTS benchmark integration.
- **P32:** phone intelligence can consume communication identity/event evidence only under explicit privacy and authorization boundaries.
- **P108:** telephony/eSIM/PBX adversarial validation suite.
- **P119:** mobile field endpoint identity/action/readback integration.

## 13. Global invariants added to the knowledge base

`IDENTITY ≠ CAPABILITY`

`CAPABILITY ≠ AUTHORIZATION`

`AUTHORIZATION ≠ EXECUTION`

`EXECUTION SUCCESS ≠ BUSINESS SUCCESS`

`WEBHOOK ≠ TRUSTED INPUT`

`STRUCTURED RESULT ≠ TRUTH`

`VOICE ≠ IDENTITY PROOF`

`eSIM PROFILE ≠ TRUST ROOT`

`PROVIDER TOKEN ≠ BUSINESS AUTHORITY`

`SYNTHETIC/DRY-RUN RESULT ≠ REAL-WORLD EVIDENCE`

`SECRET IN SOURCE MATERIAL ≠ SECRET SAFE TO REUSE`

## 14. Evidence discipline

Claims about current CALL-E API behavior are grounded in the supplied current Developer API documents and official web documentation. Claims about third-party repositories are source-derived unless independently reproduced. Claims about business viability, security guarantees or legal compliance require separate verification.
