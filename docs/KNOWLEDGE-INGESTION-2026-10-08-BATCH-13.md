# Automatic knowledge ingestion — batch 13 (2026-10-08)

**Repository:** `mojealterego/Knowledge-projects`, baseline `main@45b4b4c64ff4686101d414264b74e598c09e54c0`. **Sources:** nine PDFs and one binary DOC file. SHA-256 measured over the provided source file bytes. Source files (including 1997/older copyrighted books and sensitive human-subject material) are **not uploaded** into this public repository.

## Manifest, provenance and owner resolution
| # | File | SHA-256 | Pages or medium | Owner | Assessment |
|---:|---|---|---|---|---|
| 1 | `--AI w Tworzeniu Systemów Operacyjnych-- (1).pdf` | `bf9593e6c0bd61d945e5e993943ca8e79da45a751894751ed6e1e075a8bfc29c` | 9 PDF pages | P26/P80/P115 | Practical agent role configuration for OmniCore; previously established design lineage |
| 2 | `55_wskazowki_Ukryta_perswazja_Hogan (1).pdf` | `6f6d5700127c8cf5081bac0677ff9188f0ee139f9ded5aea8b9fe232bcdcf39a` | 73 PDF pages | P36/P40 | Copyrighted persuasion tactics; defensive analysis, no coercion playbook |
| 3 | `adminojs,+Administrator+czasopisma,+PEFIM_2015_n63_s31.pdf` | `b378ca6c1d844c8231dc3c02c0dff9378758db5f684de8eac050607aeb7a3a79` | 22 PDF pages | P66 | 2015 Polish academic hybrid online/offline strategy and Ansoff case study |
| 4 | `Agent OSINT.pdf` | `1384d8aae89e937f2945c4b6af0e4c4b4a992c36dc645929f48626d9be4257b1` | 4 PDF pages | P32 | Unsafe reference code: HTTP IP API, browser impersonation, URL redirects and HTTP-status username heuristics |
| 5 | `AI w Zarządzaniu Projektami- Automatyzacja i Instr....pdf` | `afdf76949247d126ddbf0c8de2f3f8680dff8ad4a38142e3f04231ef3912bb4e` | 11 PDF pages | P115/P90 | WBS and controlled agent execution; overlaps prior HTA/WBS knowledge |
| 6 | `AI_ Instrukcje i Automatyzacja Zadań.pdf` | `e6a46bdda53a60530883471f52715f6fb6e5c4f8a5085ecfc90e9a58ceed115a` | 15 PDF pages | P90/P115 | HTA/SOP tool orchestration, typed pre/postconditions and recovery |
| 7 | `A_Filipek_Wspomaganie_czlowieka_doroslego_w_sytuacji_przemocy_w_rodzinie.pdf` | `df6950f64700ff7c657f31005b450403c03dd39fda81d0a98f9a1625cde1eba2` | 394 PDF pages | NEW P124 | 2017 doctoral study and surveyed 152 adults in three Podlaskie cities, 2016; distinct high-stakes support-navigation domain |
| 8 | `AETHER.pdf` | `da96142c9f75fadd58b3c2c3e5ab5c2b5467f1904ff2895287dca1710e8bf7c6` | 13 PDF pages | P57/P32 | Image-only Three.js/Web Audio demo; mock machine state and network IP lookup; prior KB already exists |
| 9 | `alchemia uwodzenia, czyli erotyczna manipulacja mężczyznami scan.pdf` | `bf9eada88658241149a5448150ac8887663ecceb2c32752b7e48de14fdfac307` | 405 PDF pages | P36/P40 | Copyrighted scanned dating/relationship advice; extraction imperfect, defensive consent/red-flag analysis |
| 10 | `Anna Grzywa - Manipulacja - mechanizmy psychologiczne.doc` | `553747647361545c6754308c85d1a370dc7afdca002d94c29a8ae704fe11ea17` | DOC, 100 displayed pages | P36/P40 | 1997 psychological manipulation monograph; already covered in KB; DOC page count ~100 PDF-equivalent in attachment display |

## Deduplication and freshness
- `AETHER.pdf`: an image-only code screenshot of a NEXUS-OMEGA browser UI. This subject already exists at `docs/knowledge-base/aether-nexus-omega-interface-prototype-analysis.md`. Reviewed rendered pages 1–13. Not a running operating system, actual kernel, verified scanner or real "128TB virtual" system.
- `Anna Grzywa – Manipulacja`: topical predecessor exists in `docs/knowledge-base/influence-manipulation-defense-psychology.md`. The file is an old Word DOC and may not be bit-identical to any prior material; we do not invent byte identity.
- `AI w Zarządzaniu Projektami` and `AI_ Instrukcje i Automatyzacja Zadań`: overlap existing HTA/WBS/SOP knowledge and P90/P115, but have distinct sources/hashes and useful contracts; no duplicate new project.
- `--AI w Tworzeniu Systemów Operacyjnych--`: OmniCore agent forge existing P26/P80/P115; no new kernel project.
- `55 wskazówek` and `Alchemia uwodzenia`: copyrighted books with unverified marketing/psychological efficacy; use for defensive, consent-centred taxonomy, not a manipulation generator.
- `adminojs...2015`: old scientific marketing case study, not contemporaneous evidence of business returns in 2026.
- `Agent OSINT.pdf` contains hidden zero-width characters in text extraction, unencrypted external HTTP, browser User-Agent spoofing and unsafe username enumeration heuristics. **Reviewed as a source code safety test fixture**, not executed as a scanner.
- The 394-page Filipek dissertation contains research methodology, historical law and survey results; maintain strict aggregate-only provenance. The 2016 purposive sample of 152 adults in Białystok (97), Łomża (31) and Suwałki (24) is not representative of Poland. Historical law is **not current legal guidance**.
- `AETHER.pdf` and the 405-page scanned dating book have imperfect/nonexistent text layers; visual review was used for the interface and content categories, rather than claiming successful text extraction of every page.

## Project decisions
**Canonical updates:** P26, P32, P36, P57, P66, P90, P115. **New distinct owner:** **P124 — Adult Safety Support Evidence Navigation Lab** for the restricted domestic-violence assistance domain. Source-only speculative clinical/legal judgments are barred; no victim surveillance, diagnosis, automated intervention or legal deadlines.

**Implemented utility:** `tools/osint_provenance_gate.py` and `tools/test_osint_provenance_gate.py`: pure data-only evidence/provenance labels for simulated, observed and documented-verification records; HTTP 200 never becomes a claimed personal account match. Local tests must be recorded separately from GitHub CI.

## Verification limits
- No provider/current legal authority or social-service hotline availability verified for 2026.
- No real reconnaissance, HTTP/IP requests, social account enumeration, OS image/kernel build, behavioral human experiment, full clinical outcome study, or cloud workflow deployment.
- No claims of universal persuasion efficacy, causal proof of marketing strategy, AI project autonomy or guaranteed help outcomes.
- Attribution and source integrity do not authorize exploitation or personal-data publication.
