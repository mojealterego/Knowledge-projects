# Knowledge ingestion — 2026-10-08 — batch of 10 uploads

## Method and repository baseline
- Repository: `mojealterego/Knowledge-projects`; reviewed baseline: `main@c26a21dd469d495ccc028d64276fbd04627c4850`.
- Ingest scope: nine PDFs and one HTML file supplied by the owner. The HTML file is **not** an MP3.
- Source texts were extracted and assessed against the actual repository tree (592 entries at baseline) and existing project/knowledge artifacts.
- Content is **source-derived**, not independent validation of papers, provider pricing, medical/neuroscience effects, market results or current law.
- SHA-256 records fingerprint these particular uploads; matching titles to previous summaries proves topical coverage, **not byte-identical documents**.
- No source PDFs or personal identifying data are published into the public repository.

## Input manifest
| # | Source filename | SHA-256 | Decision | Destination |
|---:|---|---|---|---|
| 1 | `Wpływ Wizualny na Ludzki Umysł.pdf` | `7f4f0f2db3b13e2ae0e5203adf5b709e2c89a750b58256c5b5314b4ee496bbb4` | NEW-SYNTHESIS | Visual influence / defensive influence-risk |
| 2 | `Wykrywanie Kłamstw_ Analiza Behawioralna i Lingwis...PDF` | `99ae6a38f338f6f78b52c834a591260d8d891f84d704140fd71d4ee4c16fd6c6` | PREVIOUSLY-COVERED | P92 / 2026-09-10 iteration 16 |
| 3 | `Wyłączanie Ograniczeń Modelu Gemini.PDF` | `db01ffe2587d0cd6679d83c6799093c6b9c5e8db7419ab6598d31c4e023360c1` | PREVIOUSLY-COVERED | Model safety / P108 / iteration 16 |
| 4 | `Zarabianie Pieniędzy Online i Offline 2026.pdf` | `d7bcfad86671fee17c2fa19be7f799a59847de6dc936ba01449af2c0bb704eb3` | PREVIOUSLY-COVERED | P34/P66/P67 / iteration 16 |
| 5 | `Zarabianie Pieniędzy z Wykorzystaniem AI.pdf` | `fe0ba90d2af73eb3bd68820d02c2f4fbd52ca704ed7d9463771ce5d042ddad96` | PREVIOUSLY-COVERED | P34/P66/P67 / iteration 16 |
| 6 | `Zero Trust Hardware i Ekonomia Tokenowa.pdf` | `196e29c5c6b5b02845fefda9f5872993a57c7cf21389e54f0bd3059fe2024f3a` | PREVIOUSLY-COVERED | P93 / iteration 16 |
| 7 | `Zmiana Nazwy Projektu na Andre_ Manualnie i AI.pdf` | `66330ecf0a180fc41d6f7e93844a378836d2970fe17e64ecafdaa88dac5e38ea` | NEW-SYNTHESIS | P32 OSINT rebranding migration QA |
| 8 | `Zwiększanie Wydajności Gemini Pro.pdf` | `389e8e60e6af2e54c5a0d90f1774dc97ad7eebd76b46a10c99e9463b7e6a2e63` | NEW-SYNTHESIS | P14/P17/P18/P27 runtime optimization |
| 9 | `voice_preview_ian — polish narrator (warm_deep).mp3.html` | `4f81c129b90e49250d990ac8df0e16f770dc75d12d7f445d1fb0177f9d2b8973` | INVALID-AUDIO-REFERENCE | HTML ElevenLabs app shell, no audio asset |
| 10 | `zgromadzenia-poradnik.pdf` | `7f781a6767d78f8d82db159f5ced513416e2fed3790525ae4d31275d72666f06` | NEW-HISTORICAL-REFERENCE | Polish public assemblies; legal currency not verified |

## Dedupe / correlation
Five uploads are substantially covered by `docs/knowledge-base/2026-09-10-corpus-deception-gemini-osint-monetization-depin.md` (Iteration 16). The existing project files `projekty/92-...` and `projekty/93-...` already own deception analysis and DePIN hardware-attestation. Do not create duplicate standalone projects.

New marginal source dimensions:
1. Gemini Pro inference-time optimization (cost/quality/latency frontier and reproducible evaluation);
2. Visual stimulus and influence-safety taxonomy (especially accessibility and photosensitive-epilepsy testing);
3. Andre naming/migration across code, API/UI, reports, prompts, storage and audit trails;
4. Public assemblies: dated legal-reference archive rather than current legal advice;
5. Voice filename integrity: detect an HTML app page masquerading as an MP3.

## Traceable artifacts
- `docs/knowledge-base/2026-10-08-gemini-performance-evidence.md`
- `docs/knowledge-base/2026-10-08-visual-influence-defense.md`
- `docs/knowledge-base/2026-10-08-andre-migration-assurance.md`
- `docs/knowledge-base/2026-10-08-assembly-law-reference-poland.md`
- `docs/knowledge-base/2026-10-08-voice-preview-html-validation.md`
- `docs/PROJECT-EVOLUTION-2026-10-08-BATCH-10.md`
- Extension specifications for P14, P32, P36, P34 and P81.
- New delta record `docs/PORTFOLIO-DELTA-2026-10-08-BATCH-10.yaml`.

## Epistemic & authorization invariants
- Cues in facial movement/language are not evidence establishing deception or guilt; PEACE is the non-coercive design reference.
- Visual design cannot be treated as mind control; claimed effects require controlled research and safety/accessibility testing.
- Model refusal/jailbreak behavior is not a guarantee about internal mechanisms. Model-generated text is not authorization.
- Discounts, prices, model IDs, rate limits, MiCA/AI Act scope and Polish law are version- and jurisdiction-dependent.
- Profit projections and AI-agency revenues require independent primary-source verification and actual customer experiments.
- Device attestation verifies bounded measurements; physical work and token reward still require external corroboration.
- Andre rebranding is a **candidate project-specific migration**, not permission to rename the entire portfolio.
- HTML app shell is neither voice audio nor proof of ElevenLabs voice quality.

## State after ingestion
- Knowledge and architecture artifacts: documented.
- Functional code migrations, deployed applications, security tests, human studies, hardware validation, market experiments: **NOT EXECUTED** in this batch.
- New numbered projects: **0**, because proposed capabilities fit existing project ownership boundaries.
