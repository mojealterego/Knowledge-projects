# Project 47 — OmniCore Portfolio Integrity & Project Registry MAX

## Mission
Create a canonical machine-readable registry for the entire OmniCore project portfolio so duplicate concepts, conflicting numbers, successor/alias relationships, evidence lineage and integration dependencies are explicit rather than implicit.

## Problem solved
The portfolio contains legitimate artifacts that evolved over time. Numeric labels alone are therefore insufficient. Project 47 makes identity a **versioned graph with deterministic reconciliation rules**.

## Canonical identity model

```yaml
ProjectIdentity:
  canonical_id:
  title:
  aliases: []
  historical_ids: []
  status:
  lifecycle:
  supersedes: []
  superseded_by: []
  derived_from: []
  depends_on: []
  related_to: []
  canonical_artifact:
  content_sha:
  last_verified_commit:
  evidence_refs: []
  owner_scope:
```

## Reconciliation algorithm

```text
REPOSITORY INVENTORY
        ↓
PROJECT CANDIDATE EXTRACTION
        ↓
TITLE / ID NORMALIZATION
        ↓
SEMANTIC SIMILARITY
        ↓
CONTENT / LINEAGE COMPARISON
        ↓
CANONICAL CANDIDATE
        ↓
EXPLICIT COLLISION RESOLUTION
        ↓
REGISTRY UPDATE
        ↓
REFERENCE INTEGRITY CHECK
```

Semantic similarity is a discovery signal, not authority. Canonicalization requires explicit lineage evidence.

## Identity invariants

1. A numeric label is not sufficient to establish identity.
2. One canonical project may have historical aliases.
3. Renamed or superseded projects retain lineage.
4. Two distinct artifacts with the same historical number never silently overwrite each other.
5. A canonical artifact has a stable path and verifiable content identity.
6. Every project declares lifecycle status.
7. References to a project resolve through `canonical_id` before mutation.
8. Historical artifacts remain distinguishable from current authority.
9. Registry updates themselves are versioned and auditable.
10. A failed or ambiguous reconciliation blocks destructive rename/delete decisions.

## Current canonical family

```text
37  CogniSync Professional
38  Sovereign Edge AI
39  OmniCore Alibaba Cloud Agent Runtime & Cloud Fabric
40  OmniCore Agentic Development & Visual Intelligence Fabric
41  OmniCore Repository Intelligence & Multimodal Action Fabric
42  Open Creator Layer
43  Influence Literacy & Human Agency Lab
44  AI Content Product Studio
45  OmniCore Agentic Content & Commerce Factory
46  CogniSync Open Creator Influence & Content Nexus
47  OmniCore Portfolio Integrity & Project Registry
48  OmniCore Grand Challenge & All-Source Intelligence Foundry
```

This family is the current canonical sequence. Historical duplicate numeric names are retained only as lineage artifacts.

## Machine-readable registry record

```yaml
ProjectRecord:
  canonical_id: 48
  title: OmniCore Grand Challenge & All-Source Intelligence Foundry MAX
  status: architecture
  canonical_artifact: projekty/48-omnicore-grand-challenge-all-source-intelligence-foundry-max.md
  aliases: []
  historical_ids: []
  derived_from:
    - 15
    - 19
    - 26
    - 27
    - 30
    - 32
    - 34
  depends_on:
    - 47
  validates:
    - evidence lineage
    - strategy concretization
    - grand challenge research loop
```

## Project graph

```text
PROJECT
  ├── ALIAS
  ├── SUPERSEDES
  ├── DERIVED_FROM
  ├── DEPENDS_ON
  ├── RELATED_TO
  ├── IMPLEMENTED_BY
  ├── VALIDATED_BY
  ├── DOCUMENTED_BY
  └── EVIDENCED_BY
```

Every edge has:

`source_ref + observed_at + confidence + registry_version`.

## Portfolio compiler

