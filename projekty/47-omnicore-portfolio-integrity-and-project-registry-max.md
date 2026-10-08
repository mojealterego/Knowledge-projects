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
