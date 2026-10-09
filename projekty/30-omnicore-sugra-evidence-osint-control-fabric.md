# 30 — OmniCore Sugra Evidence, OSINT & Control Fabric MAX

## Status
Architecture-ready integration project.

## Objective
Build a governed external-data fabric for OmniCore that combines Sugra's MCP/API capabilities with the portfolio's existing research orchestration, OSINT, security, reasoning and trust layers.

The central design decision is to treat external data as **evidence with explicit provenance, freshness, quota and authorization semantics**, not as unquestioned model context.

Sugra exposes a broad endpoint catalog through MCP. Its hosted MCP deployment provides discovery, endpoint description, endpoint invocation and composed entity/time-series tools; the local package exposes the gateway surface through stdio. citeturn510648search0

## 1. System role

Project 30 becomes the external-evidence plane between the OmniCore control plane and external data providers:

```text
USER / AGENT INTENT
        │
        ▼
CONTROL PLANE
(policy / authz / reasoning budget / privacy)
        │
        ▼
SUGRA CAPABILITY BROKER
        │
        ├── search_endpoints
        ├── describe_endpoint
        ├── call_endpoint
        ├── fetch_data
        └── composed entity / time-series tools
        │
        ▼
DATA + META + SOURCE + FRESHNESS + QUOTA
        │
        ▼
EVIDENCE NORMALIZER
        │
        ▼
EVIDENCE GRAPH / KNOWLEDGE FABRIC
        │
        ├── Research Orchestrator
        ├── OSINT Graph
        ├── Compound Reasoning
        ├── Adaptive Model Router
        └── Verification / Audit
```

## 2. Why this changes the existing portfolio

Earlier portfolio projects already solve the reasoning and authorization problem. Project 30 adds a formalized **external evidence contract**.

A Sugra response uses a common `data` + `meta` envelope, while `meta.source` identifies the upstream dataset. This makes the returned metadata suitable for first-class evidence objects rather than disposable tool output. citeturn622684search3

The portfolio should therefore preserve at minimum:

```json
{
  "evidence_id": "...",
  "query": "...",
  "endpoint": "...",
  "operation_id": "...",
  "source": "...",
  "retrieved_at": "...",
  "freshness": {
    "data_time": "...",
    "stale": false,
    "partial": false
  },
  "request_cost": 1,
  "provenance": [],
  "content_hash": "...",
  "authorization_context": "..."
}
```

## 3. Discovery-first execution

Sugra's MCP catalog is intentionally large. Discovery tools operate locally against the bundled catalog, while actual data requests consume API quota. citeturn510648search0

Project 30 formalizes this into a compiler-like flow:

```text
NATURAL LANGUAGE
     ↓
CAPABILITY INTENT
     ↓
SEARCH ENDPOINTS
     ↓
DESCRIBE ENDPOINT
     ↓
VALIDATE INPUT SCHEMA
     ↓
CHECK POLICY / AUTHZ / PRIVACY
     ↓
ESTIMATE COST / LATENCY / FRESHNESS
     ↓
AUTHORIZE CALL
     ↓
EXECUTE
```

The agent must not blindly guess an endpoint when the catalog can resolve the capability.

## 4. Standard Evidence Object

All external results are normalized into a common contract:

```text
EvidenceObject {
  identity
  claim_scope
  raw_reference
  normalized_data
  source
  retrieved_at
  data_time
  freshness_state
  partiality_state
  licensing
  request_cost
  provenance
  transformation_chain
  confidence
}
```

`confidence` belongs to the consuming reasoning layer; upstream source metadata must never be silently converted into model confidence.

This preserves the portfolio doctrine that **source truth, model inference and hypothesis are separate types**.

## 5. Freshness gate

Sugra exposes freshness information in several live-data surfaces. For news, for example, the documentation describes `data_time`, `stale` and `partial` flags as signals that a consuming agent can use to decide whether the aggregate is trustworthy. citeturn622684search5

Project 30 turns that into policy:

```text
IF task requires current-state evidence
AND stale == true
    → reject as primary evidence

IF partial == true
    → mark result incomplete
    → trigger retrieval expansion or human review

IF data_time older than task freshness SLA
    → downgrade / refresh
```

Freshness thresholds are domain-specific and versioned in policy.

## 6. Quota-aware planning

Sugra quotas are daily and per-key. The documentation states that all endpoint directions share the same quota model, while some bulk endpoints can charge per item and hosted composed tools use fixed recipe costs. citeturn510648search0turn510648search10

Project 30 introduces a quota ledger:

```text
QuotaBudget {
  daily_limit,
  remaining,
  reset_at,
  estimated_cost,
  reserved_cost,
  task_budget,
  emergency_reserve
}
```

The reasoning engine may choose a cheaper evidence path before a more expensive one when both meet the evidence contract.

This connects directly to Project 27's reasoning budget controller: **data acquisition and model reasoning share one planning economy**.

## 7. Source and licensing registry

Sugra documents upstream source, tier and licensing information for research/development datasets. Attribution-licensed sources require upstream credit when republished, while some datasets carry their own terms. citeturn622684search6turn622684search7

Project 30 therefore maintains:

```text
SourceRegistry
├── provider
├── dataset
├── license
├── attribution_requirement
├── redistribution_policy
├── jurisdiction
└── effective_version
```

Evidence export must attach the applicable source/attribution metadata.

## 8. Research mode

Sugra Research is particularly aligned with Projects 15 and 19. Its documented research surface includes arXiv, Crossref, NBER, Maddison, Penn World Table and development datasets; the recommended workflow is to search literature, resolve DOI metadata, walk references/citations and retain source attribution. citeturn622684search3

Project 30 provides a research adapter:

```text
QUESTION
  ↓
SOURCE PLAN
  ↓
ARXIV / CROSSREF / NBER / DEVELOPMENT DATA
  ↓
EVIDENCE NORMALIZATION
  ↓
CITATION GRAPH
  ↓
CONTRADICTION CHECK
  ↓
SYNTHESIS
  ↓
FALSIFICATION TESTS
```

This becomes the preferred route for evidence-heavy claims where endpoint metadata is directly available.

## 9. OSINT / Net Atlas mode

Sugra Net Atlas documents network intelligence surfaces including IP, ASN, BGP, RPKI, prefix history, hijack/outage signals, Tor network directory, DNS/WHOIS/RDAP and certificate transparency. citeturn622684search8

Project 30 integrates these capabilities with Projects 15, 19 and the OSINT-oriented project family, but keeps the execution boundary explicit:

```text
PUBLIC / AUTHORIZED DATA
        ↓
PASSIVE ANALYSIS
        ↓
ENTITY / INFRASTRUCTURE GRAPH
        ↓
RISK HYPOTHESIS
        ↓
VERIFICATION
```

Active interactions with third-party infrastructure remain outside the default read-only evidence path and require explicit policy authorization.

## 10. Entity resolution

The hosted Sugra MCP surface exposes `resolve_entity`, `get_snapshot` and `get_timeseries` as composed agent tools. These can reduce multi-call orchestration for canonical entities and current snapshots. citeturn510648search0

Project 30 wraps them with an identity contract:

```text
Free-text entity
      ↓
Ranked candidate set
      ↓
Disambiguation
      ↓
Canonical entity ID
      ↓
Snapshot / Timeseries
```

Ambiguous entity matches must never be silently promoted to authoritative identity.

## 11. Security boundary

The Sugra MCP server is a third-party remote capability surface. The project therefore applies the same rules used elsewhere in the portfolio:

```text
EXTERNAL SERVER
     ↓
TRUST-DOMAIN IDENTIFICATION
     ↓
TOKEN / SECRET ISOLATION
     ↓
ALLOWED-TOOL POLICY
     ↓
INPUT VALIDATION
     ↓
EXECUTION
     ↓
OUTPUT SANITIZATION
     ↓
PROVENANCE
```

The broader MCP security doctrine already treats remote servers as external trust domains and requires careful handling of prompt injection, sensitive data sharing, approvals and server trust. fileciteturn246file3L160-L190

For browser-hosted agent systems, secrets for remote MCP services remain server-side rather than embedded in browser code. fileciteturn246file5L287-L289

## 12. Prompt-injection containment

External data is never converted directly into instructions.

```text
SUGRA OUTPUT
   ↓
DATA / INSTRUCTION SEPARATION
   ↓
PROVENANCE ATTACHMENT
   ↓
UNTRUSTED EVIDENCE CONTEXT
   ↓
REASONING
```

A string returned by a news feed, research record, website-derived endpoint or other external source can support a claim but cannot acquire capability authority merely by containing imperative text.

This is consistent with the portfolio's existing injection model and the supplied research showing that data-plane manipulation can cause control-plane misclassification when boundaries are weak. fileciteturn250file1L33-L43

## 13. Sugra + Compound Reasoning

Project 27's ensemble engine can use independent data acquisition strategies:

```text
Solver A → Sugra direct endpoint
Solver B → Sugra alternate source
Solver C → external web research
Solver D → cached evidence
Solver E → adversarial verification
```

