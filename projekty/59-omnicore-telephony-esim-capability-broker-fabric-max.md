# Project 59 — OmniCore Telephony & eSIM Capability Broker Fabric MAX

## 1. Purpose

Build a provider-neutral, mobile-aware capability plane for legitimate telecom and eSIM operations. The system treats phone identities, virtual numbers, SMS activations, eSIM profiles, eUICCs, LPA devices, QR activation artifacts and provider accounts as separate governed resources.

The project is explicitly **not** an account-verification bypass system, anonymous-identity generator, or automation layer for evading provider controls. Public/shared-number and filtering-evasion patterns from reviewed repositories are represented only as defensive threat cases.

## 2. Design objective

Compile a natural-language telecom/eSIM intent into a typed operation graph, evaluate authorization and risk before side effects, execute through provider/mobile adapters, and establish success through authoritative provider/device readback rather than model output.

```text
USER / OPERATOR
      ↓
INTENT COMPILER
      ↓
RESOURCE + EVIDENCE RESOLUTION
      ↓
POLICY / AUTHORIZATION / RISK
      ↓
CAPABILITY BROKER
      ↓
TYPED OPERATION GRAPH
      ↓
PROVIDER ADAPTER OR MOBILE/LPA ADAPTER
      ↓
EXECUTION / SANDBOX
      ↓
AUTHORITATIVE READBACK
      ↓
POSTCONDITION + AUDIT
      ↓
ARTIFACT / STATE DELTA
```

## 3. Resource model

### PhoneIdentity

```text
dialCode
internationalNumber
nationalNumber
isoCode
formatValidity
ownershipProof
```

The reviewed international-phone-input component provides a useful normalized data model. Format validity must never be treated as proof of identity ownership.

### TelecomNumberResource

```text
providerId
allocationId
number
country
operator
lifecycle
validFrom
validUntil
ownerSubject
authorizationScope
```

### SmsActivation

```text
activationId
numberResourceId
service
state
issuedAt
expiresAt
secretHandle
```

The OTP itself remains outside normal agent memory and logs.

### EuiccProfile

```text
profileId
provider
activationMetadata
state
targetEuiccId
targetDeviceId
provenance
```

### LpaDevice

```text
adapterId
platform
readerType
capabilities
trustState
firmwareMetadata
```

### QrActivationArtifact

```text
artifactId
issuer
payloadDigest
issuedAt
expiresAt
audienceBinding
replayState
provenance
```

## 4. Capability catalog

```text
phone.identity.normalize
phone.identity.validate-format
telecom.number.discover
telecom.number.quote
telecom.number.allocate
telecom.number.release
sms.activation.create
sms.activation.status.read
sms.activation.cancel
sms.inbox.read
sms.message.send
esim.profile.list
esim.profile.download
esim.profile.enable
esim.profile.disable
esim.profile.delete
esim.qr.generate
esim.qr.decode
lpa.device.enumerate
lpa.device.capabilities
lpa.profile.readback
observability.telecom.health
observability.telecom.audit
```

Capabilities are typed and policy-owned. Provider adapters never gain authority merely because the model selected them.

## 5. Risk classes

| Capability | Risk | Mandatory controls |
|---|---|---|
| Normalize/format validation | Low | deterministic parser |
| Country/service discovery | Low | provider provenance |
| Number allocation | High | subject authorization, quota, audit, spend policy |
| SMS inbox read | High | session binding, redaction, retention control |
| OTP access | Critical | secret handle, expiry, replay prevention, no model-memory persistence |
| SMS send | High | explicit authorization, destination policy, rate limit, audit |
| QR generation | High | issuer provenance, expiry, audience binding |
| Profile download | High | eligibility, device binding, approval, readback |
| Profile enable/disable | High | explicit policy, authoritative state verification |
| Profile delete | Critical | explicit confirmation, recovery analysis, authoritative readback |

## 6. Provider-neutral adapter contract

