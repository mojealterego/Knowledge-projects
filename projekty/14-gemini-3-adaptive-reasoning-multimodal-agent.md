# 14 — Gemini 3 Adaptive Reasoning & Multimodal Agent

## Status
Research specification → prototype target

## Objective
Build a model-agnostic agent shell that can route tasks across fast and deep-reasoning modes, preserve tool-call continuity, use multimodal analysis, execute bounded code for verification, and expose deterministic evaluation data.

## Source-derived principles
The supplied Gemini material describes reasoning-oriented Gemini variants, API-level control of thinking depth, thought-signature continuity across tool interactions, native multimodality, code execution, and Antigravity/MCP-oriented agent workflows. fileciteturn216file5L214-L243 fileciteturn216file6L253-L282

## Architecture

```text
User Intent
   ↓
Task Classifier
   ├── FAST / low-latency
   ├── BALANCED
   └── DEEP / high-reasoning
          ↓
Reasoning Runtime
          ↓
Tool / MCP Router
          ↓
State Manager
          ├── conversation state
          ├── tool-result state
          ├── provenance
          └── resumable execution
          ↓
Verification Layer
          ├── schema validation
          ├── code execution where justified
          ├── evidence checks
          └── evaluator
          ↓
Final Artifact
```

## Adaptive reasoning policy
Do not assign maximum reasoning to every task. Estimate complexity from required planning depth, uncertainty, tool count, multimodal density, and consequence level. Select the lowest reasoning budget expected to meet the evaluation threshold; escalate when verification fails.

## Multimodal verification
For image/video tasks, separate perception from measurement. Where supported, use executable analysis to turn visual observations into quantitative evidence rather than relying on free-form visual guesses. The supplied source explicitly proposes OpenCV/NumPy/Matplotlib-based code execution for metrology, chart extraction and visual verification. fileciteturn217file2L91-L116

## MCP integration
Treat MCP as a capability boundary, not as the reasoning engine. Each tool must expose typed input/output, authorization, side-effect classification and provenance. Keep business state server-side.

## Security
The supplied material contains recommendations for weakening safety controls. Those are excluded from the implementation design. Safety configuration must remain explicit, auditable and policy-controlled. Adversarial findings are converted into regression tests rather than bypass recipes.

## Evaluation matrix
- answer correctness
- tool selection accuracy
- reasoning-depth efficiency
- latency / cost
- multimodal extraction accuracy
- state continuity after tool calls
- failure recovery
- authorization correctness
- adversarial robustness

## Acceptance gate
A release requires passing deterministic schema tests, tool-contract tests, state-continuity tests, multimodal benchmark cases, adversarial cases and rollback/recovery tests.

## Implementation sequence
1. TypeScript/Python agent shell.
2. Model-routing abstraction.
3. State manager.
4. MCP tool adapter.
5. Bounded code-execution verifier.
6. Evaluation harness.
7. Observability.
8. UI/client.

## Evidence boundary
Claims about exact Gemini model behavior, limits and API fields must be revalidated against current official Google documentation before production deployment. This document preserves the supplied material as source input rather than asserting that every cited behavior remains current.

---

## Integracja wiedzy — 2026-10-08: adaptacyjna efektywność Gemini Pro

**Status:** SPECIFIED / NOT IMPLEMENTED; źródło: `Zwiększanie Wydajności Gemini Pro.pdf`.
**Specyfikacja rozszerzenia:** [P14 optimization extension](14-gemini-3-adaptive-reasoning-multimodal-agent-2026-10-08-optimization-extension.md); [analiza i ograniczenia](../docs/knowledge-base/2026-10-08-gemini-performance-evidence.md).

### Nowe komponenty architektury
1. `InferencePolicy` — wersjonowany kontrakt określający `task_class`, `model_id`, limity kosztu i opóźnienia, politykę cache, wsadowość i metodę niezależnej weryfikacji.
2. `ProviderCapabilityProbe` — weryfikacja faktycznie obsługiwanych opcji modelu, API, cache i batch przed wykonaniem zadania; brak hardcoded cenników z PDF.
3. `CacheBoundary` — klucz kontekstu powiązany z właścicielem, wersją promptu/modelu, TTL i poziomem poufności; brak nieuprawnionego współdzielenia między użytkownikami.
4. `BatchReconciler` — idempotentne identyfikatory zadań, obsługa częściowych błędów, timeouts, kontrola kosztu oraz odczyt rezultatów.
5. `InferenceEvaluator` — porównanie FAST/BALANCED/DEEP na niezmiennym zbiorze testowym z p50/p95, rzeczywistym wykorzystaniem tokenów, kosztami, błędami i wskaźnikami jakości.
6. `EvidenceGate` — typowany wynik, niezależna kontrola źródeł, bounded retry/escalation i zakaz traktowania odpowiedzi modelu jako autoryzacji narzędzia.

### Kryteria ukończenia
- Test kontrolny pojedynczego modelu oraz test wielowariantowy na tym samym zbiorze i wersjach danych.
- Wyraźne odróżnienie potwierdzonej funkcjonalności API od historycznej deklaracji źródłowej.
- Raport koszt/jakość/opóźnienie/błędy z odtwarzalnym identyfikatorem eksperymentu.
- Symulacja prompt injection, wygaśnięcia cache, utraty dostawcy i niepełnej odpowiedzi batch.
- Implementacja i realne benchmarki pozostają osobnym etapem; dokument nie stanowi dowodu równoważności „Pro = Ultra”.

---

## 2026-10-08 — batch 18: public web source evolution

Google Gemini Embedding 2 (2026-04-30) adds source-hashed multimodal retrieval contracts: text spans, image metadata, media timecodes, model revision, privacy and citation evidence. The public blog specifies 8192 text tokens, 6 images, 120 seconds video, 180 seconds audio, 6 PDF pages per embedding request. An **embedding model is not a generative reasoning model** and similarity is not factual proof. `tools/multimodal_embedding_intake_gate.py` performs static/offline checks only; provider access, remote-data consent and actual model calls have not been granted or run.
