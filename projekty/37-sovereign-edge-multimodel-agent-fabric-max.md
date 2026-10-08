# 37 — Sovereign Edge & Multi-Model Agent Fabric MAX

## Status
Architecture-ready systems project.

## Cel projektu

Zbudować provider-agnostyczną warstwę wykonawczą, która płynnie łączy **edge AI, lokalną inferencję, specjalistyczne modele chmurowe, reasoning, MCP, Skills i deterministic verification**.

Projekt rozwija istniejące Projects 10, 17, 21, 24, 31, 35 i 36. Nowe materiały pokazują trzy wspólne potrzeby: prostą konstrukcję agentów z hierarchicznym promptem i knowledge base, wyspecjalizowany routing modeli oraz local/edge-first execution.

## Zasada nadrzędna

```text
ONE AGENT CONTRACT
        ↓
MANY EXECUTION PROFILES
        ↓
EDGE / LOCAL / CLOUD / COMPOUND
```

Profil wykonania może się zmienić. Polityka, autoryzacja, provenance i wymagania bezpieczeństwa pozostają niezmienne.

## 1. Architecture

```text
                         USER / OPERATOR
                                │
                         INTENT / ARTIFACT
                                │
                ┌───────────────▼────────────────┐
                │     TRUST / CONTROL PLANE      │
                │--------------------------------│
                │ intent hypothesis              │
                │ influence-risk analysis        │
                │ policy / authorization        │
                │ privacy classification        │
                │ consequence level              │
                │ latency / cost budget         │
                │ provenance / evaluation       │
                └───────────────┬────────────────┘
                                │
                      CAPABILITY DISCOVERY
                                │
                         MODEL ROUTER
                                │
          ┌─────────────────────┼─────────────────────┐
          │                     │                     │
        EDGE                  LOCAL                CLOUD
    browser/NPU            CPU/GPU/NPU       specialist/reasoning
          │                     │                     │
          └─────────────────────┼─────────────────────┘
                                │
                       COMPOUND REASONING
                                │
                         TOOL / MCP / SKILL
                                │
                          VERIFICATION
                                │
                      AUTHORIZED EXECUTION
                                │
                     OBSERVABILITY / AUDIT
                                │
                         VERSIONED RESULT
```

## 2. Agent Constitution

Materiały o tworzeniu własnych asystentów wzmacniają wzorzec hierarchicznego kontraktu agenta: Role, Mission, Workflow/Rules oraz Limitations, połączone z knowledge base, integracjami i testami. fileciteturn72file7L21-L40

MAX formalizuje go jako `AgentContract`:

```yaml
identity:
role:
mission:
workflow:
policy:
capabilities:
knowledge_sources:
state_model:
failure_policy:
verification_policy:
observability:
```

Prompt jest tylko jedną reprezentacją kontraktu. Krytyczne reguły bezpieczeństwa są egzekwowane poza naturalnym językiem modelu.

## 3. Knowledge / capability separation

Knowledge base opisuje **co agent wie**; Skills opisują **jak wykonuje procedurę**; MCP/tools opisują **do czego ma dostęp**.

```text
KNOWLEDGE ≠ CAPABILITY
SKILL ≠ AUTHORIZATION
MODEL OUTPUT ≠ EXECUTION AUTHORITY
```

Każdy workflow musi najpierw odkryć minimalny zestaw wymaganych capability.

## 4. Adaptive Model Router MAX

Routing jest wielowymiarowy:

```text
TASK CLASS
MODALITY
PRIVACY
CONSEQUENCE
CONTEXT SIZE
LATENCY
COST
AVAILABILITY
HISTORICAL QUALITY
TOOL SUPPORT
LOCAL RESOURCE PROFILE
```

Router wybiera najmniejszy wystarczający profil wykonawczy. Gdy wynik nie przechodzi weryfikacji, następuje kontrolowana eskalacja:

```text
EDGE → LOCAL → SPECIALIST CLOUD → COMPOUND / HUMAN REVIEW
```