```text
SCAN
 ↓
EXTRACT
 ↓
NORMALIZE
 ↓
COLLISION DETECTION
 ↓
LINEAGE CLASSIFICATION
 ↓
CANONICALIZATION
 ↓
REFERENCE REWRITE
 ↓
INTEGRITY REPORT
```

The compiler should produce:

- canonical registry;
- collision report;
- orphan-reference report;
- stale-reference report;
- lineage graph;
- dependency graph;
- unresolved-ambiguity queue.

## Agent-safe project resolution

Before an agent modifies a project:

```text
USER INTENT
 ↓
PROJECT LOOKUP
 ↓
CANONICAL ID RESOLUTION
 ↓
CURRENT ARTIFACT FETCH
 ↓
DEPENDENCY / LINEAGE CHECK
 ↓
PATCH PLAN
 ↓
AUTHORIZATION
 ↓
MODIFY
 ↓
TEST
 ↓
VERIFY
 ↓
REGISTRY UPDATE
```

No agent may infer canonical identity from a filename alone when a collision or historical alias exists.

## Stale reference protection

Each dependency reference stores the expected artifact identity:

```yaml
Reference:
  canonical_id:
  artifact_path:
  expected_content_sha:
  observed_commit:
  relation:
```

A changed SHA does not automatically mean a project changed identity; it triggers content verification and lineage review.

## Portfolio consistency checks

The registry should continuously test:

```text
PROJECT TABLE ↔ PROJECT FILES
PROJECT LINKS ↔ TARGET ARTIFACTS
ALIASES ↔ CANONICAL IDS
DEPENDENCIES ↔ EXISTING PROJECTS
README ↔ REGISTRY
KNOWLEDGE BASE ↔ PROJECT REFERENCES
```

## Evidence integrity

A registry can store claims about projects, but it must distinguish:

`repository observation ≠ inferred relationship ≠ architectural judgment`.

The registry itself therefore has an evidence ledger.

## Integration with the OmniCore control plane

Project 47 becomes a governance service:

```text
TASK
 ↓
PROJECT / CAPABILITY SEARCH
 ↓
CANONICAL ID RESOLUTION
 ↓
DEPENDENCY CONTEXT
 ↓
RELEVANT KNOWLEDGE
 ↓
SAFE MODIFICATION PLAN
```

## Definition of Done

- canonical IDs resolvable deterministically;
- zero silent numeric-collision overwrites;
- aliases and historical IDs represented explicitly;
- stable artifact/content identity;
- dependency and lineage graphs;
- stale-reference detection;
- orphan-reference detection;
- registry/index consistency checks;
- ambiguity queue;
- auditable registry revisions;
- agent mutation blocked until canonical identity is resolved.

---

## 2026-10-08 — batch 12: numbered-project identity collision repair

Observed repository defect: **P122 was assigned to two unrelated owners**: `CHEMIA` consent-aware mobile game and gas-appliance manual safety reference. The gas project came from the previous auto-ingestion batch despite the portfolio already containing CHEMIA/P122.

### Correction
- Keep CHEMIA canonical project **P122** with all its existing material, unchanged.
- Reassign gas-appliance safety reference to the next unused number **P123** and move its README/Python files without altering their tested lookup logic.
- Update ingestion reports, YAML delta, indexes and external canonical references that describe the gas appliance.
- Treat historical git commits as immutable provenance; append a reconciliation document rather than rewrite ancestry.

### Permanent registry gate
`ProjectIdentity` = stable numeric ID + canonical owner/path + product-boundary description. Future genesis must query the complete current tree (including folder projects and legacy Markdown) to test uniqueness before allocation. Multiple files for a **single** project ID are valid only when they share the same owner lineage; distinct products sharing an ID are an **error**.

**Acceptance:** P122-CHEMIA remains readable, P123 gas read-only artifact exists, old gas path absent, all new current-state references point to P123, and no claim of tested model control. This repair is a repository integrity correction, not a new scientific discovery.

