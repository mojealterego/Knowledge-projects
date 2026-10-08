# P126 — IBM Cloud Cognitive Game Backend Reference

**Status:** SOURCE_REVIEWED → OFFLINE FUNCTIONAL REFERENCE / CLOUD DEPLOYMENT NOT VERIFIED
**Distinct scope:** game state, NPC answer lifecycle, server-authoritative transitions, Cloudant _rev optimistic concurrency, IBM identity integration and deployment proof. Related **P86** creates game clients; **P99** handles game LiveOps; **P121** infrastructure/SRE; **P126** owns *server game-state correctness and cloud-adapter qualification*. The 26-page `Architektura Backendu IBM Cloud.PDF` is its only direct input in batch 16.

## Source architecture
```text
AUTHENTICATED PLAYER ACTION (INTENT NOT CLIENT-SUPPLIED BALANCE)
  → SERVER VALIDATION + AUTHORIZATION
  → READ PLAYER DOCUMENT AND _rev
  → ASYNC Watsonx NPC PROPOSAL (NEVER HOLD DB LOCK WHILE GENERATING)
  → DOMAIN POLICY / BOUNDED RETRY
  → Cloudant _rev COMPARE-AND-SWAP
  → 409 CONFLICT => READ AGAIN AND RECOMPUTE AGAINST CURRENT STATE
  → AUDIT RECEIPT + IDEMPOTENCY KEY
  → RESPONSE / TRUSTED READBACK
```
Source also proposes Go Clean Architecture, IBM Code Engine, Cloud Object Storage and JWT/JWKS validation. **Do not implement JWT validation from a generic source example without validating exact IBM issuer/product/audience/alg/key rotation**, especially IBM IAM versus App ID differences. `_rev` is concurrency control, not a transaction across multiple documents. The report's observed 1–3 second model latency and cost claims are unverified.

## Actual working reference
`state_reference.py` is a pure stdlib Python **in-memory single-process** emulator of revision checks, authenticated player scope, idempotency on duplicate event IDs and server-authoritative energy/XP actions. The same contents are tested by `tools/test_ibm_game_state_reference.py` via a stdlib loader. It is **not** connected to IBM Cloud or Watsonx and does not implement Go or remote authentication.

**Prototype security invariant:** the client submits an allowlisted action and bounded quantity. It cannot submit an authoritative final balance or modify another player's state; stale revision fails closed rather than overwriting intervening state.

## Production backlog
1. Pin licensed IBM Go SDK, Cloudant and Watsonx versions and verify current official IAM/JWKS claims; no hard-coded service credentials.
2. Implement Go `domain/usecase/infra` adapters with conditional Cloudant `_rev` updates and durable event IDs, multi-document consistency rules and bounded conflict retries. Avoid concurrent writes during model inference.
3. Add independent signature verification, TLS, audience/issuer restrictions, rate limits, input and tool-safety checks, circuit breaker on Watsonx and neutral NPC fallback.
4. Record IBM Code Engine deployment and service cost evidence; resource provisioning requires explicit consent. CI should test race/idempotency, authorization and replay.
5. Execute real integration tests only in an authorized isolated IBM project.

[Source ledger](../../docs/KNOWLEDGE-INGESTION-2026-10-08-BATCH-16.md) · [Evaluation notes](../../docs/knowledge-base/2026-10-08-dgm-gguf-ibm-evaluation.md).
