# Instrukcja użytkownika — AXIOM FORGE

**Nazwa własna:** AXIOM FORGE · **Ewidencja:** 004 · **Numer seryjny:** `AM-ENG-AXF-2026-000004` · **Agent ID:** `AM-OCAF-AI-0004` · **Katalog:** `AM-OCAF-ENG-004` · **Nr certyfikatu:** `CERT-AM-2026-0004` (rezerwacja, niewydany).

## Przygotowanie

Repo: `mojealterego/Knowledge-projects`, checkout przypiętego commita. Host Rust stable; nie podawać prawdziwych credentiali, tokenów GitHub, haseł Android i danych klienta. `config/agent.toml` ma zawsze `network_enabled=false` i `source_access.enabled=false`.

## Uruchomienie i test

```sh
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-004/Cargo.toml" -- demo
```

CLI nie wykonuje żadnej zewnętrznej operacji; używa przygotowanego w kodzie demonstracyjnego wejścia. Nie łączy się z urządzeniem Android, klastrem, repozytorium produkcyjnym ani LLM.

## Kontrakt w OCAF

Nazwa: `graph.review`. Przykładowy payload:

```json
{
  "graph_id": "g1",
  "tasks": [
    {
      "task_id": "plan",
      "dependencies": [],
      "stage": "plan"
    },
    {
      "task_id": "build",
      "dependencies": [
        "plan"
      ],
      "stage": "build"
    }
  ]
}
```

Host testowy rejestruje `zenoh::manifest()` i `AgentTool` w lokalnym `OcafRuntime`; nie jest to sesja prawdziwego serwera Zenoh/OMAs/OMAt. Wynik przeznaczony wyłącznie do przeglądu operatora: `GraphReview: status=structurally_valid|blocked, task_count, order[], issues[], execution_authorized=false, verified=false`.

## Obsługa odmów

Błędny serial/nazwa → `InvalidConfiguration`. Nieznane lub niedozwolone wejścia → domenowy błąd walidacji. Sieć i odczyt plików źródłowych → DENY. Samo zadeklarowane zezwolenie nie umożliwia aktywacji ani działania; podpisany niezależny grant nie istnieje.

## Gdy powstaje incydent

Zanotować SHA i job CI, zachować logi bez sekretów; nie włączać publicznego transportu; nie zmieniać numeru seryjnego w produkcie już wydanym. Szczegóły: `SERVICE.md`, `RELEASE_CHECKLIST.md`, `SECURITY.md`.