---

## 2026-10-08 — batch 13: project genesis and source maturity bookkeeping

Checked the actual `main` tree before assignment: **P124 was unallocated**. The new P124 `Adult Safety Support Evidence Navigation Lab` is distinct from P36 influence security and P90 SOP, and is explicitly restricted to architecture/research due to the vulnerable-survivor, historical-law and shared-device threat model. Do not promote this proposed public-interest product to a live resource or claim current hotline/clinical availability without independently verified providers and professional safeguarding sign-off.

Maintain `ProjectIdentity{id:P124,owner_slug:adult-safety-support-evidence-navigation-lab,source_batch:13,status:ARCHITECTURE}`. Project ID uniqueness alone does not certify safety or avoid duplicates in substance; link existing projects as non-owning collaborators. Follow the existing `tools/project_id_gate.py` and re-check remote main prior to merge.

P32 OSINT provenance gate is a shared **tooling** enhancement and not a new product; tests checking simulated/observed/verified labels do not constitute real OSINT evidence.

---

## Knowledge evolution — batch 14 / 2026-10-08: byte-vs-text deduplication and source gap audits

Two 11-page `Automatyzacja Tworzenia Oprogramowania z AI` PDFs have **different SHA-256 byte fingerprints** but identical text after normalized extraction (`text_similarity=1.0`), so count them as one document lineage for research independence. Existing AURA P50/P71, GCG P60/P108, OmniCore P09/P26/P80, SOP P90/P115, strategy P66 and language P75 justify **no new numbered projects**.

Add `SourceCompletenessAudit`: AURA 60 declared cards, 30 item instances sufficiently identified (20 domain + 10 anomaly), 30 unresolved; thermochrom 29 vs 26–27°C contradictory. Add `FileNameAuthorityAudit`: UX581 laptop-model filename is not authoritative model/serial hardware evidence. Portfolio promotion requires GitHub readback of **canonical** modifications plus code tests, not only separate extension files.

---

## Knowledge evolution — batch 15 / 2026-10-08: batch 15 source family dedupe and new P125 identity

Among five ODYN/Nexus feasibility PDF variants, files `...(2).PDF` and `...(3).PDF` have **distinct SHA-256 byte digests but identical normalized extracted text** (similarity=1.0). `...(1)` introduces Jevbridge/AlterEgo/Hermes, `...(2)/(3)` latency/validation/memory-of-failure, `...(4)` semantic caching and ephemeral scripts. Track these as a single evolving architecture witness series, not five independently demonstrated deployments.

The 9 scanned pages in `7 Easy AI Digital Products.pdf` are a partial chapter 8–10 excerpt; source completeness must reflect that. `Analiza Chemiczno-Cyfrowa Zagrożeń` declares an "ASI" persona but that is author text, not validated capability.

P125 `sovereign-contextual-android-launcher` is assigned the previously unoccupied project number for a genuinely new Android HOME screen product; P105 actuation and P119 Android local agents remain separate owners. Verify ID uniqueness against current main before merge. Do not use report assertions as execution receipts. [Batch manifest](../docs/KNOWLEDGE-INGESTION-2026-10-08-BATCH-15.md).

---

## Knowledge evolution — batch 16 / 2026-10-08

Batch 16: 10 PDFs, 556 pages. Five feasibility PDFs (5)–(9) are one evolving MAS/Nexus lineage; DGM (1) and (2) differ in byte SHA256 but have **identical normalized extracted text**. The 372-page GGUF file is a conversation export, not a code bundle. New **P126** has unique owner scope (IBM Cloud server game state), distinct from P86 game app generation/P99 LiveOps/P121 SRE, and its number was free on baseline main. Never count report implementation assertions as CI/test receipts.

---

## 2026-10-08 — batch 17: Samsung Quick Share bulk-source acquisition gate

