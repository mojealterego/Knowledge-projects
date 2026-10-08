# Gemini Pro performance engineering — source-derived research, 2026-10-08

Source: `Zwiększanie Wydajności Gemini Pro.pdf` (18 pages extracted locally, technical whitepaper).
Related sources: `Wyłączanie Ograniczeń Modelu Gemini.PDF`, previously summarized in Iteration 16.
Owners: P14 Adaptive Reasoning, P17 Model Router, P18 Gemini Cognitive Agent, P27 Compound Reasoning.

## Evidence boundary
The source compares Pro/Flash/Ultra and describes Gemini 3 controls, implicit/explicit context caching, batch API, adaptive thinking, CoT/ToT, self-consistency, LangGraph, RAG, fine-tuning, media-resolution, retry and budget strategies. The material is a **proposal**, not independent proof that Pro equals or outperforms Ultra. Concrete pricing (including a claimed 90% cached-input discount), API parameter names, supported fine-tuning paths, quotas, model versions and latency promises require confirmation against the *current provider API and tariff* before coding.

## Reusable engineering model
```text
Typed task + quality target + privacy constraint + cost/latency budget
  -> capability discovery / compatible provider model
  -> routing decision (fast/balanced/deep) with bounded budget
  -> relevant RAG evidence, cache eligibility and TTL
  -> tool-bound reasoning or offline batch fan-out
  -> schema validation / independent verifier
  -> sufficient evidence? return result : bounded retry/escalate/fail
  -> cost + accuracy + latency + tool trace and audit record
```

## Optimization levers, not bypasses
1. **Prompt/context caching:** track stable input prefix fingerprint, model, ownership, TTL, eviction and data sensitivity; compare actual cache hit and billed usage. Context caching reduces redundant work only if platform supports it and economics justify it.
2. **Batch mode:** asynchronous high-volume, noninteractive tasks; preserve per-item IDs, idempotency keys, ordering independence, partial failures and reconciliation.
3. **Reasoning effort:** choose task-tier policies empirically. Never promise deeper thinking will resolve an inherently unanswerable question.
4. **Self-consistency / critiques:** independent candidate generation, evidence-aware re-ranking and disagreement logging. Correlated errors mean consensus is not truth.
5. **RAG:** ingestion, chunk lineage, retrieval tests, counterevidence and time-aware source freshness. Retrieval output remains untrusted input, not executable instructions.
6. **Agents/tools:** permission broker outside the model; scope, approval, sandbox, structured args, postcondition verification and timeouts.
7. **Media:** downsampling policy with a measurable regression suite for salient details before reducing resolution.
8. **Fine-tuning / distillation:** evaluate task-specific performance versus RAG and simpler prompt/schema improvements; document data rights and safety regression.

## Evaluation matrix
| Dimension | Measured evidence | Required gate |
|---|---|---|
| Quality | task-level gold-set scoring, false positives, task completion | baseline comparison + confidence interval |
| Cost | billed input/output/cache tokens, batch job costs | real provider invoices, not doc estimates |
| Latency | p50/p95 end-to-end and each stage | same workload and environment |
| Robustness | prompt injection, tool failure, retry storms, stale RAG | independent authorization test |
| Privacy | source retention, cache reuse scope, model transfer | explicit policy/consent |
| Reproducibility | model/version, prompt hash, dataset, time | fully versioned run manifest |

## Acceptance criteria for P14/P17/P18/P27
- Schema `InferencePolicy{task_class,model_id,reasoning_tier,max_cost,max_latency,cache_policy,batch_mode,verify_policy}` versioned.
- A/B test versus single-model baseline with tracked resource consumption.
- No statement of "Ultra-level" parity unless benchmarked on an appropriate held-out corpus.
- No provider quota, limit or safety bypass implied by the optimization.
- No deployment state is inferred from this research artifact.