Inspired by the historical Twilio/Tropo abstraction and modern MCP/provider adapters:

```text
capabilities()
health()
listRegions()
listServices()
quote()
allocateNumber()
readActivationStatus()
releaseNumber()
readInbox()
sendMessage()
```

Adapters translate provider protocols; they do not decide whether an operation is allowed.

## 7. MCP integration

An MCP surface may expose read-only discovery and tightly constrained mutation tools. Tool schemas must encode required fields, bounded values, resource references and explicit risk metadata.

Recommended envelope:

```json
{
  "capability": "esim.profile.enable",
  "subjectId": "...",
  "resourceId": "...",
  "scope": "...",
  "expiresAt": "...",
  "approvalToken": "...",
  "requestId": "...",
  "expectedPostcondition": "profile.enabled"
}
```

The agent proposes the operation. The broker authorizes it. The adapter executes it. The observer establishes the resulting state.

## 8. eSIM/LPA mobile boundary

Use the LPA repository's device-adapter pattern as the mobile substrate boundary:

```text
React Native / platform UI
        ↓
LPA domain
        ↓
Device interface
        ↓
Native module / bridge
        ↓
OMAPI / CCID / CryptoTokenKit / other approved substrate
        ↓
eUICC
```

Each device adapter must advertise capabilities and trust state. Unsupported capabilities fail closed.

## 9. eSIM state machine

```text
DISCOVERED
   ↓
ELIGIBLE
   ↓
DOWNLOAD_PENDING
   ↓
DOWNLOADED
   ↓
ENABLED
   ↕
DISABLED
   ↓
DELETED
```

Every transition records actor, capability, policy decision, resource version, timestamp, provider/device response and authoritative readback.

## 10. SMS/OTP secrecy

SMS activation data is a secret-bearing workflow:

```text
PROVIDER
   ↓
activation resource
   ↓
secret handle
   ↓
short-lived secure retrieval
   ↓
explicitly authorized consumer
   ↓
redacted audit
```

Never:

- put raw OTPs into durable memory;
- write OTPs to ordinary logs or traces;
- infer authorization from receipt of an OTP;
- allow one session to retrieve another session's activation;
- expose shared/public inboxes without explicit privacy labeling.

## 11. QR security

A QR payload is an artifact carrying activation information, not proof of authority.

Gate order:

```text
Decode
 → Syntax validation
 → Issuer/provenance validation
 → Policy/eligibility
 → Device/audience binding
 → Expiry/replay validation
 → Execute
 → Read back device/provider state
```

## 12. Reliability and concurrency

Virtual threads or equivalent concurrency primitives are useful for I/O-heavy connector workloads, but parallelism remains bounded.

Controls:

- per-provider concurrency limits;
- per-user/session budgets;
- deadline propagation;
- cancellation;
- queue-based retry;
- exponential backoff with jitter;
- circuit breakers;
- idempotency keys;
- durable checkpoints.

Do not turn “cheap threads” into unbounded external API fan-out.

## 13. Local-first / edge-first deployment

For mobile LPA operations, keep sensitive device interaction local where technically supported. Use cloud/edge services for discovery, policy, orchestration and non-sensitive metadata.

```text
T0 Device-local
T1 Local network / edge
T2 Regional control service
T3 Cloud provider API
T4 External telecom provider
```

The system records the actual execution tier and does not assume that “local-first” configuration means all data stayed local.

## 14. Observability

Borrow the infrastructure-bootstrap pattern from the reviewed IBM Terraform module, but enforce stronger trust properties:

```text
runtime bootstrap
   ↓
health / metrics / logs / audit
   ↓
independent observer
   ↓
control-plane response
```

Certificate validation is mandatory. Monitoring configuration cannot silently weaken transport trust.

## 15. Monitorability-aware control

From Project 58:

```text
Strong model monitorability
  → normal verification tier

Weak/empty/unreliable reasoning trace
  → stronger action/state verification
  → tighter capability scope
  → deeper audit
  → higher human-review threshold
```