Nine shared collections advertised **101 files (~489.2 MB)**. Rendered manifests exposed 96 filenames, but **no underlying document bytes** were available to inspect/hash in the current tool environment; the ninth collection advertised five files still uploading. This is a **LISTED_ONLY** state, NOT ingestion, duplicate confirmation or project genesis. [Acquisition ledger](../docs/QUICKSHARE-INTAKE-2026-10-08-BATCH-17.md).

New executable `tools/source_bundle_audit.py` checks ZIP-member integrity and SHA-256 offline, without extracting, executing or transmitting contents. It rejects path traversal, symlinks, encrypted members, case-insensitive path collisions, extreme decompression ratios and size overflows; filenames are omitted from the default report. It does not read document semantics. Portfolio lifecycle is now `LISTED_ONLY → BYTES_RECEIVED → HASHED → CONTENT_READ → OWNER_MAPPED → VERIFIED → COMMITTED`. No automatically assigned project IDs or claims of technical improvements based on filename matches. Potentially sensitive named-person OSINT/private correspondence must be redacted or excluded from the public repository.

---

## 2026-10-08 — batch 18: public web source evolution

Web-source batch 18: 34 supplied URLs, 33 distinct after duplicate Cloud Translation URL. Public provider docs and Gemini CLI descriptions were readable; private console/API key/payment pages, Zed/Railway board items and unavailable bug-bash site were not. P21/P29/P37/P57/P100/P105/P106/P108/P115/P121/P125 already cover their topics — new numbered projects = 0. Never mark user account settings, API keys, bounty reward state, malware sample binaries or private workspaces as extracted from URL alone. See `docs/WEB-SOURCE-INGESTION-2026-10-08-BATCH-18.md`.

---

## 2026-10-09 — batch 19: agent marketplace, MCP and consent-based games

Batch 19: 19 URLs span GitHub Agent Apps, MCP registries, prompt CI, Codex events, Gemini Drops and competing adult two-player games. `SourceAccessStatus` distinguishes directly read listing, provider documentation alternate, inaccessible registry and conflicting first-party feature descriptions. **P122 CHEMIA already owns consent-aware paired gameplay**; P72/P100/P108 already own agent integrations, P87 SysML2/deterministic generators, P29/P114 research memory. Hence **0 new numbered projects** rather than redundant provider-branded variants. LovePlay homepage single-device versus its blog remote-pairing claims conflict; do not choose one without operational evidence. Full 19-link ledger `docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-19.md`.

---

## 2026-10-09 — batch 20: ODYN / Hermes / Nous ecosystem

**Batch 20 source accounting:** 80 unique `NousResearch/*` repository metadata objects verified using GitHub API, without scanning all 80 code trees; `atropos` reports `archived=true`; many repos are forks, not necessarily current parent upstreams. User also included repeated `nomos`, `cline`, Agentskills and Hermes docs links — deduplicate by canonical URL and source content, not by count in prompt. [Full 80-repo inventory](../docs/UPSTREAM-NOUSRESEARCH-REPOSITORY-CATALOG-2026-10-09.md) and [Hermes/ODYN source ledger](../docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-20.md). Cross-check user-owned `ODYN-AI` actual default branch **`codex/termux-five-goals` at `df56169...`** rather than assuming `main`; repo `PLANY-I-POST-PY--W-REPOZYTORIACH` contains workplans, not guaranteed deployed code. New numbered projects **0** because P17/P37/P72/P114/P115/P119 already own integration scope.

---

## 2026-10-09 — nadrzędna integralność zakresu repozytoriów

Po korekcie użytkownika portfolio ma jeden i tylko jeden repozytoryjny `write_target`: `mojealterego/Knowledge-projects`. Każde inne GitHub/HTTPS repo (w tym własny ODYN-AI) jest `external_source_read_only`, niezależnie od właściciela, gałęzi i pozornej kompatybilności modułów. Zewnętrzna struktura `AGENTS.md` nie zmienia tego zakresu. Wdrożenie konceptów z ODYN/Hermes odbywa się **poprzez własne pliki projektu i testy w Knowledge-projects**, a nie przez drugi PR w źródle.

