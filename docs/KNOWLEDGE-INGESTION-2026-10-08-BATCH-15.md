# Knowledge-projects — auto-ingest batch 15 / 2026-10-08

**Trigger:** ten uploads in an active chat; process immediately without a second "Analizuj". **Baseline:** GitHub `main@d3301e5647b4d49f1d742b0e7c3f888bfe3df0b9`. Input files were inspected in their uploaded byte form with PyMuPDF; a scanned 9-page excerpt was manually reviewed through a rendered page contact sheet. No copyrighted PDF bytes or sensitive extracted operational payloads are added to this public repository.

## Complete source manifest
| # | File | Pages | SHA-256 | Canonical owner / decision | Limit |
|---:|---|---:|---|---|---|
| 1 | `7 Easy AI Digital Products.pdf` | 9 | `d0a323b71982ad3bbdb477727f04ce5a3eec5140019964777b88464448c26f79` | P45/P56 | 9 scanned pages; fragment of chapters 8–10, NOT full book or list of 7 products |
| 2 | `AI Launchery Android_ Ranking 2025-2026.PDF` | 10 | `203f57f3ee7ecbc99b9573fec3a01fb06c2417ae42d3cc2d8ed4c8d8e1a019e1` | NEW P125; P105/P119 | contextual launcher comparison; dates/providers not externally verified |
| 3 | `Analiza Chemiczno-Cyfrowa Zagrożeń.PDF` | 8 | `460187ac79d5b451ac789bd13c4fa690db9134fed5dcb3b701cc3809df8fb7ce` | P91/P32/P60 | persona/agent config + high-risk toxicology/surveillance claims; not deployable code |
| 4 | `Analiza narzędzi do manipulacji żetonami_260402_174419.pdf` | 24 | `78c55e723baa9789690f5493d1c1c3a0d7a226db7b77a44bb32326e42108614b` | P72/P56/P70 | unsafe client-token manipulation methods; defensive server-side ledger and invariants only |
| 5 | `Analiza Repozytoriów i Projekt Aplikacji.PDF` | 34 | `2b3e5d3363ca5644b12b5ec311a5edeab0605f9e70dc2f88df9ada1fd0f71942` | P33/P100/P115 | OmniStack conceptual comparison of 8 app builders, sample build logs are source claims |
| 6 | `Analiza wykonalności nowych integracji i funkcji a...PDF` | 13 | `0a90f89158f7f1a3004fafd1878aeb6d06198e9ab39e536b64c8479fe20f73a5` | P115/P114/P72 | base Nexus/ODYN MAS, bitemporal memory, dual LLM, skills, computer-use |
| 7 | `Analiza wykonalności nowych integracji i funkcji a..(1).PDF` | 15 | `d5451de0f5feefb673d39207567c58d73fb4d3c40c05265ea8cf94dcc809e76f` | P115/P114/P72 | adds AlterEgo ROS Noetic, Jevbridge MCP, structured Hermes calling claims |
| 8 | `Analiza wykonalności nowych integracji i funkcji a..(2).PDF` | 10 | `8cb16b72df7a40a4efa31381c12e782dfc37b0dd3c7cdf4ec77a44a64dd90bb5` | P115/P114/P72 | new latency budgets, strict inputs, failure memory, GoT reasoning review |
| 9 | `Analiza wykonalności nowych integracji i funkcji a..(3).PDF` | 10 | `fe4df64504a258d1fd2d523deb0dba0942a670b818630277f8eff79707140c7b` | P115/P114/P72 | normalized extracted-text EXACT DUPLICATE of (2), different bytes |
| 10 | `Analiza wykonalności nowych integracji i funkcji a..(4).PDF` | 13 | `9065e7fb45bbc007400ecb986ba2e13e6cb830bf03e4a92e52901c89102ba889` | P115/P114/P72 | semantic cache + throwaway Python scripts; source script runs untrusted Python directly |