Agreement is weighted by evidence independence. Multiple agent paths consuming the same Sugra source do not count as independent corroboration.

## 14. Sugra + Prompt Compiler

Project 24 can compile a research goal into a typed data-acquisition program:

```text
Intent
  → evidence requirements
  → freshness SLA
  → acceptable sources
  → Sugra capability search
  → endpoint schema
  → quota budget
  → retrieval plan
  → evidence contract
```

This converts API usage from ad-hoc tool calling into a reproducible research program.

## 15. Sugra + Model Router

Project 17 may route the reasoning stage after evidence acquisition rather than routing every data query through the same model.

Example policy:

```text
simple retrieval / normalization → fast model
entity disambiguation → stronger model if needed
high-consequence synthesis → compound reasoning
contradictory evidence → adversarial escalation
```

The data plane and model plane remain separately observable.

## 16. Observability

Every call should emit structured telemetry:

- task_id;
- evidence_id;
- endpoint / operation_id;
- source;
- latency;
- request cost;
- quota before/after;
- freshness state;
- partiality state;
- error class;
- policy decision;
- downstream use;
- citation/export status.

Failures should be typed rather than collapsed into generic errors. Sugra documents structured error classes such as `upstream_timeout`, `upstream_connect_error` and `upstream_transport_error`. citeturn510648search0

## 17. Runtime modes

### Mode A — Evidence Retrieval
Read-only external data acquisition with full provenance.

### Mode B — Research
Multi-source literature/data synthesis with citation graph.

### Mode C — Monitoring
Scheduled freshness checks and signal detection.

### Mode D — OSINT Graph
Public-source network/entity enrichment under explicit scope.

### Mode E — High-Consequence Review
Evidence collection plus mandatory stronger verification and, where required, human approval.

## 18. Failure handling

```text
401 → credential / session repair
403 → policy or route restriction; do not retry blindly
429 → quota-aware backoff / alternate evidence path
5xx → bounded retry with exponential backoff
stale → refresh or downgrade evidence
partial → mark incomplete and expand retrieval
ambiguous entity → disambiguate before continuation
schema mismatch → stop and re-describe endpoint
```

The distinction between authentication, quota and route restrictions is explicitly documented by Sugra and should be encoded in the adapter rather than left to model interpretation. citeturn622684search1

## 19. Definition of done

Project 30 is complete when:

1. Sugra hosted MCP is reachable through a dedicated capability adapter;
2. endpoint discovery precedes uncertain endpoint invocation;
3. every retrieved datum can be traced to source and retrieval time;
4. freshness and partiality influence evidence admissibility;
5. quota is represented in the planning budget;
6. source licensing metadata survives export;
7. remote MCP remains a separate trust domain;
8. external content cannot escalate capability authority;
9. evidence objects integrate with Projects 15, 19, 24, 25, 26 and 27;
10. the same task is reproducible from a versioned retrieval plan.

## Portfolio position

Project 30 becomes the **external evidence and data plane** for the OmniCore portfolio. It turns Sugra from a convenient MCP connection into a governed subsystem that supplies structured evidence to research, OSINT, reasoning, routing and verification layers while preserving provenance, freshness, quota economics and security boundaries.

## Batch 24 — case-evidence intake boundary

The 10 uploaded PDFs include overlapping Seeker OSINT drafts, a hardware-awareness report and a large communications glossary. The admissible engineering pattern is provenance-preserving evidence intake and a reviewable case record, not a person-location engine. Extend each case item with authorized scope, recorded source hash, source rights, acquisition timestamp, reliability class, redaction, independent verification and retention constraints. Claims about a device, person or coordinates must be separately corroborated; inferred identifiers cannot be promoted to direct observations. An executable non-network preflight now exists at tools/seeker_evidence_scope_gate.py with tests. It does not conduct searches, traffic interception or contact analysis. Document hashes/pages and OSINT-version similarities are in docs/UPLOADED-PDF-INGESTION-2026-10-09-BATCH-24.md.

---

## 2026-10-09 — batch 25: evidence-qualified new uploads

Missing-person OSINT PDF (15 pages) proposes inferred whereabouts based on MSISDN/phone status. P30 evidence schema must separate publicly verifiable facts, owner-provided logs and unsubstantiated inference, with authorization, timestamp, source hash, alternate explanations, witness confidence and privacy-limited retention. Subscriber activity can reflect a device/account, not a person's location or proof of life. No intrusive HLR/SS7 queries, impersonation or bulk private-account scraping is part of the implementation. Data from the source document remains out of GitHub; only provenance and safe engineering controls are retained.
