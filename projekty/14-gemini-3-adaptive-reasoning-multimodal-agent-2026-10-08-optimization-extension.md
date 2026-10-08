# P14 extension — evidence-based inference optimization (2026-10-08)

## Integration (do not replace P14 canonical specification)
Reference: `docs/knowledge-base/2026-10-08-gemini-performance-evidence.md`.
Owners: P14 runtime; P17 router, P18 Gemini agent, P27 multi-candidate reasoning.

## Contract
```yaml
InferencePolicy:
  version: "1"
  model_id: null
  task_class: fast|balanced|deep
  max_cost_usd: null
  max_latency_ms: null
  max_parallel_candidates: 1
  cache:
    enabled: false
    ttl_seconds: null
    source_fingerprint: null
  batch:
    allowed: false
  verification:
    schema_required: true
    independent_evidence: true
    max_retries: 0
```

## Safety invariants
- Discovery of supported provider API/quotas occurs at runtime; never hard-code source-PDF price or API assumptions.
- A candidate answer cannot approve its own consequential tool call.
- Cache policy accounts for confidentiality, cross-tenant separation and expiry.
- Batch outputs are correlated by idempotent request ID and independently validated.
- Deeper reasoning, candidate votes and RAG are heuristics, not proof.
- An optimization must be evaluated against a reproducible baseline for cost, latency, accuracy and failure rate.

## Proposed acceptance tests
Task set with diverse input size, modality and tool count; cache on/off; single vs batch; deep vs fast; prompt injection and provider outages; end-to-end p95 and actual billed cost. Publish evaluation manifest and confidence intervals.

State: architecture extension only. No production implementation or benchmark is claimed.