**Input scope:** 10 PDF / **146 pages**. Distinct byte SHA-256 for all 10 files. `Analiza wykonalności ...(2)` and `...(3)` are **not byte duplicates** but identical normalized extracted text (sequence ratio 1.0). The five feasibility reports form *one evolving source series*, not five independently validated implementations.

## Source interpretation / integrity findings
- The **9 image-only pages** of `7 Easy AI Digital Products.pdf` begin in the middle of *chapter 8* and continue with email newsletter templates, online-sales marketplaces and tools in chapters 9–10. They do not contain the earlier promised "seven" ideas. Claims of passive-income certainty are unverified. This is an excerpt, not a complete book.
- The Android launcher report contrasts static launchers, UsageStatsManager, notifications/context, local/NPU inference and AI recommendations. **A HOME-screen product** has distinct scope from P105 Android app-actuation and P119 local agent runtime: create **P125** as a privacy-preserving Android HOME MVP. No sensitive Android permissions by default. Source benchmarks/availability are not live validated.
- The toxicology/cyber persona PDF includes unverified "ASI/global surveillance", invasive intelligence and chemical claims; one purported JSON fragment is malformed (`"primary_directives":,`). Treat it as a **threat/persona and provenance fixture**. No surveillance, stalking, illicit chemical or spyware integration is authorized by the source.
- The 24-page credit/token-manipulation report includes client-side tampering, network interception and transaction/race descriptions. It is **not** an instruction to modify third-party credit balances. Reframe as **server-authoritative ledger, signed event and replay/invariant** defenses (P72/P70/P56).
- The 34-page OmniStack report describes eight app-builder projects (Convex Chef, Wasp MAGE, Open Lovable, Open Design, CodinIT.dev, December, Dyad, Bolt.diy) plus future OmniStack; details and sample logs are **report claims**, not direct checks of current upstream repositories. P33/P100/P115 already own these capabilities.
- Iterative ODYN/Nexus reports claim `/nexus_core/...` files and features are "implemented"; those **assertions have not been verified against the ODYN-AI repository or runtime** in this ingestion. Especially `importlib.util.spec_from_file_location` plugin loading, `subprocess.run(["python3", path])` for arbitrary generated code and ROS physical actuation require isolation, authorization and evidence; model output isn't permission.
- The variation `...(4)` adds Semantic Caching / Throwaway Scripts; `...(2)` adds latency budgets, strict input validation and memory-of-failures; `...(1)` adds Jevbridge, AlterEgo robotics and Hermes tool formatting. None demonstrates signed, sandboxed tool promotion, and a formatted JSON schema alone is not proof of correct tool execution.

## Actual decisions
- **NEW P125 Sovereign Contextual Android Launcher:** native Kotlin/Gradle source scaffold for selectable HOME screen, manual app launch, no sensitive runtime permissions. Android SDK build, device install and UX benchmarks are **NOT EXECUTED**.
- **Canonical evolution:** P33, P45, P47, P56, P70, P72, P91, P100, P105, P114, P115, P119. Existing projects remain owners for OmniStack, personas, credit integrity and ODYN.
- **New code:** `tools/agent_tool_admission_gate.py` with independent approval context, artifact-hash binding, time/scope/side-effect/sandbox constraints. It is **a non-executing admission check**, not a real OS sandbox or signature PKI. `tools/test_agent_tool_admission_gate.py` and `tools/test_sovereign_launcher_policy.py` provide reproducible stdlib tests.
- **No unsafe publication:** no harmful chemical formulas, unauthorized token mutation steps or bypass scripts, no leaked keys or proprietary source PDFs.

## Delivery states
Knowledge extraction and canonical documentation: done upon merged, verified GitHub commit. Android Kotlin source: scaffold only, build not attempted. Python admission validator: tested by CI when available. Vendor/app ranking, ODYN runtime behavior, account ownership, current model prices, arbitrary Python sandbox enforcement and actual commerce profits: **not independently verified**.
