# Wstępny rejestr narzędzi — rozdzielenie istniejącego kodu i OMAt

W `mojealterego/Knowledge-projects/tools/` znajduje się 44 plików, w tym 20 źródłowych skryptów Python nieoznaczonych nazwą `test_*` i 24 plików `test_*.py`. Są to **istniejące ścieżki kodu**, nie dowód działania jako narzędzia OMAt.

| Obserwowany plik | Wstępne zastosowanie według nazwy | Powiązanie z OMAt | Test |
|---|---|---|---|
| [`tools/agent_tool_admission_gate.py`](../tools/agent_tool_admission_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/agent_tool_loop_guard.py`](../tools/agent_tool_loop_guard.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/aura_deck_gate.py`](../tools/aura_deck_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/cloud_operation_budget_gate.py`](../tools/cloud_operation_budget_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/communication_link_policy.py`](../tools/communication_link_policy.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/dgm_cycle_gate.py`](../tools/dgm_cycle_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/external_opportunity_evidence_gate.py`](../tools/external_opportunity_evidence_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/hermes_upstream_adoption_gate.py`](../tools/hermes_upstream_adoption_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/marketplace_agent_review_gate.py`](../tools/marketplace_agent_review_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/mcp_registry_catalog.py`](../tools/mcp_registry_catalog.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/mobile_vault_reader.py`](../tools/mobile_vault_reader.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/multimodal_embedding_intake_gate.py`](../tools/multimodal_embedding_intake_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/osint_provenance_gate.py`](../tools/osint_provenance_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/program_claim_gate.py`](../tools/program_claim_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/project_id_gate.py`](../tools/project_id_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/seeker_evidence_scope_gate.py`](../tools/seeker_evidence_scope_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/single_repository_scope_gate.py`](../tools/single_repository_scope_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/source_bundle_audit.py`](../tools/source_bundle_audit.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/telephony_topology_gate.py`](../tools/telephony_topology_gate.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |
| [`tools/untrusted_agent_source_review.py`](../tools/untrusted_agent_source_review.py) | INFERRED — określić z kodu | CANDIDATE / UNVERIFIED | NOT RUN |

### Odzyskany korpus OMAt

Zgłoszone źródło obejmowało co najmniej 333 numerowane pozycje innej rozmowy Gemini, w tym 330 Dark-Matter. Nie zostało tutaj dostarczone, nie ma więc podstaw do przypisywania numerów, gotowości ani kierunku migracji Python/JSON/Rust. Nie deklarować katalogu ponad 400 działających narzędzi. Zenoh rozważać po określeniu uzasadnienia rozproszonego transportu.

**Dla każdej pozycji po dostarczeniu:** oryginalna nazwa/numer, hash źródła, typ kodu, owner, capabilities, side-effect class, sandbox, timeout, retry policy, Rust port/testy równoważności, integracja OMAs, status.

## Zweryfikowane implementacje źródłowe Python → nowe pakiety Rust

| Źródło | Nowy wpis | Zakres faktycznie przeniesiony | Testy |
|---|---|---|---|
| [`agent_tool_admission_gate.py`](../tools/agent_tool_admission_gate.py) | [006 CUSTOS ADMISSION](OCAF-AGENT-006/docs/README.md) `tool.admit` | SHA-256, allowlista, host-owned approvals, timeout, expiry, non-executing | Python + Rust + local OCAF CI |
| [`agent_tool_loop_guard.py`](../tools/agent_tool_loop_guard.py) | [007 VIGIA LOOP](OCAF-AGENT-007/docs/README.md) `tool.loop_gate` | max calls/tokens, no-progress repeats, sticky halt, host-only progress | Python + Rust + local OCAF CI |

**Uwaga:** To nie oznacza migracji wszystkich narzędzi OMAt. OMAt jest odrębnym serwerem narzędzi i nie istnieje tutaj wdrożona usługa sieciowa; adapter OCAF jest lokalny. Dodatkowe testy Python potwierdzają oryginalne reguły, nie zastępują formalnej równoważności Python/Rust.
