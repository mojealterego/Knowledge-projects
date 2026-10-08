# Evolucja projektów — 19 linków, partia 19 / 2026-10-09

| Projekt | Uzyskana wiedza i konkretne rozszerzenie | Stan |
|---|---|---|
| **P19** research orchestration | SAKH read-only bounded corpus, page-level provenance, knowledge graph vs independent evidence | CANONICAL_DOC |
| **P29** evidence fabric | Traveler profile/trip-scoped memory, SAKH, source permissions and truthful no-availability rule | CANONICAL_DOC |
| **P47** registry | marketplace/MCP pages public vs unverifiable, 19 source-status rows, no duplicate genesis | CANONICAL_DOC |
| **P56** commerce | feature/trial/paid provider pricing declarations need dated verification, no activation | CANONICAL_DOC |
| **P72** agent trust | GitHub agent owner-grant tool scope, OIDC/sandbox/readback, app-vs-ChatGPT plugin separation | DOC+OFFLINE_CODE |
| **P87** SysML2 compiler | Structura model-first generated code; Apricot capability constraints and tool-list checks | CANONICAL_DOC |
| **P90** SOP | InstructVault source-controlled prompt CI/provenance, no blind install | CANONICAL_DOC |
| **P100** developer IDE | Miro architecture diagrams, AgentHQ tools, Codex meetup research, provider adapters | CANONICAL_DOC |
| **P108** security | SonarQube/SCA/Endor/DAST/Bright tool classes and safe local scope rules | CANONICAL_DOC |
| **P121** infra SRE | LaunchDarkly/Octopus/Packfiles rollout/deploy/migration service boundaries | CANONICAL_DOC |
| **P122 CHEMIA** | independent adult preference intersection, skip/revocation, no unilateral disclosure, two-phone claims flagged | CANONICAL_DOC+OFFLINE_CODE |
| **P50** physical/online game foundry | adult game genre benchmark, safe opt-in mechanic evidence and regulatory review | CANONICAL_DOC |
| **P17/P21** model capability | Gemini Drops time-versioned vendor feature release references, no entitlement assumptions | CANONICAL_DOC |

**New numbered projects:** **0**. P122 is already a separate consent-aware adult pair-game product; marketplace/MCP integration work is covered by P72/P100/P108 and current source/provenance/research owners. No duplicate app business or travel booking project warranted by these URLs.

### Executable material
- `tools/marketplace_agent_review_gate.py` and `tools/test_marketplace_agent_review_gate.py`: 13 stdlib tests.
- `projekty/122-chemia-consent-aware-intimate-two-player-game/consent_intersection.py` and `tools/test_chemia_consent_intersection.py`: 12 stdlib tests.
- Existing GitHub workflow should run both; report observed CI outcome, not hypothetical successes.

**Explicit unimplemented:** third-party app install, MCP OAuth, prompt compilation via InstructVault, Miro diagram integration, adult session server and real age check, online two-phone sync, penetration testing, third-party billing.
