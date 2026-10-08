# 29 — Sugra Agentic Data & Research Fabric MAX

## Status
Architecture-ready integration project.

## Objective
Turn Sugra into a first-class data/research substrate for the agent portfolio: a governed MCP/API fabric for finance, macro, entity, internet infrastructure, news, earth and research data, with explicit provenance, freshness, quotas, endpoint discovery and authorization boundaries.

Sugra's current documentation describes a single API surface with 1,569+ endpoints backed by 163 upstream sources across seven data directions. Responses use a consistent `data` + `meta` envelope, while system endpoints such as health/about/services/sources are public. citeturn425504search1

## Architectural role

Sugra is not treated as "one giant tool". It becomes a **data capability fabric** behind the portfolio control plane.

```text
USER / AGENT INTENT
        ↓
RESEARCH / TASK PLANNER
        ↓
SUGRA CAPABILITY ROUTER
        ├─ Finance
        ├─ Macro
        ├─ Entity
        ├─ Net Atlas
        ├─ News
        ├─ Earth
        └─ Research
        ↓
ENDPOINT DISCOVERY
        ↓
AUTHORIZED DATA CALL
        ↓
PROVENANCE + FRESHNESS + QUOTA
        ↓
EVIDENCE GRAPH
        ↓
SYNTHESIS / DECISION
```

## 1. MCP-first integration

Sugra exposes a hosted MCP endpoint at `https://app.sugra.ai/mcp` using Streamable HTTP, with OAuth sign-in or API-key authorization. Its hosted surface currently exposes gateway/discovery tools plus composed entity tools such as `resolve_entity`, `get_snapshot` and `get_timeseries`; the local package is available through `sugra-api-mcp` and stdio. citeturn425504search0

The portfolio therefore supports two deployment modes:

### Hosted mode

Use for ChatGPT, remote agents and zero-install execution.

```text
Agent
  → MCP client
  → https://app.sugra.ai/mcp
  → Sugra
```

### Local mode

Use for desktop/IDE agents, self-hosting, version pinning and local control.

```text
Agent
  → stdio MCP
  → sugra-api-mcp
  → Sugra API
```

The decision is a deployment policy, not business logic. citeturn425504search0

## 2. Tool discovery instead of loading the entire catalog

Sugra's MCP design separates catalog discovery from data requests. `search_endpoints` and `describe_endpoint` inspect the bundled endpoint catalog locally, while `call_endpoint` executes a selected operation. `fetch_data` provides a one-step natural-language route for finding and calling an endpoint. citeturn425504search0

This maps directly onto the portfolio's deferred-tool doctrine:

```text
TASK
 ↓
SEARCH CAPABILITY CATALOG
 ↓
DESCRIBE EXACT OPERATION
 ↓
VALIDATE SCHEMA / COST / LIMITS
 ↓
CALL ONLY REQUIRED OPERATION
```

Large tool surfaces should not be blindly exposed to every reasoning step. The OpenAI MCP guidance likewise recommends deferred tool loading for MCP servers with large function sets. fileciteturn246file3L140-L178

## 3. Endpoint as typed capability

Each Sugra operation is normalized internally to:

```json
{
  "operation_id": "...",
  "method": "GET",
  "path": "...",
  "input_schema": {},
  "output_schema": {},
  "source": "...",
  "freshness_policy": {},
  "quota_cost": 1,
  "side_effect": "read_only",
  "authorization_scope": "data.read"
}
```

The agent never reasons from an unstructured description when an endpoint schema is available.

## 4. Provenance becomes first-class evidence

Sugra's standard response envelope separates result data from metadata, and endpoint documentation exposes source information and, for many surfaces, freshness-related metadata. citeturn425504search1turn270359search0

The agent fabric therefore promotes provenance into the evidence graph:

```text
DATA POINT
  ↓
SOURCE
  ↓
OBSERVATION TIME
  ↓
FETCH TIME
  ↓
TRANSFORM
  ↓
MODEL INTERPRETATION
```

A model-generated conclusion is never recorded as though it were a source observation.

## 5. Freshness contracts

Freshness is a query-time property, not a generic label.

Each evidence object stores:

- source;
- observed timestamp;
- retrieval timestamp;
- freshness budget;
- stale flag;
- cache state;
- point-in-time/as-of semantics where supported.

For compliance workflows, Sugra explicitly uses source-coverage checks and fail-closed behavior: missing or stale required sanctions corpora produce `503`, never a fabricated `clear`. citeturn270359search0

This becomes the portfolio standard for high-consequence data:

```text
REQUIRED DATA MISSING / STALE
        ↓
NO SILENT FALLBACK
        ↓
ESCALATE / RETRY / HUMAN REVIEW
```

## 6. Research orchestration

Sugra materially strengthens Project 15/19 research architecture by supplying machine-readable primary and curated datasets.

Example pattern:

```text
QUESTION
 ↓
SOURCE PLAN
 ↓
Sugra CATALOG SEARCH
 ↓
ENDPOINT DESCRIPTIONS
 ↓
PARALLEL DATA COLLECTION
 ↓
CROSS-SOURCE CHECK
 ↓
DERIVED COMPUTATION
 ↓
EVIDENCE GRAPH
 ↓
FALSIFICATION / COUNTERCHECK
 ↓
REPORT
```

The documented macro-regime recipe is a concrete model: multiple official datasets are combined, then reduced into separate growth, inflation and yield-curve axes, with independent cross-checks. citeturn270359search1

## 7. Entity resolution as an agent primitive

The hosted MCP surface includes composed entity tools designed to resolve free-text entities and produce bounded snapshots or time series. citeturn425504search0