Nie zakładamy, że konkretny dostawca jest stale lepszy. Dostarczone porównanie Claude/Gemini ma charakter historycznego materiału porównawczego; jego właściwą wartością architektoniczną jest idea specjalizacji i routingu, nie ponadczasowy ranking modeli. fileciteturn72file2L7-L20 fileciteturn72file2L46-L73

## 5. Context economics

Materiały dotyczące wydajności Gemini wskazują na context caching, batch processing, streaming, asynchroniczne wywołania, RAG i tuning jako dźwignie systemowe. fileciteturn74file0L10-L48

MAX traktuje kontekst jak zasób:

```text
CONTEXT BUDGET
├─ static knowledge
├─ task context
├─ retrieved evidence
├─ tool results
└─ conversation state
```

Router może preferować cache + tańszy model zamiast każdorazowego użycia większego modelu. Każda taka decyzja jest mierzona przez `cost_per_successful_task`.

## 6. Sovereign execution tiers

Materiały local-first proponują połączenie reproducible development, lokalnego Ollama, edge inference i cloud fallback. fileciteturn74file6L269-L283

MAX definiuje trzy podstawowe warstwy:

### Edge

Dla:

- prostych klasyfikacji;
- prywatnego preprocessingu;
- niskolatencyjnych interakcji;
- offline/near-offline UX.

### Local

Dla:

- długich sesji;
- danych prywatnych;
- lokalnego RAG;
- sustained reasoning;
- pracy bez sieci.

### Cloud

Dla:

- najtrudniejszego reasoning;
- specjalistycznych modeli;
- dużych zadań multimodalnych;
- niedostępnych lokalnie capability.

Cloud jest fallbackiem, nie domyślnym źródłem prawdy.

## 7. Model-agnostic adapter

Każdy backend implementuje wspólny kontrakt:

```ts
interface AgentModelAdapter {
  generate(input: AgentInput): Promise<ModelResult>;
  stream(input: AgentInput): AsyncIterable<ModelEvent>;
  structured(input: AgentInput, schema: Schema): Promise<unknown>;
  estimateCost(input: AgentInput): CostEstimate;
  capabilities(): CapabilityProfile;
  health(): HealthState;
  cancel(runId: string): Promise<void>;
}
```

Dzięki temu zmiana modelu nie zmienia warstwy polityki ani logiki biznesowej.

## 8. Zero-cost claim correction

Materiały marketingują lokalno-edge architekturę jako potencjalnie „zero-cost”. fileciteturn74file4L185-L195

MAX zastępuje ten slogan mierzalnym pojęciem **marginal cloud cost minimization**.

Pełny koszt obejmuje:

```text
HARDWARE
ENERGY
STORAGE
BANDWIDTH
MAINTENANCE
OBSERVABILITY
SECURITY
SUPPORT
CLOUD FALLBACK
```

Nie ma absolutnego „zero cost” bez jawnego określenia granic kosztu.

## 9. Defensive influence boundary

Materiały mentalistyczne opisują synchronizację, sugestię, oczekiwania i zakotwiczenia jako elementy performance. fileciteturn75file1L60-L79

Materiały typu `Czarne Lustro` idą dalej i opisują konstrukcję komunikatu, którego celem jest wywołanie pożądanego działania bez bezpośredniego rozkazu oraz przeniesienie odpowiedzialności na odbiorcę. fileciteturn72file4L9-L22

MAX nie implementuje takich mechanizmów jako funkcji sterowania człowiekiem. Wykorzystuje je do:

- wykrywania hidden directives;
- wykrywania consequence framing;
- wykrywania compliance pressure;
- wykrywania deceptive personalization;
- testowania odporności interfejsu i agenta.

## 10. Apeiron interaction architecture

Codex Obscura opisuje system, w którym karta ma stan spoczynku, aktywny reveal, ukryte dane UV oraz funkcję systemową. fileciteturn73file1L70-L102

MAX przenosi ten wzorzec do agentic UX:

```text
DORMANT STATE
     ↓
INTERACTION
     ↓
REVEAL / COMPUTE
     ↓
INTERPRETATION
     ↓
AUDITABLE LOG
```