CoT is evidence, never the authorization boundary.

## 16. Threat model

| Threat | Control |
|---|---|
| Disposable/shared numbers used as pseudo-identities | provenance, ownership state, reputation, policy restrictions |
| Shared inbox data leakage | tenant/session isolation, short retention |
| OTP theft | secret boundary, expiry, replay protection |
| Automated account-verification bypass | prohibited capability; only authorized first-party workflows |
| Network/filter evasion | defensive detection only; no operational bypass path |
| Malicious QR payload | provenance, allowlists, replay checks, sandbox parsing |
| Rogue provider adapter | signed/versioned adapter registry, capability allowlist |
| Mobile native bridge compromise | platform sandboxing, code signing, least privilege |
| Stale state / races | resource versions, compare-and-swap, postcondition readback |
| Credential leakage | secret manager, short-lived credentials, trace redaction |
| Model metagaming / monitor awareness | external state checks, evaluator separation, policy independence |

## 17. Verification ladder

```text
L0 schema validation
L1 deterministic normalization
L2 policy validation
L3 capability authorization
L4 provider/mobile response validation
L5 authoritative state readback
L6 cross-observer consistency check
L7 human approval for critical mutation
L8 post-release telemetry/evaluation
```

A successful provider HTTP response is not equivalent to a successful real-world state transition.

## 18. Mapping to existing projects

- Project 31 → Agent OS, Skills, MCP, capability security.
- Project 37 → local/edge/cloud routing and provider-neutral contracts.
- Project 39 → cloud substrate separation and observability.
- Project 41 → world/evidence graphs and typed actions.
- Project 52 → state-machine simulation and safety gates.
- Project 54 → contradiction-aware epistemic control.
- Project 57 → multimodal/mobile physical interfaces.
- Project 58 → monitorability-aware oversight.

## 19. Implementation phases

### Phase A — Control-plane core

Implement resource schemas, capability registry, policy engine, audit events and lifecycle state machines.

### Phase B — Read-only providers

Add provider discovery, pricing, health and capability introspection with strict provenance.

### Phase C — Telephony stateful adapter

Add authorized number allocation/release and activation status with idempotency and bounded polling.

### Phase D — LPA/mobile adapter layer

Integrate the LPA device interface with platform-specific native bridges and authoritative profile readback.

### Phase E — MCP surface

Expose safe read operations and selected approved mutations with explicit risk annotations and approval states.

### Phase F — Verification/evaluation

Add simulator/replay suites, fault injection, concurrency tests, stale-state tests, secret-leak tests, adapter conformance and monitorability-aware evaluation.

## 20. Non-goals

- bypassing telecom/platform verification;
- evading network or regional restrictions;
- bulk anonymous account registration;
- covert OTP collection;
- exploiting public SMS inboxes;
- optimizing user manipulation or engagement through identity/telephony controls.

## 21. Evidence status

This project is a **source-derived engineering synthesis** from the reviewed repositories, not a statement that every external implementation is production-ready. Legacy repositories are treated as architectural lineage. Provider documentation, coverage, compatibility and service behavior must be verified independently before implementation.

---

## 2026-10-09 — batch 25: uploaded MCP, Unity, eSIM and adversarial agent sources

Uploaded 18-page private eSIM project proposes seven internal extensions 7001–7007 over Open5GS / IMS(Asterisk) / licensed SIP trunk / PSTN. The PDF makes deployment-ready claims but uses unsafe SIP identity based only on source IP, host-wide privileged containers and an unpinned latest image. A static plan must require operator spectrum/license and numbering rights, SIP subscriber identity authentication, emergency calling review, encrypted secrets, pinned images, bounded container network and verified UE test equipment. New tools/telephony_topology_gate.py with 17 unit tests checks these conditions offline; it never activates eSIM, IMS, RAN, public phone numbers, SIM profiles or calls.
