# Batch 12 — Automatic source ingestion and cross-project evolution — 2026-10-08

## Execution intent
Ten user-uploaded PDF files received in the active conversation; standing rule is **automatic processing** without waiting for a second “Analizuj”. SHA-256 below is computed from each actual supplied PDF's bytes. Text was extracted across all pages and cross-checked against the repository's prior project ownership and knowledge corpus.

## Source manifest and allocation
| No. | Source | Pages | SHA-256 | Existing owner(s) | Decision |
|---:|---|---:|---|---|---|
| 1 | `Upadek Epoki Brązu- Klimat i Cywilizacje.pdf` | 9 | `dc10313270758ecea33c3888e2357113a861c330e4750dd2fb9ba7c3479339cb` | P19 + historical evidence research | Previously covered in 2026-09-10 multi-source corpus; new test fixture |
| 2 | `Unikalna Osobowość Modelu Językowego.pdf` | 11 | `e339a59392671bfacc10dfe6d01d844cf50e72fdc5feb3f23c3cc7aa44a94a62` | P91 | Persona source family; near-duplicate |
| 3 | `Unikalna Osobowość Modelu Językowego (1).pdf` | 14 | `2ed7f460697daa6e7d98c691dfbb0978705109ff9003be3f48577d5d3147a2d4` | P91 | Near-duplicate (0.9696 normalized similarity) |
| 4 | `Unifikacja Języków i Kodu Legacy.pdf` | 7 | `432d457ed78c3fcd101fe495285d3ea82ff4d95a9f31c2ae71afd3326e4a21e8` | P87 / P23 | Previously covered MLIR/CIRA lineage |
| 5 | `Ulepszanie modeli AI- zaawansowane techniki.pdf` | 13 | `3658a1fea5f751c92e904b64f96937a7ee94c325ede733da8c6496f5a1cd0849` | P11 / P17 / P27 | New benchmark specification for speculative decode, GraphRAG |
| 6 | `Ukryte Komendy w Tekście.PDF` | 10 | `378734d2289f4d7d4263f6d11ee85f18ed25dda6562ec6bbb9d877f1c771b91d` | P36 / P54 | New defensive hidden-instruction input review |
| 7 | `Tworzenie zaawansowanego agenta badawczego.pdf` | 13 | `a50cc79fa191721cdf34bef5516ae159c77a9eb3e3b0b2cde5d155b0b2273c1e` | P19 / P15 / P48 | New evaluated fan-out and citation grounding |
| 8 | `Tworzenie Systemu MVP Omnicore.pdf` | 16 | `dde381e43ac34adc56e7fd7e18159d6f9f0874352ea27bd528cac2d9f9ba3dd3` | P80 / P87 | MVP proof gates; no compilation evidence |
| 9 | `Tworzenie Raportów i Delegowanie AI.pdf` | 14 | `9540dbdb2e692e231d473ce9316c90a8cdaf8109649cd6a8422462e314e98257` | P90 | Prior SOP lineage; add execution receipts and delegation contract |
| 10 | `Tworzenie Nowego Systemu Dywinacji.pdf` | 12 | `1d376719ec10dd045ce1765f750a39196defbb71b715abea87862c55209ca2b4` | P01 / P53 / P71 | Existing Apeiron lineage; new FOG contract |

## Deduplication and project genesis
- The two `Unikalna Osobowość` PDFs are **different files**, not byte-identical. Their extracted, whitespace-normalized text is **~96.96% similar**; prior corpus had already recognized a ~97.6% overlap under a separate normalization. They jointly strengthen P91, not independent evidence for two new projects.
- The 2026-09-10 corpus `docs/knowledge-base/2026-09-10-corpus-paula-sop-omnicore-persona-legacy-ue5-history-math-vantage.md` had already absorbed related persona, legacy/Omnis, SOP, OmniCore, and ancient-history material.
- P01, P11, P19, P23, P36, P80, P87, P90 and P91 all have **real prior owner boundaries**. This batch materially extends canonical docs for those and P47 but justifies **zero new numbered projects**.
- A separate pre-existing collision was detected and corrected: P122 already named **CHEMIA** when prior automation created another P122 for gas safety. The latter was reindexed **P123** (identical read-only lookup code), CHEMIA remains P122. See `docs/PROJECT-NUMBER-RECONCILIATION-2026-10-08.md`. This is an integrity correction, **not** a newly conceived batch-12 project.

## New synthesis delivered
1. Bronze Age and Voynich source becomes a **causal-inference and provenance stress test** for P19, not a verified single-cause historical verdict.
2. Persona pair becomes a **licensed dataset qualification + measurable style-consistency and regression suite** for P91.
3. MLIR/CIRA documentation becomes a **migration equivalence dossier** for P87/P23.
4. GPU/memory/speculative decoding/GraphRAG becomes an **apples-to-apples benchmark specification** for P11.
5. Concealed text becomes a **defensive untrusted-input/UX audit** for P36, not an operational coercion method.
6. Deep Research document contributes **scoped fan-out, provider snapshot, source-level citations and contradiction checks** for P19.
7. OmniCore MVP report contributes **no-LLM-in-Ring-0 boundary, isolated QEMU, prototype promotion gates** for P80.
8. SOP document contributes **task/action receipts, least privilege and workflow rollback contracts** for P90.
9. Apeiron report contributes **five-node symbolic FOG loop, operator/user agency and falsifiable reflection gates** for P01.

## Integrity / validation
- Source **page count and byte SHA-256: measured**.
- Previous ownership: confirmed from current GitHub `main`.
- Existing projects: actual canonical Markdown changed; this is design/architecture evolution unless otherwise shown.
- Historic research findings, psychological manipulation efficacy, quantum/divination claims, performance prices and unverified academic references are **SOURCE_CLAIMS** requiring independent validation.
- No LLM fine-tuning, compiler build, QEMU kernel boot, cloud deployment, medical/therapeutic procedure, human-subject persuasion experiment, or commercially deployed SOP performed.
- No user PDF copies, proprietary code, credentials, or personally identifying details added to public GitHub.