Portfolio abstraction:

```text
raw mention
   ↓
canonical entity
   ↓
identifier set
   ↓
source-linked observations
```

This prevents downstream agents from repeatedly solving the same entity-identification problem.

## 8. Compliance and high-consequence workflows

Sugra's compliance recipe establishes a strong pattern for agent architecture: coverage validation first, identity resolution second, screening third, human review for `review`, and blocking/escalation for `hit`. Each verdict retains an audit identifier and provenance. citeturn270359search0

For the portfolio this becomes a generic **high-consequence workflow contract**:

```text
coverage check
 → identity/context resolution
 → policy evaluation
 → evidence attachment
 → decision
 → approval / block / escalate
```

A data API's result is a signal, not authorization.

## 9. Rate-limit-aware planning

Sugra documents per-key daily quotas, UTC reset semantics, and structured handling for `429` responses. The same API key model is used across the data surface. citeturn548344search2turn548344search6

The agent router must therefore include quota state in planning:

```text
expected_information_gain
        /
remaining_quota + latency + cost
```

The planner should prefer fewer high-value requests, batched endpoints where appropriate, cached catalog metadata and composed endpoints when their fixed recipe cost is lower than reproducing the same chain manually. Sugra documents explicit per-item bulk costs and fixed costs for composed tools. citeturn425504search0

## 10. Authentication and secret handling

Direct Sugra API data requests require `x-api-key`; the documentation explicitly states that API keys must not be placed in browser JavaScript, mobile binaries or public repositories and should instead be loaded from environment variables or a secret store. citeturn548344search2

Portfolio rule:

```text
browser/mobile
      ✕
long-lived API key
      ✕
public repo
      ✕
backend / secret store
      ✓
```

For hosted MCP, the client authenticates through the supported hosted flow rather than embedding long-lived secrets in client UI code. citeturn425504search0

## 11. Security boundary

Sugra remains an external data service and therefore belongs behind the same MCP trust boundary as every third-party capability.

```text
UNTRUSTED / EXTERNAL DATA
        ↓
SOURCE + PROVENANCE
        ↓
NORMALIZATION
        ↓
MODEL CONTEXT
        ↓
POLICY
        ↓
CAPABILITY BROKER
```

Tool output is data, not authority. This is consistent with the portfolio's MCP security doctrine and with the OpenAI guidance that remote MCP servers can expose or act on sensitive data and should be reviewed carefully. fileciteturn246file4L275-L289

## 12. Suggested capability taxonomy

### Research

- macro regime reconstruction;
- financial/company analysis;
- literature and research discovery;
- news/event synthesis;
- network intelligence;
- environmental and weather context.

### Operations

- weather-aware alerts;
- grid/energy monitoring;
- compliance screening;
- internet-outage monitoring;
- market/event monitoring.

### Agent infrastructure

- entity resolution;
- time-series retrieval;
- source/endpoint discovery;
- freshness checks;
- evidence packaging.

## 13. Portfolio integration matrix

| Portfolio component | Sugra role |
|---|---|
| Project 15 Research Orchestrator | primary external evidence fabric |
| Project 19 Omniscience | multi-source verification substrate |
| Project 24 Prompt Compiler | typed data-capability target |
| Project 25 Control Tower | capability registry + quota metadata |
| Project 26 Trust Lab | external-data security boundary |
| Project 27 Compound Reasoning | retrieval branches + evidence diversity |
| Project 28 AI Foundry | external technical research source |
| New agents | domain-specific data plane |

## 14. Definition of done

Sugra integration is complete when:

1. the portfolio can discover an endpoint without loading the full tool catalog;
2. every selected endpoint is represented as a typed capability;
3. source, freshness and retrieval metadata survive into the evidence graph;
4. quotas influence planning;
5. high-consequence workflows fail closed on missing required data;
6. secrets never enter client bundles or repositories;
7. external tool output cannot grant authority;
8. research agents can combine Sugra evidence with independent sources and explicitly represent disagreement.

## Position in the portfolio

Project 29 becomes the **external data and evidence fabric** for the agent ecosystem. It materially upgrades Projects 15, 19, 24, 25, 26 and 27 and gives the portfolio a standardized path from natural-language research intent to typed, provenance-aware, quota-aware data acquisition.

---

## 2026-10-08 — batch 18: public web source evolution

Gemini Embedding 2 maps text/images/audio/video/PDF into shared representation (Google Developers Blog 2026-04-30; official model card). Add CrossModalRetrievalEvidence {source_sha, rights, modality, media_offset, embedding_model_revision, vector_dimensions, task_prefix_version, indexed_at, retrieval_score, original_citation_span, data_sensitivity}. Evaluate Polish Recall@k, adversarial false citations and bitemporal contradictions against frozen text-only RAG before service selection. Passing new offline input policy tests is **not** a real embedding request or proven retrieval improvement.

---

## 2026-10-09 — batch 19: agent marketplace, MCP and consent-based games

New source-driven adapters: SAKH research corpus (hybrid dense/sparse with source references), Traveler.md (OAuth-scoped profile/trip memory, **not a booking or inventory search provider**), plus Google Gemini Drops release metadata (time-sensitive vendor features). `EvidenceProviderRecord`: source URL, owner/version, API read/write scope, rights, language, timestamps, citation confidence, tenant/privacy scope, independent corroboration. Never return hotel/restaurant real-time availability from a travel-memory tool; never treat publisher-curated religious/political documents as globally validated normative truth. The three direct registry URLs for Structura, Apricot and Traveler were not themselves retrievable; supporting official/vendor docs were inspected instead.
