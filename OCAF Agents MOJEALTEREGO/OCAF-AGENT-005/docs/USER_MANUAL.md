# Instrukcja użytkownika — ARGUS MOBILE

**Nazwa własna:** ARGUS MOBILE · **Ewidencja:** 005 · **Numer seryjny:** `AM-MOB-ARG-2026-000005` · **Agent ID:** `AM-OCAF-AI-0005` · **Katalog:** `AM-OCAF-MOB-005` · **Nr certyfikatu:** `CERT-AM-2026-0005` (rezerwacja, niewydany).

## Przygotowanie

Repo: `mojealterego/Knowledge-projects`, checkout przypiętego commita. Host Rust stable; nie podawać prawdziwych credentiali, tokenów GitHub, haseł Android i danych klienta. `config/agent.toml` ma zawsze `network_enabled=false` i `source_access.enabled=false`.

## Uruchomienie i test

```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-005/Cargo.toml" -- demo
```

CLI nie wykonuje żadnej zewnętrznej operacji; używa przygotowanego w kodzie demonstracyjnego wejścia. Nie łączy się z urządzeniem Android, klastrem, repozytorium produkcyjnym ani LLM.

## Kontrakt w OCAF

Nazwa: `ui.proposal_assess`. Przykładowy payload:

```json
{
  "session_id": "s1",
  "snapshot_id": "snap1",
  "node_id": "node1",
  "action": "CLICK",
  "operator_grant": true,
  "expected_postcondition": "new view"
}
```

Host testowy rejestruje `zenoh::manifest()` i `AgentTool` w lokalnym `OcafRuntime`; nie jest to sesja prawdziwego serwera Zenoh/OMAs/OMAt. Wynik przeznaczony wyłącznie do przeglądu operatora: `ActionAssessment: status=blocked|requires_grant|operator_review, execution_authorized=false, readback_required=true, verified=false, reasons[]`.

## Obsługa odmów

Błędny serial/nazwa → `InvalidConfiguration`. Nieznane lub niedozwolone wejścia → domenowy błąd walidacji. Sieć i odczyt plików źródłowych → DENY. Samo zadeklarowane zezwolenie nie umożliwia aktywacji ani działania; podpisany niezależny grant nie istnieje.

## Gdy powstaje incydent

Zanotować SHA i job CI, zachować logi bez sekretów; nie włączać publicznego transportu; nie zmieniać numeru seryjnego w produkcie już wydanym. Szczegóły: `SERVICE.md`, `RELEASE_CHECKLIST.md`, `SECURITY.md`.
