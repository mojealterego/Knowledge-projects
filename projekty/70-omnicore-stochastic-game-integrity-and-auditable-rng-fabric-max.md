# Project 70 — OmniCore Stochastic Game Integrity & Auditable RNG Fabric MAX

## Mission
Create a provider-neutral integrity layer for lottery, casino and game-like stochastic systems. The project extracts the useful engineering content from the supplied probability and gambling-system reports while excluding compulsion optimization and exploitation of vulnerable users.

## Architecture
```text
GAME / LOTTERY SPEC
      ↓
STATE-SPACE MODEL
      ↓
RNG / RANDOMNESS SERVICE
      ↓
RULE ENGINE
      ↓
ACCOUNTING / PAYOUT
      ↓
RECONCILIATION
      ↓
STATISTICAL AUDIT
      ↓
EVIDENCE LEDGER
```

## Randomness contract
```yaml
RandomnessContract:
  generator_id:
  version:
  entropy_source:
  seed_policy:
  isolation_boundary:
  reproducibility_mode:
  distribution_tests:
  failure_behavior:
```

The source IMDP report presents strong claims about deterministic predictability of physical lottery machines and PRNG reconstruction. Project 70 treats these as hypotheses requiring controlled measurement. Chaotic determinism alone does not imply practical predictability when initial conditions are unobservable or measurement uncertainty dominates.

## Mathematical audit
Evaluate:
- sample space;
- probability mass/density;
- expected value;
- variance and tail risk;
- independence assumptions;
- calibration;
- distributional drift;
- anomaly significance;
- reproducibility.

## Casino-system integrity
The supplied architecture identifies player account/wallet, game engine, RNG, payment and supporting services as separable components in a scalable distributed system. Project 70 strengthens the separation so critical randomness cannot be silently altered by presentation, promotion or personalization services.

## Replay
```text
RULESET HASH
+ RNG VERSION
+ SEED / INPUT ENTROPY RECORD
+ ACTION TRACE
+ OUTPUT TRACE
        ↓
REPLAY
        ↓
EXPECTED / OBSERVED
        ↓
STATISTICAL DIFFERENCE
```

## Tamper and anomaly detection
Track code/version changes, distribution drift, unusual correlation, state divergence, payout reconciliation and unexplained seed/entropy changes. An anomaly becomes an investigation signal, not proof of fraud.

## Responsible design boundary
Forbidden objectives:
- compulsive-session optimization;
- loss chasing;
- biometric targeting;
- vulnerability targeting;
- hidden reward manipulation;
- covert persuasion.

Allowed objectives:
- fairness;
- transparency;
- reproducibility;
- responsible limits;
- accessibility;
- reliability;
- auditability.

## Novel contribution
Project 70 creates a **Stochastic Integrity Plane** reusable across games, simulations and probabilistic services, so the same RNG/rules/audit discipline can be applied without inheriting the behavioral-manipulation objectives of the source material.

---

## Knowledge evolution — batch 15 / 2026-10-08: virtual-credit ledger data integrity

Source: 24-page defensive reading of `Analiza narzędzi do manipulacji żetonami_260402_174419.pdf`. Client-side localStorage, WebStorage, React state and request/response copies are **untrusted representations**, even if a UI says "balance updated". No instructions, tooling or endpoints for unauthorized third-party credit modification are accepted.

`CreditEvent` should bind account_id, trusted_issuer, event_id, idempotency_key, signed amount/currency, prior committed version, server timestamp, audit hash and fraud-policy verdict. Atomic server-side balance changes, concurrency/race tests, replay/duplicate denial, nonnegative constraints, reconciliation against payment/refund records and external entitlement checks are mandatory. Browser mutations can be used **only as authorized QA fixtures** to ensure they do not change true credits. P72 owns cross-framework action authorization; P56 owns commercialization/reconciliation. A spec does not constitute a payment-safe deployed ledger.
