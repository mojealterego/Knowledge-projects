# 24 — Agentic Prompt Compiler & DSL

## Status
Research + architecture-ready project.

## Objective
Turn natural-language system intent into a typed, testable prompt/program specification. Treat prompting as a compilation pipeline instead of a string-editing exercise.

## Source-derived idea
The supplied prompt-engineering research frames a strong prompt as the combination of structural precision, cognitive architecture and iterative evaluation, with examples, decomposition, meta-prompting and constraint-based methods as controllable dimensions. fileciteturn219file1L17-L31 fileciteturn226file6L256-L292

## DSL

```text
AgentSpec
├─ objective
├─ inputs
├─ context_policy
├─ capabilities
├─ reasoning_policy
├─ constraints
├─ evidence_policy
├─ output_schema
├─ failure_policy
├─ evaluation_suite
└─ deployment_policy
```

## Compiler passes

```text
intent
  ↓
semantic normalization
  ↓
requirements extraction
  ↓
constraint generation
  ↓
capability binding
  ↓
prompt/program synthesis
  ↓
static linting
  ↓
benchmark execution
  ↓
promotion
```

## Static analysis

Detect:

- contradictory instructions;
- ambiguous ownership of tasks;
- missing output contracts;
- unsupported claims;
- redundant prompt fragments;
- unsafe capability assumptions;
- context bloat;
- evaluation gaps.

## Optimization

Prompt variants should be generated as controlled transformations with measurable diffs. Frozen holdout suites prevent overfitting. Prompt changes require benchmark evidence before promotion.

## Runtime integration

Compiled specs map to:

- OpenAI Agents SDK agents;
- MCP tool declarations;
- policy/approval rules;
- model routing policy;
- sandbox requirements;
- observability metadata.

This creates a common intermediate representation across different model providers and agent frameworks.

## Definition of done

A prompt is production-ready only when its compiled specification has a valid schema, deterministic tool contract, passing regression suite, security review and reproducible version identifier.

---

## Integracja wiedzy — batch 11 (2026-10-08): prompt/model runtime provenance

Źródła: `Zaawansowane Wykorzystanie Sztucznej Inteligencji.pdf` (12 stron) i `(1).pdf` (14 stron), ≈90,6% podobieństwa tekstowego — dwa warianty jednego zbliżonego opracowania.

```yaml
ModelRuntimeSpec:
  provider_model: null
  model_revision: null
  prompt_hash: null
  few_shot_set_id: null
  sampling:
    temperature: null
    top_p: null
    top_k: null
  retrieval_refs: []
  allowed_capabilities: []
  output_schema_ref: null
  holdout_eval_id: null
```

**Nowe przebiegi kompilatora:** walidacja dostępności opcji sampling u dostawcy, kontrolowane CoT/ToT/few-shot/Chain-of-Table warianty, pomiar błędów/kosztu/opóźnień, niezmienna próba walidacyjna, wyraźna granica untrusted context i rights/authorization. LangGraph/AutoGen/CrewAI pozostają adapterami orkiestracji wymagającymi niezależnej polityki narzędzi. Opisywana w PDF abliteration jest zachowana jako zagadnienie odporności/adversarial research, a nie automatyczny wzrost bezpieczeństwa ani pozwolenie na obejście zabezpieczeń. **Status: specyfikacja, bez benchmarków**.
