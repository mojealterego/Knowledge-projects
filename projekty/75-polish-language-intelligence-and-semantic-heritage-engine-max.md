# Project 75 — Polish Language Intelligence & Semantic Heritage Engine MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-09

## Mission
Build a provenance-aware language intelligence system for Polish that can operate across historical and contemporary language, preserving lexical evidence, semantic change, stylistic register and normative context instead of collapsing everything into one undifferentiated language model.

## Why this is a new project
The supplied corpus combines a historical lexicographic source with a structured course on contemporary Polish language culture. Existing repository projects provide general research, content and agent infrastructure, but no identified project owns the complete **historical-to-modern Polish semantic layer** as an independently reusable product.

## Core pipeline
```text
HISTORICAL / CONTEMPORARY SOURCES
        ↓
OCR / DOCUMENT INGESTION
        ↓
LEXICAL RECORDS + PROVENANCE
        ↓
TEMPORAL SEMANTIC GRAPH
        ↓
STYLE / REGISTER / NORM LAYER
        ↓
QUERY + COMPARISON ENGINE
        ↓
AI GENERATION WITH EVIDENCE
        ↓
LINGUISTIC CRITIC / VERIFIER
        ↓
AUDITABLE ANSWER
```

## Architecture
- **Lexicon Store** — versioned lexical records.
- **Semantic Graph** — synonymy, polysemy, derivation, historical meaning and phraseology.
- **Temporal Layer** — separates historical attestation from modern usage.
- **Norm Layer** — descriptive usage vs normative recommendation.
- **Corpus Adapter** — imports scans, OCR, documents and future corpora.
- **Language Agent** — answers questions and generates text with source references.
- **Critic** — detects anachronism, unsupported claims, register mismatch and source leakage.
- **Evaluation Harness** — historical-form recognition, semantic drift and contemporary correctness tests.

## Data contract
```yaml
LexicalRecord:
  id:
  lemma:
  form:
  historical_forms: []
  meaning:
  period:
  register:
  phraseology: []
  examples: []
  source_ref:
  source_page:
  provenance:
  normative_status:
  confidence:
  semantic_relations: []
```

## Key use cases
1. Explain historical words and forms without pretending they are contemporary norm.
2. Compare historical and modern meanings.
3. Assist Polish-language writing while preserving stylistic intent.
4. Analyze press/public communication and lexical change.
5. Support narrative/game/creative projects requiring historically coherent Polish.
6. Provide evidence-grounded linguistic research for other agents.

## Safety / integrity
The engine must not invent dictionary entries. Historical forms require source provenance. Normative recommendations require a clearly identified normative source. Generated language is not evidence merely because it sounds plausible.

## Portfolio integration
Extends Projects 15, 19, 22, 24, 32, 42, 44, 48, 54 and 57, and supplies a domain-specific language layer to Projects 65 and 72.

## Definition of Done
- Historical lexical records are versioned and provenance-linked.
- Contemporary normative status is separately represented.
- Semantic relations are queryable.
- OCR uncertainty is retained rather than silently corrected.
- AI answers expose source/evidence boundaries.
- Evaluation includes historical and modern test sets.
- The system can be consumed as a reusable agent/tool capability.

---

## Knowledge evolution — batch 14 / 2026-10-08: contemporary lexical debate on sexualization

Three-page article `Seks – seksualizm – seksualizacja – seksualizować` by S. Dubisz (`Poradnik Językowy`, 2019; DOI `10.33896/PorJ.2019.7.12`) compares dictionary meanings, register shifts and contested rhetorical redefinitions of psychological/educational terminology.

`LexicalSenseEvidence` adds `lemma, sense_id, dictionary_or_source, time, register, field, context, competing_usage, semantic_shift_claim, uncertainty`. Neither an author's interpretation nor a quoted public statement independently reveals a speaker's motives. P75 should support neutral multiple-sense explanations without classifying personal sexuality, political loyalty or intent. Review official current dictionaries and the quoted context before providing normative conclusions.

---

## 2026-10-08 — batch 18: public web source evolution

Public Google Translation/Natural Language docs suggest glossary-driven translation, linguistic analysis and language processing. Add LexicalEvidence {Polish_original,language,source_span,register,morphology,translation_candidate,glossary_version,ambiguity,reviewer}. Retain original Polish even if translated; cross-lingual embeddings may assist search but not establish legal/medical translation validity. Free-tier figures are time-sensitive per model; no unlimited use or API calls implied.
