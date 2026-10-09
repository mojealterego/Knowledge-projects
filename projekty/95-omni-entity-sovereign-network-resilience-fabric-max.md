# Project 95 — Omni-Entity Sovereign Network & Cyber-Resilience Fabric MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Design a defensive, distributed infrastructure fabric for a sovereign digital organization/entity that can preserve identity, connectivity, service continuity and evidence integrity despite node, provider, route or infrastructure failures.

This project does **not** provide offensive cyber operations, covert intrusion, persistence, credential theft, destructive actions or evasion playbooks.

## Why this is a distinct project

Project 37 addresses sovereign edge inference/runtime delivery. Project 93 addresses hardware-rooted attestation, physical/computational work and reward eligibility. Project 95 addresses the higher-level **network and organizational resilience boundary**: distributed connectivity, cryptographic identity, failure-domain design, service continuity, anti-Sybil admission, defensive detection and recovery across independently operated infrastructure.

## Source-derived architecture

The supplied Omni-Entity report proposes mesh networking, cryptographic addressing/identity, post-quantum cryptography, QKD, honeypots and anti-Sybil mechanisms as components of a resilient distributed infrastructure. It compares Yggdrasil, B.A.T.M.A.N. and CJDNS and positions them for different network layers and scopes.

These are architectural candidates, not validated production recommendations.

## Core architecture

```text
                    ┌─────────────────────────────┐
                    │  OPERATOR / GOVERNANCE      │
                    └──────────────┬──────────────┘
                                   │ policy
                    ┌──────────────▼──────────────┐
                    │ IDENTITY + CAPABILITY PLANE │
                    │ keys / roles / admission    │
                    └──────────────┬──────────────┘
                                   │
       ┌───────────────────────────▼──────────────────────────┐
       │          SOVEREIGN NETWORK FABRIC                    │
       │                                                      │
       │  global mesh     local mesh       gateway domains    │
       │  Yggdrasil*      BATMAN*          controlled links   │
       │  CJDNS*          Wi-Fi/ethernet    independent ISPs  │
       └───────────┬───────────────┬───────────────┬──────────┘
                   │               │               │
             ┌─────▼────┐    ┌─────▼────┐    ┌─────▼────┐
             │ Edge A   │    │ Edge B   │    │ Edge C   │
             │ local AI │    │ services │    │ storage  │
             └─────┬────┘    └─────┬────┘    └─────┬────┘
                   │               │               │
                   └───────────────┼───────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │ EVIDENCE / TELEMETRY PLANE  │
                    │ logs • attestations • graph │
                    └──────────────┬──────────────┘
                                   │
                    ┌──────────────▼──────────────┐
                    │ DETECT → ISOLATE → RECOVER  │
                    │ → REPROVISION → VERIFY     │
                    └─────────────────────────────┘
```

*Protocol selection must be benchmarked against the actual topology and threat model; the source's recommendation is not treated as verified fact.

## Design pillars

### 1. Failure-domain independence

Avoid dependence on a single cloud, ISP, datacenter, DNS provider, CA or routing domain where the threat model requires continuity. Model correlated failures explicitly.

### 2. Cryptographic identity

Nodes and services receive cryptographically bound identities. Identity is distinct from authorization; possession of a key does not by itself grant consequential capabilities.

### 3. Stateful admission and anti-Sybil controls

Node admission, key rotation, revocation, reputation and anomaly state are explicit state machines. Unknown or contradictory identity state fails closed.

### 4. Resilient routing

Evaluate mesh and overlay protocols for convergence time, route stability, latency, bandwidth overhead, partition behavior, key management and operational complexity.

### 5. Defensive detection

Use honeypots, network telemetry, anomaly detection, integrity checks and evidence correlation to identify compromise. Detection output is an observation/hypothesis, not proof of attacker identity or intent.

### 6. Post-quantum migration readiness

Maintain crypto-agility so algorithms can be replaced without rebuilding the entire identity plane. QKD is treated as an optional research component, not a prerequisite.

### 7. Recovery over permanence

The objective is not to make compromise logically impossible. The objective is to minimize blast radius, preserve evidence, revoke compromised trust, restore service and independently verify recovery.

## Integration with portfolio

- **P31/P37:** sovereign agent and edge runtime foundations.
- **P54/P72:** cognitive/agent security, prompt injection and stateful assurance.
- **P59/P60:** hardware and physical trust foundations.
- **P61/P80:** OmniCore runtime, learned kernel, AI Foundry and self-evolving substrate.
- **P93:** hardware attestation and physical-work evidence at the node layer.
- **P95:** network/entity-level resilience and continuity across those components.

## Verification plan

1. Define threat model and failure domains.
2. Build isolated network testbed.
3. Benchmark candidate overlays under packet loss, node loss, partition and route churn.
4. Test key rotation/revocation and compromised-node isolation.
5. Test anti-Sybil admission using synthetic adversarial nodes.
6. Validate telemetry and evidence provenance.
7. Run disaster-recovery exercises.
8. Independently reproduce critical measurements.
9. Promote only measured configurations to architecture-ready status.

## Non-goals

- offensive intrusion tooling
- destructive cyber operations
- covert persistence or evasion
- credential theft
- unauthorized surveillance
- autonomous geopolitical or legal action
- claims of guaranteed censorship-proof or state-proof operation

## Success criteria

A configuration qualifies only when it demonstrates, in a controlled and reproducible testbed:

- measurable multi-domain failure tolerance;
- cryptographically verifiable node/service identity;
- bounded compromise blast radius;
- recoverable trust state;
- auditable evidence chain;
- verified restoration after partition or node loss.

## Evidence classification

**SOURCE-DERIVED:** protocol candidates and conceptual architecture from supplied Omni-Entity material.

**INFERRED:** separation of network/entity resilience from node-level attestation is a useful project boundary.

**UNKNOWN:** real-world geopolitical resilience, legal sovereignty, guaranteed censorship resistance and production economics.

**VERIFIED:** none until implementation and independent testing are completed.

## Batch 24 — 2026-10-09 PDF source evolution

The 117-page Komunikacja.pdf surveys IrDA, BLE, NFC, UWB, Wi-Fi Direct, LoRaWAN, Li-Fi/FSO, satellite and software messaging such as Zenoh. Keep radio/optical physical transport distinct from middleware and location services. No identification prefix, wireless packet or OUI alone proves a person's location. Prototype offline tools/communication_link_policy.py describes eight conservative choices for owner-approved device testbeds with mutual consent and no background surveillance, external publication or network scan. Actual radio throughput, range and reliability need device-level measurements; no hardware tests were performed.
