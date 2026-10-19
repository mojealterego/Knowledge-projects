# EVIDENTIA RESEARCH — OCAF-AGENT-003

**Nazwa własna:** EVIDENTIA RESEARCH · **Numer ewidencyjny:** 003 · **Numer seryjny:** `AM-RES-EVI-2026-000003` · **Agent ID:** `AM-OCAF-AI-0003` · **Nr katalogowy:** `AM-OCAF-RES-003` · **Certyfikat:** `CERT-AM-2026-0003` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: lokalna kontrola jakości przesłanek badawczych. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Instalacja i test

Z checkoutu `Knowledge-projects` uruchom:

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-003/Cargo.toml" -- "Przykładowa teza"
```

`main.rs` demonstruje **jawnie sztuczne** dane wejściowe, nie połączenie z dostawcą ani rzeczywistą infrastrukturą. Użytkownik nie musi dostarczać kluczy API; nie przekazuj kluczy do tego procesu.

## Wywołanie testowe OCAF

Nazwa narzędzia: `research.assess`, payload:

```json
{
  "claim": "Przykładowa teza",
  "evidence": [
    {
      "source_id": "A",
      "stance": "supports",
      "excerpt": "Rzekomy argument"
    },
    {
      "source_id": "B",
      "stance": "contradicts",
      "excerpt": "Rzekomy kontrargument"
    }
  ]
}
```

OCAF lokalnie waliduje JSON i przekazuje go do deterministycznej funkcji domenowej. Błędne lub nieznane pola powodują odmowę. Opis źródła nie staje się faktograficznie prawdziwy poprzez przekazanie go modelowi ani lokalną analizę; nawet sprzeczność wymaga niezależnej adjudykacji.

## Błędy i incydenty

Błędna nazwa/serial/TOML → `InvalidConfiguration`, niepoprawne dane → `InvalidClaim/InvalidEvidence/DuplicateSource`. Próba odczytu plików źródłowych → `SourceEncryptionNotImplemented`. Wymuszenie ACTIVE → `DeploymentNotAuthorized`. Zachowaj jedynie niezbędne logi bez osobowych danych i sekretów; brak serwera do restartu.

## Ostrzeżenia

Agent **nie wykonuje poleceń i nie pozyskuje źródeł**, nie przesyła danych przez Zenoh, nie integruje się sieciowo z OMAs/OMAt. Zobacz `RELEASE_CHECKLIST.md` przed jakimkolwiek wdrożeniem.