Element estetyczny może być dynamiczny, ale znaczenie i stan systemowy muszą być jawne, typed i odwracalne.

## 11. MCP integration

Materiały dotyczące asystentów pokazują ścieżkę od prostego agenta do integracji CRM/database oraz testowania MCP w sandboxie. fileciteturn73file0L11-L35

MAX standardizes:

```text
TASK
 ↓
TOOL SEARCH
 ↓
MINIMAL CAPABILITY SET
 ↓
SCHEMA CHECK
 ↓
AUTHORIZATION
 ↓
SANDBOX / CALL
 ↓
RESULT VERIFICATION
```

Nie wolno traktować deklaracji w promptcie jako mechanizmu autoryzacji.

## 12. Safety / intent / influence integration

Project 35 odpowiada za intent trajectory; Project 36 za influence risk. Project 37 dodaje do nich wykonanie:

```text
INTENT HYPOTHESIS
      +
INFLUENCE RISK
      +
CONSEQUENCE
      +
PRIVACY
      ↓
EXECUTION PROFILE
      ↓
CAPABILITY POLICY
      ↓
MODEL / TOOL / HUMAN ROUTE
```

Dzięki temu system nie tylko ocenia „czy zadanie jest dozwolone”, ale także **gdzie i w jaki sposób powinno być wykonane**.

## 13. Reliability modes

Każda sesja posiada jawny tryb:

`FAST | VERIFIED | PRIVATE | OFFLINE | HIGH_ASSURANCE`

Przykład:

```text
PRIVATE + HIGH_ASSURANCE
→ local model
→ local evidence
→ deterministic verification
→ no external MCP
```

`FAST + LOW_CONSEQUENCE` może dopuścić edge model i brak ciężkiej eskalacji.

## 14. Evaluation matrix

| Dimension | Metric |
|---|---|
| Quality | verified task success |
| Routing | successful-task rate by profile |
| Cost | cost per successful task |
| Latency | p50/p95 |
| Privacy | external data egress |
| Reliability | recovery success |
| Safety | policy/influence violation rate |
| Context | cache hit / retrieval quality |
| Edge | local completion rate |
| Cloud | fallback rate |

## 15. Definition of done

Projekt jest gotowy, gdy:

1. model można wymienić bez zmiany polityki agenta;
2. edge/local/cloud są równoprawnymi execution profiles;
3. routing jest oparty na pomiarach, a nie marketingowym rankingu modeli;
4. context budget i koszty są mierzalne;
5. MCP/Skills pozostają za capability boundary;
6. local-first nie jest mylone z „bez kosztów”;
7. influence-security działa defensywnie;
8. wszystkie ścieżki mają verification, audit i recovery.

## Portfolio position

Project 37 jest warstwą wykonawczą pomiędzy **intent/control plane** a konkretnym modelem lub urządzeniem. Łączy routing Project 17, sovereign stack Project 21, agent operating fabric Project 31 oraz defensive intent/influence security Projects 35–36.

---

## Knowledge evolution — batch 16 / 2026-10-08

The 372-page GGUF Studio PDF is chat-origin design material describing Python/FastAPI/llama-cpp-python, WebSocket chat, model routing and agent tools. It is not an installable ZIP or benchmark. Extend LocalModelCapabilityTest with GGUF hash/license/revision, quantization, tokenizer, RAM/VRAM envelope, context, tool template, throughput, p95 latency, crash fallback, privacy boundaries and actual hardware receipts. No GGUF model downloaded or run.

---

## 2026-10-08 — batch 18: public web source evolution

Google Gemma 4 source 2026-04-02: E2B, E4B, 26B MoE, 31B Dense; blog states Apache-2.0. Add Gemma4Qualification {exact weight URL/hash, tokenizer/template, license, quantization, GGUF/runtime support, RAM/KV cache/CPU/GPU/NPU, thermal and latency tests, task/safety regression, rollback}. Do not extrapolate usable inference speeds or compatibility with any Android handset from a launch blog. No weights downloaded or model run.