`RepositoryMutationEvidence` powinien zawierać `target_repo, operation, branch_or_ref, requested_source_repo, expected_main_sha, new_commit_sha, pr_number, ci_status, readback`. Nie akceptuj `target_repo != mojealterego/Knowledge-projects`. Dodano `tools/single_repository_scope_gate.py` — deterministyczny preflight przyjmujący tylko repo docelowe, gałęzie robocze, PR do `main`, osobno pozwalający czytać dowolne repo źródłowe. **Checker nie blokuje bezpośrednich wywołań GitHub API — to ograniczenie do udokumentowania.**

[Erratum partii 20](../docs/REPOSITORY-SCOPE-ERRATUM-2026-10-09.md). PR #17 wykonany w ODYN-AI był błędem zakresu i nie upoważnia do kolejnych zmian ani samoczynnego rollbacku.

---

## 2026-10-09 — batch 21: Zed Guild, Railway bounties, WordPress premium

**Source reconciliation / batch 21:** These three URLs were already recorded as title/shell or account-restricted sources in batches 18–19, so no new conceptual project is warranted. New evidence now includes official `zed.dev/community/guild` track descriptions and current GitHub REST issue states (Zed #51333/#65199 OPEN and #65205 CLOSED on 2026-10-09), official Railway templates README+Station historical examples, and current WordPress plugin support guide (reviewed 2026-10-05). Keep `BOARD_SHELL`, `ISSUE_STATE_API_VERIFIED`, `OLD_SOLVED_REWARD`, `AUTH_REQUIRED` separate; a bounty board shell or private paid-plugin URL does not confer job, money, account access or installed plugin state. All implementation writes are restricted to Knowledge-projects. [Batch 21](../docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-21.md).

---

## 2026-10-09 — batch 22: Googlebook responsive launcher implemented

**One already-known source, one new code delta:** user supplied `android-developers.googleblog.com/2026/09/adaptive-development-scale-app-googlebook.html?m=1`, the mobile rendering of the same Sept 22 2026 article already ingested in batch **18**. Normalize by canonical article URL and publication date rather than treating query `?m=1` as a distinct primary document. **Do not create new P127**: P125 already owns Android HOME launcher and P105/P100 own UI actuation/developer tooling. The new evidence is a committed patch to P125 Kotlin/XML and 13 static tests, **not** a separate Android app or independently benchmarked device. Preserve `SOURCE_REUSED → EXISTING_OWNER_EVOLVED → CODE_COMMITTED → CI_STATIC_PASS/FAIL → SDK_BUILD_PENDING`. See [batch22 ledger](../docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-22.md).

---

## 2026-10-09 — batch 23: seven-page GitHub MCP Registry discovery

**Observed source:** GitHub `/mcp?page=1…7` dynamic registry lists **210 unique entries** (30/page) against a catalog-declared **394 total** on 2026-10-09; 184 outside supplied pages remain unreviewed. This is NOT 394 installed plugins, and source descriptions are **vendor-authored claims**, not inspected code or runtime `tools/list`. The actual complete 210-entry source snapshot lives at `docs/mcp-registry/2026-10-09-github-mcp-pages-1-7.json` with source page, slot, display title, description, exact listing URL, and `PUBLIC_LISTING_DESCRIPTION_UNVERIFIED`. Two cards lack titles in extraction; preserve null rather than fabricate. Implemented `tools/mcp_registry_catalog.py` validates cardinality (7×30), links, duplicates and missing-title evidence with tests. Canonical source owner is P47; no new project ID for each MCP vendor. All write actions restricted to Knowledge-projects by AGENTS.md. See `docs/WEB-SOURCE-INGESTION-2026-10-09-BATCH-23.md`.
