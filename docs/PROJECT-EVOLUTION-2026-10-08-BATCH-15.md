# Portfolio evolution — batch 15 / 2026-10-08

## Real canonical project updates
| Project | New knowledge | Class |
|---|---|---|
| **P33** | eight builder mechanisms, secure agent build/test/deploy architecture | architecture |
| **P45** | digital newsletter/marketplace bundles, copyright/consent and packaging proof | architecture |
| **P47** | five-document version lineage, byte-vs-text duplicate, P125 unique owner | registry |
| **P56** | customer outcomes/profit claims separated from subscriptions and token ledger integrity | architecture |
| **P70** | token-like credits, atomic server-side balance and replay safeguards, no client authority | defensive |
| **P72** | agent tool synthesis privilege review plus token API trust boundaries | defensive |
| **P91** | self-declared "ASI" chemical-cyber persona needs parse/safety/competence verification | defensive |
| **P100** | OmniStack comparative TDD/design/security enforcement contracts | architecture |
| **P105** | boundary between opt-in launcher and verified Android actuation | architecture |
| **P114** | ODYN bitemporal/semantic cache provenance, stale/tenant-aware memory controls | architecture |
| **P115** | report version evolution, independent build receipts, bounded tool proposal/admission | architecture + validator |
| **P119** | launcher as optional UI entry point, not automated accessibility/notification control | architecture |
| **NEW P125** | privacy-preserving HOME launcher Kotlin app source, build config, XML, static tests | code scaffold |

## Tangible changes versus unverified source claims
**Tangible:** code, tests, canonical Markdown, ledger and indexes committed to GitHub; CI runs subject to readback.
**Not proven:** marketed Android launcher rankings; functioning local GGUF on launcher; eight upstream builder current APIs; actual Nexus/ODYN source files and tool execution; chemical/toxicology expertise; ability to alter third-party tokens; proven revenue from email-template sales.
**No new tool permission:** security-critical or physical actions require independent authorization outside model output and auditable separate tool execution.

## Project genesis
**P125 is a new specific product**: an actual Android HOME role UX. It differs from P105 (mobile GUI actions) and P119 (local agent runtime). Confirmed the number was unused in pre-merge baseline tree. No further project is created: the other nine documents map to existing owners.

## Test contract
- `tools/test_agent_tool_admission_gate.py`: external approval map, digest matching, restricted permissions, timeout, expiry, forbidden side-effects and zero code execution.
- `tools/test_sovereign_launcher_policy.py`: source-level Android manifest HOME intent, app visibility query, no sensitive permissions, tap-to-open, no default data exfiltration.
- `.github/workflows/knowledge-project-python-gates.yml` from prior batch discovers all `tools/test_*.py`.
- **Android build/integration tests remain separately required**. Treat static tests as policy smoke tests only.
